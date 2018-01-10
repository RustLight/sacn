// Copyright 2017 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::str::Utf8Error;
use core::fmt;

#[cfg(feature = "std")]
use std::error::Error;

use uuid::ParseError as UUidParseError;

/// Error for parsing of sACN network packets.
#[derive(Debug)]
pub enum ParseError<'a> {
    Uuid(UUidParseError),
    Utf8(Utf8Error),
    PduInvalidLength(usize),
    PduInvalidFlags(u8),
    PduVectorNotSupported(u32),
    OtherInvalidData(&'a str),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Uuid(ref err) => write!(f, "UUID parsing error: {}", err),
            ParseError::Utf8(ref err) => write!(f, "UTF8 error: {}", err),
            ParseError::PduInvalidLength(len) => write!(f, "Length {} is invalid", len),
            ParseError::PduInvalidFlags(flags) => write!(f, "Flags {} are invalid", flags),
            ParseError::PduVectorNotSupported(vec) => write!(f, "Vector {} not supported", vec),
            ParseError::OtherInvalidData(ref msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::Uuid(ref err) => err.description(),
            ParseError::Utf8(ref err) => err.description(),
            ParseError::PduInvalidLength(_) => "PDU invalid length",
            ParseError::PduInvalidFlags(_) => "PDU invalid flags",
            ParseError::PduVectorNotSupported(_) => "PDU vector not supported",
            ParseError::OtherInvalidData(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Uuid(ref err) => Some(err),
            ParseError::Utf8(ref err) => Some(err),
            ParseError::PduInvalidLength(_) => None,
            ParseError::PduInvalidFlags(_) => None,
            ParseError::PduVectorNotSupported(_) => None,
            ParseError::OtherInvalidData(_) => None,
        }
    }
}

impl<'a> From<UUidParseError> for ParseError<'a> {
    fn from(err: UUidParseError) -> ParseError<'a> {
        ParseError::Uuid(err)
    }
}

impl<'a> From<Utf8Error> for ParseError<'a> {
    fn from(err: Utf8Error) -> ParseError<'a> {
        ParseError::Utf8(err)
    }
}
