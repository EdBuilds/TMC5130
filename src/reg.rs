//! Declaration of the TMC2209 registers and their implementations.
//!
//! Please refer to the TMC2209 datasheet for information on what each of these registers and their
//! fields mean. The register map is described under section 5 of the datasheet.
//!
//! https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC2209_Datasheet_V103.pdf

#![allow(non_camel_case_types)]

// Register Traits
// --------------------------------------------------------

/// Implemented for all register types.
///
/// NOTE: This should not be implemented for custom types. If the user attempts to request a custom
/// register type from the register `Map`, the method call will hang indefinitely.
pub trait Register: Into<State> {
    const ADDRESS: Address;
}

/// Implemented for all registers that can be read from.
pub trait ReadableRegister: Register + From<u32> {}

/// Implemented for all registers that can be written to.
pub trait WritableRegister: Register + Into<u32> {}

/// An error that might occur in the case that an address could not be parsed.
#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct UnknownAddress;

/// An error indicating an unexpected `State`.
#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct UnexpectedAddress;

// Register Declarations
// --------------------------------------------------------

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct GCONF(u32);
    impl Debug;
    u16;
    pub i_scale_analog, set_i_scale_analog: 0;
    pub internal_rsense, set_internal_rsense: 1;
    pub en_spread_cycle, set_en_spread_cycle: 2;
    pub enc_commutation, set_enc_commutation: 3;
    pub shaft, set_shaft: 4;
    pub diag0_error, set_diag0_error: 5;
    pub diag0_otpw, set_diag0_otpw: 6;
    pub diag0_stall_step, set_diag0_stall_step: 7;
    pub diag1_stall_dir, set_diag1_stall_dir: 8;
    pub diag1_index, set_diag1_index: 9;
    pub diag1_onstate, set_diag1_onstate: 10;
    pub diag1_steps_skipped, set_diag1_steps_skipped: 11;
    pub diag0_int_pushpull, set_diag0_int_pushpull: 12;
    pub diag1_poscomp_pushpull, set_diag1_poscomp_pushpull: 13;
    pub small_hysteresis, set_small_hysteresis: 14;
    pub stop_enable, set_stop_enable: 15;
    pub direct_mode, set_direct_mode: 16;
    pub test_mode, set_test_mode: 17;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct GSTAT(u32);
    impl Debug;
    u8;
    pub reset, _: 0;
    pub drv_err, _: 1;
    pub uv_cp, _: 2;
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct IFCNT(pub u32);

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct SLAVECONF(u32);
    impl Debug;
    u8;
    pub slaveaddr, set_slaveaddr: 7, 0;
    pub senddelay, set_senddelay: 11, 8;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct IOIN(u32);
    impl Debug;
    u16;
    pub refl_step, _: 0;
    pub refr_dir, _: 1;
    pub encb_dcen_cfg4, _: 2;
    pub enca_dcen_cfg5, _: 3;
    pub drv_enn_cfg6, _: 4;
    pub enc_n_dco, _: 5;
    pub sd_mode, _: 6;
    pub swcomp_in, _: 7;
    pub version, _: 31, 24;
}




bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct OUTPUT(u32);
    impl Debug;
    u32;
    pub get, set: 0;
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct X_COMPARE(pub u32);

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct IHOLD_IRUN(u32);
    impl Debug;
    u8;
    pub ihold, set_ihold: 4, 0;
    pub irun, set_irun: 12, 8;
    pub ihold_delay, set_ihold_delay: 19, 16;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct TPOWERDOWN(u32);
    impl Debug;
    i32;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct TSTEP(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct TPWMTHRS(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct TCOOLTHRS(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct THIGH(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct RAMPMODE(u32);
    impl Debug;
    u8;
    pub get, set: 1, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct XACTUAL(u32);
    impl Debug;
    i32;
    pub get, _: 31, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct VACTUAL(u32);
    impl Debug;
    i32;
    pub get, _: 23, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct VSTART(u32);
    impl Debug;
    u32;
    pub get, set: 17, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct A1(u32);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct V1(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct AMAX(u32);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct VMAX(u32);
    impl Debug;
    u32;
    pub get, set: 22, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct DMAX(u32);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct D1(u32);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct VSTOP(u32);
    impl Debug;
    u32;
    pub get, set: 17, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct TZEROWAIT(u32);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct XTARGET(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct VDCMIN(u32);
    impl Debug;
    u32;
    pub get, set: 22, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct SW_MODE(u32);
    impl Debug;
    u16;
    pub stop_l_enable, set_stop_l_enable: 0;
    pub stop_r_enable, set_stop_r_enable: 1;
    pub pol_stop_l, set_pol_stop_l: 2;
    pub pol_stop_r, set_pol_stop_r: 3;
    pub swap_lr, set_swap_lr: 4;
    pub latch_l_active, set_latch_l_active: 5;
    pub latch_l_inactive, set_latch_l_inactive: 6;
    pub latch_r_active, set_latch_r_active: 7;
    pub latch_r_inactive, set_latch_r_inactive: 8;
    pub en_latch_encoder, set_en_latch_encoder: 9;
    pub sg_stop, set_sg_stop: 10;
    pub en_softstop, set_en_softstop: 11;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct RAMP_STAT(u32);
    impl Debug;
    u16;
    pub status_stop_l, _: 0;
    pub status_stop_r, _: 1;
    pub status_latch_l, set_status_latch_l: 2;
    pub status_latch_r, set_status_latch_r: 3;
    pub event_stop_l, _: 4;
    pub event_stop_r, _: 5;
    pub event_stop_sg, set_event_stop_sg: 6;
    pub event_pos_reached, set_event_pos_reached: 7;
    pub velocity_reached, _: 8;
    pub position_reached, _: 9;
    pub vzero, _: 10;
    pub t_zerowait_active, _: 11;
    pub second_move, set_second_move: 12;
    pub status_sg, _: 13;
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct XLATCH(pub u32);

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT0(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT1(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT2(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT3(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT4(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT5(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT6(pub u32);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct MSLUT7(pub u32);

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct MSLUTSEL(u32);
    impl Debug;
    u16;
    pub w0, set_w0: 1, 0;
    pub w1, set_w1: 3, 2;
    pub w2, set_w2: 5, 4;
    pub w3, set_w3: 7, 6;
    pub x1, set_x1: 15, 8;
    pub x2, set_x2: 23, 16;
    pub x3, set_x3: 31, 24;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct MSLUTSTART(u32);
    impl Debug;
    u32;
    pub start_sin, set_start_sin: 7, 0;
    pub start_sin90, set_start_sin90: 23, 16;
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct SGTHRS(pub u32);

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct SG_RESULT(u32);
    impl Debug;
    u16;
    pub get, _: 9, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct COOLCONF(u32);
    impl Debug;
    u16;
    pub semin, set_semin: 3, 0;
    pub seup, set_seup: 6, 5;
    pub semax, set_semax: 11, 8;
    pub sedn, set_sedn: 14, 13;
    pub seimin, set_seimin: 15;
    i8;
    pub sgt, set_sgt: 22,16;
    u16;
    pub sfilt, set_sfilt: 24;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct DCCTRL(u32);
    impl Debug;
    u16;
    pub dc_time, set_dc_time: 9, 0;
    pub dc_sg, set_dc_sg: 23, 16;
}
bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct MSCNT(u32);
    impl Debug;
    u16;
    pub get, _: 9, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct MSCURACT(u32);
    impl Debug;
    u16;
    pub cur_a, _: 8, 0;
    pub cur_b, _: 24, 16;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct CHOPCONF(u32);
    impl Debug;
    u32;
    pub toff, set_toff: 3, 0;
    pub hstrt, set_hstrt: 6, 4;
    pub hend, set_hend: 10, 7;
    pub fd3, set_fd3:  11;
    pub disfdcc, set_disfdcc: 12;
    pub rndtf, set_rndtf: 13;
    pub chm, set_chm: 14;
    pub tbl, set_tbl: 16, 15;
    pub vsense, set_vsense: 17;
    pub vhighfs, set_vhighfs: 18;
    pub vhighchm, set_vhighchm: 19;
    pub sync, set_sync: 23, 20;
    pub mres, set_mres: 27, 24;
    pub intpol, set_intpol: 28;
    pub dedge, set_dedge: 29;
    pub diss2g, set_diss2g: 30;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct DRV_STATUS(u32);
    impl Debug;
    u32;
    pub sg_result, _: 9,0;
    pub fsactive, _: 15;
    pub cs_actual, _: 20, 16;
    pub stallguard, _: 24;
    pub ot, _: 25;
    pub otpw, _: 26;
    pub s2ga, _: 27;
    pub s2gb, _: 28;
    pub ola, _: 29;
    pub olb, _: 30;
    pub stst, _: 31;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct PWMCONF(u32);
    impl Debug;
    u8;
    pub pwm_ampl, set_pwm_ampl: 7, 0;
    pub pwm_grad, set_pwm_grad: 15, 8;
    pub pwm_freq, set_pwm_freq: 17, 16;
    pub pwm_autoscale, set_pwm_autoscale: 18;
    pub pwm_symmetric, set_pwm_symmetric: 19;
    pub freewheel, set_freewheel: 21, 20;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct PWM_SCALE(u32);
    impl Debug;
    u8;
    pub get, _: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct ENCM_CTRL(u32);
    impl Debug;
    u8;
    pub inv, set_inv: 0;
    pub maxspeed, set_maxspeed: 1;
}

bitfield! {
    #[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "hash", derive(hash32_derive::Hash32))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    pub struct LOST_STEPS(u32);
    impl Debug;
    u32;
    pub get, _: 19, 0;
}
// Implementation Macros
// --------------------------------------------------------

/// A macro for generating `ReadableRegister` and `WritableRegister` implementations for the
/// register types based on the `R`, `W` or `RW` prefix.
macro_rules! impl_rw {
    (RW $T:ident) => {
        impl ReadableRegister for $T {}
        impl WritableRegister for $T {}
    };
    (R $T:ident) => {
        impl ReadableRegister for $T {}
    };
    (W $T:ident) => {
        impl WritableRegister for $T {}
    };
}

macro_rules! is_readable {
    (RW) => {
        true
    };
    (R) => {
        true
    };
    (W) => {
        false
    };
}

macro_rules! is_writable {
    (RW) => {
        true
    };
    (R) => {
        false
    };
    (W) => {
        true
    };
}

macro_rules! map_indices {
    ($ix:expr, $T:ident) => {
        pub(crate) const $T: usize = $ix;
    };
    ($ix:expr, $T:ident, $($Ts:ident),*) => {
        pub(crate) const $T: usize = $ix;
        map_indices!($T + 1, $($Ts),*);
    };
}

/// A macro for generating the `Address` enum along with the `Register` trait implementations.
macro_rules! impl_registers {
    ($($RW:ident $addr:literal $T:ident $map_access:ident $map_access_mut:ident,)*) => {
        /// Generate a private, unique index for each register into the `Map`'s inner array.
        mod map_index {
            map_indices!(0, $($T),*);
        }

        /// A dynamic representation of a register's 8-bit address.
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
        pub enum Address {
            $(
                $T = $addr,
            )*
        }

        /// A dynamic representation of a register's 32-bit state.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
        pub enum State {
            $(
                $T($T),
            )*
        }

        /// A map of the state of all registers in the TMC2209.
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
        pub struct Map {
            arr: MapArray,
        }

        /// The inner array storing all register state.
        ///
        /// Each register is laid out in the array in the order in which they are declared in the
        /// `impl_registers` macro. The `map_index` module is used internally to map register
        /// addresses and their state to the associated elements in the array.
        type MapArray = [State; COUNT];

        /// The total number of documented registers in the TMC2209.
        ///
        /// Useful for statically allocated register maps, etc.
        pub const COUNT: usize = 0 $(+ { let _ = Address::$T; 1 })*;

        impl Map {
            /// The total number of documented registers in the TMC2209.
            pub const LEN: usize = COUNT;

            /// Read-only access to the register of the given type.
            pub fn reg<T>(&self) -> &T
            where
                T: 'static + Register,
            {
                self.state(T::ADDRESS)
                    .reg::<T>()
                    // We gaurantee that `TmcRegisters` will always have state for each register, but need
                    // to avoid generating panicking branches, so we use an infinite loop rather than
                    // unwrap.
                    .unwrap_or_else(|_| loop {})
            }

            /// Mutable access to the register of the given type.
            pub fn reg_mut<T>(&mut self) -> &mut T
            where
                T: 'static + Register,
            {
                self.state_mut(T::ADDRESS)
                    .reg_mut::<T>()
                    // We gaurantee that `TmcRegisters` will always have state for each register, but need
                    // to avoid generating panicking branches, so we use an infinite loop rather than
                    // unwrap.
                    .unwrap_or_else(|_| loop {})
            }

            /// Read-only access to the dynamic representation of the register state at the given
            /// address.
            pub fn state(&self, addr: Address) -> &State {
                match addr {
                    $(
                        // We gaurantee that `Map` will always have state for each register.
                        Address::$T => unsafe {
                            self.arr.get_unchecked(map_index::$T)
                        }
                    )*
                }
            }

            /// Mutable access to the dynamic representation of the register state at the given
            /// address.
            ///
            /// Note: This should remain private for internal use only, as the user should never be
            /// allowed to change the stored `State` to a different variant.
            fn state_mut(&mut self, addr: Address) -> &mut State {
                match addr {
                    $(
                        // We gaurantee that `Map` will always have state for each register.
                        Address::$T => unsafe {
                            self.arr.get_unchecked_mut(map_index::$T)
                        }
                    )*
                }
            }

            /// Update the given register state.
            pub fn set_state(&mut self, state: State) {
                *self.state_mut(state.addr()) = state;
            }

            // Generate the short-hand names for gaining direct access to typed register state.
            $(
                pub fn $map_access(&self) -> &$T {
                    self.reg::<$T>()
                }

                pub fn $map_access_mut(&mut self) -> &mut $T {
                    self.reg_mut::<$T>()
                }
            )*
        }

        impl Address {
            /// All register addresses.
            pub const ALL: &'static [Self] = &[
                $(
                    Self::$T,
                )*
            ];

            /// Whether or not we can send a read request to the register address.
            pub fn readable(&self) -> bool {
                match *self {
                    $(
                        Self::$T => is_readable!($RW),
                    )*
                }
            }

            /// Whether or not we can send a write request to the register address.
            pub fn writable(&self) -> bool {
                match *self {
                    $(
                        Self::$T => is_writable!($RW),
                    )*
                }
            }
        }

        impl State {
            /// Construct a register state from its address and data represented as a `u32`.
            pub fn from_addr_and_data(addr: Address, data: u32) -> Self {
                match addr {
                    $(
                        Address::$T => State::$T(<_>::from(data)),
                    )*
                }
            }

            /// Construct the default register state associated with the given address.
            pub fn from_addr_default(addr: Address) -> Self {
                match addr {
                    $(
                        Address::$T => State::$T(<_>::default()),
                    )*
                }
            }

            /// The address of the register with which this state is associated.
            pub fn addr(&self) -> Address {
                match *self {
                    $(
                        State::$T(_) => Address::$T,
                    )*
                }
            }

            /// Attempt to retrieve a reference to a register of type `R` from the dynamic register
            /// `State` representation.
            ///
            /// Returns an `Err` if the register type does not match.
            pub fn reg<R>(&self) -> Result<&R, UnexpectedAddress>
            where
                R: 'static + Register,
            {
                match *self {
                    $(
                        Self::$T(ref r) => (r as &dyn core::any::Any)
                            .downcast_ref()
                            .ok_or(UnexpectedAddress),
                    )*
                }
            }

            /// Attempt to retrieve a mutable reference to a register of type `R` from the dynamic
            /// register `State` representation.
            ///
            /// Returns an `Err` if the register type does not match.
            pub fn reg_mut<R>(&mut self) -> Result<&mut R, UnexpectedAddress>
            where
                R: 'static + Register,
            {
                match *self {
                    $(
                        Self::$T(ref mut r) => (r as &mut dyn core::any::Any)
                            .downcast_mut()
                            .ok_or(UnexpectedAddress),
                    )*
                }
            }
        }

        impl Default for Map {
            fn default() -> Self {
                let arr = [$(
                    State::$T($T::default()),
                )*];
                Map { arr }
            }
        }

        impl core::ops::Deref for Map {
            type Target = MapArray;
            fn deref(&self) -> &Self::Target {
                &self.arr
            }
        }

        #[cfg(feature = "hash")]
        impl hash32::Hash for Address {
            fn hash<H>(&self, state: &mut H)
            where
                H: hash32::Hasher,
            {
                (*self as u8).hash(state)
            }
        }

        #[cfg(feature = "hash")]
        impl hash32::Hash for State {
            fn hash<H>(&self, state: &mut H)
            where
                H: hash32::Hasher,
            {
                let u: u32 = (*self).into();
                u.hash(state)
            }
        }

        impl core::ops::Index<Address> for Map {
            type Output = State;
            fn index(&self, addr: Address) -> &Self::Output {
                self.state(addr)
            }
        }

        impl core::ops::IndexMut<Address> for Map {
            fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
                self.state_mut(addr)
            }
        }

        impl Into<u8> for Address {
            fn into(self) -> u8 {
                self as u8
            }
        }

        impl Into<u32> for State {
            fn into(self) -> u32 {
                match self {
                    $(
                        State::$T(r) => r.into(),
                    )*
                }
            }
        }

        impl core::convert::TryFrom<u8> for Address {
            type Error = UnknownAddress;
            fn try_from(u: u8) -> Result<Self, Self::Error> {
                let reg = match u {
                    $(
                        $addr => Self::$T,
                    )*
                    _ => return Err(UnknownAddress),
                };
                Ok(reg)
            }
        }

        $(
            impl From<u32> for $T {
                fn from(u: u32) -> $T {
                    $T(u)
                }
            }

            impl From<$T> for State {
                fn from(r: $T) -> Self {
                    State::$T(r)
                }
            }

            impl Into<u32> for $T {
                fn into(self) -> u32 {
                    self.0 as u32
                }
            }

            impl Register for $T {
                const ADDRESS: Address = Address::$T;
            }

            impl core::convert::TryFrom<State> for $T {
                type Error = UnexpectedAddress;
                fn try_from(state: State) -> Result<Self, Self::Error> {
                    match state {
                        State::$T(s) => Ok(s),
                        _ => Err(UnexpectedAddress),
                    }
                }
            }
        )*

        $(
            impl_rw!{$RW $T}
        )*
    };
}

// Register Implementations
// --------------------------------------------------------

impl_registers! {
    // General Registers.
    RW 0x00 GCONF gconf gconf_mut,
    RW 0x01 GSTAT gstat gstat_mut,
    R  0x02 IFCNT ifcnt ifcnt_mut,
    W  0x03 SLAVECONF slaveconf slaveconf_mut,
    R  0x04 IOIN ioin ioin_mut,
    R  0x05 X_COMPARE x_compare x_compare_mut,
    W  0x10 IHOLD_IRUN ihold_irun ihold_irun_mut,
    W  0x11 TPOWERDOWN tpowerdown tpowerdown_mut,
    W  0x12 TSTEP tstep tstep_mut,
    W  0x13 TPWMTHRS tpwmthrs tpwmthrs_mut,
    W  0x14 TCOOLTHRS tcoolthrs tcoolthrs_mut,
    W  0x15 THIGH thigh thigh_mut,
    // Ramp generation
    RW 0x20 RAMPMODE rampmode rampmode_mut,
    RW 0x21 XACTUAL xactual xactual_mut,
    R 0x22 VACTUAL vactual vactual_mut,
    W 0x23 VSTART vstart vstart_mut,
    W 0x24 A1 a1 a1_mut,
    W 0x25 V1 v1 v1_mut,
    W 0x26 AMAX amax amax_mut,
    W 0x27 VMAX vmax vmax_mut,
    W 0x28 DMAX dmax dmax_mut,
    W 0x2A D1 d1 d1_mut,
    W 0x2B VSTOP vstop vstop_mut,
    W 0x2C TZEROWAIT tzerowait tzerowait_mut,
    RW 0x2D XTARGET xtarget xtarget_mut,
    W 0x33 VDCMIN vdcmin vdcmin_mut,
    RW 0x34 SW_MODE sw_mode sw_mode_mut,
    RW 0x35 RAMP_STAT ramp_stat ramp_stat_mut,
    R 0x36 XLATCH xlatch xlatch_mut,
    //encoder registers missing
    W 0x60 MSLUT0 mslut0 mslut0_mut,
    W 0x61 MSLUT1 mslut1 mslut1_mut,
    W 0x62 MSLUT2 mslut2 mslut2_mut,
    W 0x63 MSLUT3 mslut3 mslut3_mut,
    W 0x64 MSLUT4 mslut4 mslut4_mut,
    W 0x65 MSLUT5 mslut5 mslut5_mut,
    W 0x66 MSLUT6 mslut6 mslut6_mut,
    W 0x67 MSLUT7 mslut7 mslut7_mut,
    W 0x68 MSLUTSEL mslutsel mslutsel_mut,
    W 0x69 MSLUTSTART mslutstart mslutstart_mut,
    R 0x6A MSCNT mscnt mscnt_mut,
    R 0x6B MSCURACT mscuract mscuract_mut,
    RW 0x6C CHOPCONF chopconf chopconf_mut,
    W 0x6D COOLCONF coolconf coolconf_mut,
    W 0x6E DCCTRL dcctrl dcctrl_mut,
    R 0x6F DRV_STATUS drv_status drv_status_mut,
    W 0x70 PWMCONF pwmconf pwmconf_mut,
    R 0x71 PWM_SCALE pwm_scale pwm_scale_mut,
    W 0x72 ENCM_CTRL encm_ctrl encm_ctrl_mut,
    R 0x73 LOST_STEPS lost_steps lost_steps_mut,
}

impl VACTUAL {
    /// Creates the `VACTUAL` register enabled for UART control but in a stopped state.
    pub const ENABLED_STOPPED: Self = VACTUAL(1);
}

// Default Register States (taken from TMC-API reference).
// --------------------------------------------------------

impl Default for GCONF {
    fn default() -> Self {
        Self(0x00000041)
    }
}

impl Default for IHOLD_IRUN {
    fn default() -> Self {
        Self(0x00001F00)
    }
}

impl Default for CHOPCONF {
    fn default() -> Self {
        Self(0x00000000)
    }
}

impl Default for PWMCONF {
    fn default() -> Self {
        Self(0xC10D0024)
    }
}


// Sanity Checks
// --------------------------------------------------------
