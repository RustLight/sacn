// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// The errors used within the SacnLibrary specifically those related to parsing and packeting packets received/sent on the network.
///

pub mod sacn_parse_pack_error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum SacnParsePackError {
        #[error("Error when parsing data into packet, msg: {0}")]
        ParseInvalidData(String),

        #[error("Attempted to parse a priority value that is outwith the allowed range of [0, 200], msg: {0}")]
        ParseInvalidPriority(String),

        #[error("Error when parsing page value, msg: {0}")]
        ParseInvalidPage(String),

        #[error("Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999], msg: {0}")]
        ParseInvalidSyncAddr(String),

        #[error("Attempted to parse a universe value that is outwith the allowed range of [1, 63999], msg: {0}")]
        ParseInvalidUniverse(String),

        #[error("Attempted to parse a packet with an invalid ordering of universes, msg: {0}")]
        ParseInvalidUniverseOrder(String),

        #[error("When packing a packet into a buffer invalid data encountered, msg: {0}")]
        PackInvalidData(String),

        #[error("Supplied buffer is not large enough to pack packet into, msg: {0}")]
        PackBufferInsufficient(String),

        #[error("Supplied buffer does not contain enough data, msg: {0}")]
        ParseInsufficientData(String),

        #[error("PDU Flags {0:#b} are invalid for parsing")]
        ParsePduInvalidFlags(u8),

        #[error("PDU Length {0} is invalid")]
        PduInvalidLength(usize),

        #[error("Vector {0:#x} not supported")]
        PduInvalidVector(u32),

        #[error("Error parsing the received UUID, msg: {0}")]
        UuidError(String),

        #[error("Error parsing received UTF8 string, msg: {0}")]
        Utf8Error(String),

        #[error("Attempted to parse invalid source name, msg: {0}")]
        SourceNameInvalid(String),
    }
}