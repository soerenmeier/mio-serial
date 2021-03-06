//! # mio-serial - Serial port I/O for mio
//!
//! This crate provides a serial port implementation compatable with mio.
//!
//! **Windows support is present but largely untested by the author**
//!
//! ## Links
//!   - repo:  https://github.com/berkowski/mio-serial
//!   - docs:  https://docs.rs/mio-serial
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

// Enums, Structs, and Traits from the serialport crate
pub use serialport::{
    // Enums
    ClearBuffer,
    DataBits,
    // Structs
    Error,
    ErrorKind,
    FlowControl,
    Parity,
    // Traits
    SerialPort,

    SerialPortInfo,
    SerialPortBuilder,

    StopBits,
};

pub use serialport::new as new_builder;

// Re-export port-enumerating utility function.
pub use serialport::available_ports;

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use unix::Serial;

/// A type for results generated by interacting with serial ports.
pub type Result<T> = serialport::Result<T>;
