// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Errors used through the sacn crate.

#![warn(missing_docs)]

use core::fmt;
use core::str::Utf8Error;

#[cfg(feature = "std")]
use std::error;

use uuid;

/// Errors for parsing of sACN network packets.
#[derive(Debug)]
pub enum ParseError {
    /// Received PDU flags are invalid.
    PduInvalidFlags(u8),

    /// Received PDU length is invalid.
    PduInvalidLength(usize),

    /// Received PDU vector is invalid.
    PduInvalidVector(u32),

    /// Other invalid data received.
    InvalidData(&'static str),

    /// Buffer does not contain enough data.
    NotEnoughData,

    /// Error parsing the received UUID.
    UuidError(uuid::ParseError),

    /// Error parsing received UTF8 string.
    Utf8Error(Utf8Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::PduInvalidFlags(flags) => write!(f, "Flags {:#b} are invalid", flags),
            ParseError::PduInvalidLength(len) => write!(f, "Length {} is invalid", len),
            ParseError::PduInvalidVector(vec) => write!(f, "Vector {:#x} not supported", vec),
            ParseError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ParseError::NotEnoughData => write!(f, "Not enough data supplied"),
            ParseError::UuidError(err) => write!(f, "UUID parsing error: {}", err),
            ParseError::Utf8Error(err) => write!(f, "UTF8 error: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::PduInvalidFlags(_) => "PDU invalid flags",
            ParseError::PduInvalidLength(_) => "PDU invalid length",
            ParseError::PduInvalidVector(_) => "PDU vector not supported",
            ParseError::InvalidData(msg) => msg,
            ParseError::NotEnoughData => "Not enough data supplied",
            ParseError::UuidError(ref err) => err.description(),
            ParseError::Utf8Error(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::UuidError(ref err) => Some(err),
            ParseError::Utf8Error(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<uuid::ParseError> for ParseError {
    fn from(err: uuid::ParseError) -> ParseError {
        ParseError::UuidError(err)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> ParseError {
        ParseError::Utf8Error(err)
    }
}

/// Errors for packing of sACN network packets.
#[derive(Debug)]
pub enum PackError {
    /// Packet contains invalid data.
    InvalidData(&'static str),

    /// Supplied buffer is not large enough.
    BufferNotLargeEnough,
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PackError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            PackError::BufferNotLargeEnough => write!(f, "Supplied buffer is not large enough"),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for PackError {
    fn description(&self) -> &str {
        match *self {
            PackError::InvalidData(msg) => msg,
            PackError::BufferNotLargeEnough => "Supplied buffer is not large enough",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}

impl From<ParseError> for PackError {
    fn from(err: ParseError) -> PackError {
        PackError::InvalidData(match err {
            ParseError::PduInvalidFlags(_) => "PDU invalid flags",
            ParseError::PduInvalidLength(_) => "PDU invalid length",
            ParseError::PduInvalidVector(_) => "PDU vector not supported",
            ParseError::InvalidData(msg) => msg,
            ParseError::NotEnoughData => "Not enough data supplied",
            ParseError::UuidError(_) => "Invalid UUID",
            ParseError::Utf8Error(_) => "Invalid UTF-8",
        })
    }
}
