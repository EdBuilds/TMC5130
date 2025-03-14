#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod mock_peripherals;
#[macro_use]
extern crate bitfield;
pub mod reg;


use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};
use core::borrow::BorrowMut;
use crate::reg::{Address, SPISTATUS, State};

/// Driver for the Tmc5130 4-wire touch screen controller.
pub struct Tmc5130<SPI> {
    /// The SPI interface used to communicate with the Tmc5130 chip.
    spi: SPI,
}
pub enum Action<'a> {
    read(&'a mut State),
    write(&'a State)
}
impl<SPI> Tmc5130<SPI>
where
    SPI: SpiDevice<u8>,
{
    const RW_BIT: u8 = 0b1000_0000;
    /// Creates a new instance of the `Tmc5130` driver.
    ///
    /// # Arguments
    ///
    /// * `spi` - The SPI interface used to communicate with the Tmc5130 chip.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Tmc5130` instance or an error if the register update fails.
    pub fn new(
        spi: SPI,
    ) -> Self{
        let mut instance = Self {
            spi,
        };
        instance
    }
    fn as_u32_be(array: &[u8]) -> u32 {
        ((array[0] as u32) << 24) +
            ((array[1] as u32) << 16) +
            ((array[2] as u32) << 8) +
            ((array[3] as u32) << 0)
    }
    pub fn read_register<R>(&mut self) -> Result<(reg::SPISTATUS, R), <SPI as ErrorType>::Error>
    where R: reg::ReadableRegister
    {
        let address = R::ADDRESS as u8 & !Self::RW_BIT;

        let mut address_buffer = [address;1];
        let mut data_buffer = [0u8;4];
        self.spi.transaction(&mut [Operation::TransferInPlace(address_buffer.borrow_mut()), Operation::TransferInPlace(data_buffer.borrow_mut())])?;
        let mut address_buffer = [address;1];
        self.spi.transaction(&mut [Operation::TransferInPlace(address_buffer.borrow_mut()), Operation::TransferInPlace(data_buffer.borrow_mut())])?;
        let status = reg::SPISTATUS(address_buffer[0]);

        Ok((status,R::from(u32::from_be_bytes(data_buffer))))
    }
    pub fn bulk_register_action(&mut self, actions: &mut [Action]) -> Result<reg::SPISTATUS, <SPI as ErrorType>::Error>
    {
        let mut result = reg::SPISTATUS(0);
        let act_len = actions.len();
        let mut extra_transmission = false;
        if let Some(Action::read(_)) = actions.last() {
            extra_transmission =true;
        }

        for i in 0..act_len {
            let (mut address_buf,mut data_buf)  = match actions[i].borrow_mut() {
                Action::read(state) => {
                    //defmt::info!("Read address: {}", (state.reg()::ADDRESS) as u8);
                    //defmt::info!("Read state address: {:x}", state.addr() as u8);
                    //defmt::info!("buf: {:x}", state.addr() as u8 | Self::RW_BIT);
                    let state_num: u32 = (*(*state)).into();
                    ([state.addr() as u8 & ! Self::RW_BIT;1], state_num.to_be_bytes())
                }
                Action::write(state) => {
                    let state_num: u32 = (*(*state)).into();
                    ([state.addr() as u8 | Self::RW_BIT;1], state_num.to_be_bytes())
                }
            };
            result = reg::SPISTATUS(address_buf[0]);
            self.spi.transaction(&mut [Operation::TransferInPlace(address_buf.borrow_mut()), Operation::TransferInPlace(data_buf.borrow_mut())])?;
            if i > 0 {
                if let Action::read(last_state) = &mut actions[i-1] {
                    **last_state= reg::State::from_addr_and_data(last_state.addr(), u32::from_be_bytes(data_buf));
                }
            }
        }
        if extra_transmission {
            // Last transaction was a read. we need another read to get the data out.
            if let Action::read(last_state) = &mut actions[act_len-1] {
                let state_num: u32 = (*(*last_state)).into();
                let (mut address_buf, mut data_buf) = ([last_state.addr() as u8 | Self::RW_BIT; 1], state_num.to_be_bytes());
                self.spi.transaction(&mut [Operation::TransferInPlace(address_buf.borrow_mut()), Operation::TransferInPlace(data_buf.borrow_mut())])?;
                **last_state = State::from_addr_and_data(last_state.addr(), u32::from_be_bytes(data_buf));
                result = reg::SPISTATUS(address_buf[0]);
            }
        }
        Ok(result)
    }
    pub fn write_register<R>(&mut self, register:R) -> Result<reg::SPISTATUS, <SPI as ErrorType>::Error>
        where R: reg::WritableRegister
    {
        let address = R::ADDRESS as u8 | Self::RW_BIT;
        let mut address_buffer = [address;1];
        let mut data_buffer = (<R as Into<u32>>::into(register)).to_be_bytes();
        self.spi.transaction(&mut [Operation::TransferInPlace(address_buffer.borrow_mut()), Operation::TransferInPlace(data_buffer.borrow_mut())])?;
        Ok(reg::SPISTATUS(address_buffer[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_peripherals::{MockOperation, MockSimpleHalSpiDevice};

    // Predefined control words for testing
    const CTRL_WORD_X_NO_IRQ: u8 = 0b11010011;
    const CTRL_WORD_Y_NO_IRQ: u8 = 0b10010011;
    const CTRL_WORD_Z1_NO_IRQ: u8 = 0b10110011;
    const CTRL_WORD_Z2_NO_IRQ: u8 = 0b11000011;

    const CTRL_WORD_X_IRQ: u8 = 0b11010000;
    const CTRL_WORD_Y_IRQ: u8 = 0b10010000;
    const CTRL_WORD_Z1_IRQ: u8 = 0b10110000;
    const CTRL_WORD_Z2_IRQ: u8 = 0b11000000;

    //Helper function to assert SPI operations
    fn assert_spi_operations<Word: std::cmp::PartialEq + std::fmt::Debug + std::marker::Copy>(
        ops: &mut [Operation<'_, Word>],
        expected_ops: &[MockOperation<'_, Word>],
    ) {
        assert_eq!(ops.len(), expected_ops.len());
        let both_ops = ops.iter_mut().zip(expected_ops.iter());
        for (op, expected_op) in both_ops {
            match expected_op {
                MockOperation::Read(expected_read_buf) => {
                    match op {
                        // In case of a reading operation, we copy the expected values into the buffer.
                        Operation::Read(op_read_buf) => {
                            assert_eq!(op_read_buf.len(), expected_read_buf.len());
                            for i in 0..op_read_buf.len() {
                                op_read_buf[i] = expected_read_buf[i];
                            }
                        }
                        _ => assert!(false),
                    }
                }
                MockOperation::Write(expected_write_buf) => {
                    match op {
                        // In case of a Writing operation, the value of the buffer is compared against the value of the expected buffer.
                        Operation::Write(op_write_buf) => {
                            assert_eq!(op_write_buf, expected_write_buf);
                        }
                        _ => assert!(false),
                    }
                }
            }
        }
    }

    static X_TOUCH_VALUE: u16 = 100;
    static Y_TOUCH_VALUE: u16 = 100;
    static Z1_TOUCH_VALUE: u16 = 5;
    static Z2_TOUCH_VALUE: u16 = 2053;

    static INIT_RETURN_BUF: [u8; 2] = [0, 0];
    static READ_X_RETURN_BUF: [u8; 2] = [(X_TOUCH_VALUE >> 5) as u8, (X_TOUCH_VALUE << 3) as u8];
    static READ_Y_RETURN_BUF: [u8; 2] = [(Y_TOUCH_VALUE >> 5) as u8, (Y_TOUCH_VALUE << 3) as u8];
    static READ_Z1_RETURN_BUF: [u8; 2] = [(Z1_TOUCH_VALUE >> 5) as u8, (Z1_TOUCH_VALUE << 3) as u8];
    static READ_Z2_RETURN_BUF: [u8; 2] = [(Z2_TOUCH_VALUE >> 5) as u8, (Z2_TOUCH_VALUE << 3) as u8];
    #[test]
    fn test_get_touch_no_irq() {
        let expected_ops_init = [
            MockOperation::Write(&[CTRL_WORD_X_NO_IRQ]),
            MockOperation::Read(&INIT_RETURN_BUF),
        ];
        let expected_ops_x = [
            MockOperation::Write(&[CTRL_WORD_X_NO_IRQ]),
            MockOperation::Read(&READ_X_RETURN_BUF),
        ];
        let expected_ops_y = [
            MockOperation::Write(&[CTRL_WORD_Y_NO_IRQ]),
            MockOperation::Read(&READ_Y_RETURN_BUF),
        ];
        let expected_ops_z1 = [
            MockOperation::Write(&[CTRL_WORD_Z1_NO_IRQ]),
            MockOperation::Read(&READ_Z1_RETURN_BUF),
        ];
        let expected_ops_z2 = [
            MockOperation::Write(&[CTRL_WORD_Z2_NO_IRQ]),
            MockOperation::Read(&READ_Z2_RETURN_BUF),
        ];
        let mut mock_spi_dev = MockSimpleHalSpiDevice::new();

        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_init);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_x);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_y);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_z1);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_z2);
                Ok(())
            });
        let mut test_driver =
            Tmc5130::new(mock_spi_dev).expect("Could not create driver");
        let mut chopconf = reg::CHOPCONF::default();
        chopconf.set_toff(3);
        chopconf.set_hstrt(4);
        chopconf.set_hend(1);
        chopconf.set_tbl(2);
        chopconf.set_chm(false);
        test_driver.write_register(chopconf).unwrap();

    }

    #[test]
    fn test_get_touch_irq() {
        let expected_ops_init = [
            MockOperation::Write(&[CTRL_WORD_X_NO_IRQ]),
            MockOperation::Read(&INIT_RETURN_BUF),
        ];
        let expected_ops_irq_set = [
            MockOperation::Write(&[CTRL_WORD_X_IRQ]),
            MockOperation::Read(&INIT_RETURN_BUF),
        ];

        let expected_ops_x = [
            MockOperation::Write(&[CTRL_WORD_X_IRQ]),
            MockOperation::Read(&READ_X_RETURN_BUF),
        ];
        let expected_ops_y = [
            MockOperation::Write(&[CTRL_WORD_Y_IRQ]),
            MockOperation::Read(&READ_Y_RETURN_BUF),
        ];
        let expected_ops_z1 = [
            MockOperation::Write(&[CTRL_WORD_Z1_IRQ]),
            MockOperation::Read(&READ_Z1_RETURN_BUF),
        ];
        let expected_ops_z2 = [
            MockOperation::Write(&[CTRL_WORD_Z2_IRQ]),
            MockOperation::Read(&READ_Z2_RETURN_BUF),
        ];
        let mut mock_spi_dev = MockSimpleHalSpiDevice::new();

        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_init);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_irq_set);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_x);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_y);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_z1);
                Ok(())
            });
        mock_spi_dev
            .expect_transaction()
            .times(1)
            .returning(move |operations| {
                assert_spi_operations(operations, &expected_ops_z2);
                Ok(())
            });
        let mut test_driver =
            Tmc5130::new(mock_spi_dev).expect("Could not create driver");
    }
}
