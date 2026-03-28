//! `ft6x06-rs` is a pure-Rust `embedded-hal`-based driver for the I2C-based `ft6x06` capacitive touch
//! screen controller. This crate aims to provide high-level functionality for most use-cases.
//!
//! All interactions are through the `FT6x06` or `FT6x06Async` struct depending on the enabled features.
//!
//! **Features**
//! - `sync-driver` - default, sync driver using `embedded-hal`.
//! - `async-driver` - async version of the driver using `embedded-hal-async`.
//! - `defmt` - default, enables `defmt::Format` derives on public types. Disable if using `defmt` 1.0+.
//!
//! The sync and async drivers are the same except async includes `wait_for_touch` functionality.

#![no_std]

mod common;

#[cfg(feature = "sync-driver")]
mod sync;
#[cfg(feature = "sync-driver")]
pub use sync::FT6x06;

#[cfg(feature = "async-driver")]
mod asynch;
#[cfg(feature = "async-driver")]
pub use asynch::FT6x06Async;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TouchEvent {
    pub primary_point: TouchPoint,
    pub secondary_point: Option<TouchPoint>,
    pub gesture: GestureType,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TouchPoint {
    pub x: u16,
    pub y: u16,
    pub weight: u8,
    pub area: u8,
    pub touch_type: TouchType,
    pub touch_id: u8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TouchType {
    PressDown,
    LiftUp,
    Contact,
    NoEvent,
    Invalid,
}

impl TouchType {
    pub fn from_register(reg: u8) -> Self {
        let data = reg >> 6;
        Self::from(data)
    }
}

impl From<u8> for TouchType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::PressDown,
            1 => Self::LiftUp,
            2 => Self::Contact,
            3 => Self::NoEvent,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GestureType {
    MoveUp,
    MoveRight,
    MoveDown,
    MoveLeft,
    ZoomIn,
    ZoomOut,
    NoGesture,
    Invalid,
}

impl From<u8> for GestureType {
    fn from(value: u8) -> Self {
        match value {
            0x10 => Self::MoveUp,
            0x14 => Self::MoveRight,
            0x18 => Self::MoveDown,
            0x1C => Self::MoveLeft,
            0x48 => Self::ZoomIn,
            0x49 => Self::ZoomOut,
            0x00 => Self::NoGesture,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControlMode {
    ForceActive = 0,
    MonitorIdle = 1,
}

impl TryFrom<u8> for ControlMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ControlMode::ForceActive),
            1 => Ok(ControlMode::MonitorIdle),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptMode {
    Poll = 0,
    Trigger = 1,
}

impl TryFrom<u8> for InterruptMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Poll),
            1 => Ok(Self::Trigger),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DriverError<I2CError> {
    I2cError(I2CError),
    InvalidResponse,
    IrqError,
}

impl<I2CError> From<I2CError> for DriverError<I2CError> {
    fn from(value: I2CError) -> Self {
        Self::I2cError(value)
    }
}
