# TMC5130

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/EdBuilds/TMC5130/ci.yml?style=for-the-badge&labelColor=555555)
![Crates.io Version](https://img.shields.io/crates/v/tmc5130?style=for-the-badge&labelColor=555555)
![docs.rs](https://img.shields.io/docsrs/tmc5130?style=for-the-badge&labelColor=555555)


TMC5130 SPI stepper motor driver written in rust

## Overview

This crate is a platform-agnostic Rust driver for the TMC5130 SPI stepper driver. This crate provides a high-level interface to interact with the TMC5130 chip.

The driver is designed to work with any hardware abstraction layer (HAL) that implements the embedded-hal v1.0.0 traits and works in `no_std` environments.

## Features

## Installation

To use this crate in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
tmc5130 = "0.1.0"