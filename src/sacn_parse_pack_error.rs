// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! The errors used within the SacnLibrary specifically those related to parsing and packeting packets received/sent on the network.

use crate::String;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ParsePacketError {
    /// When parsing packet invalid data encountered.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to what data was invalid.
    #[error("Error when parsing data into packet, msg: {0}")]
    ParseInvalidData(String),

    /// Attempted to parse a priority value that is outwith the allowed range of [0, E131_MAX_PRIORITY].
    /// As per ANSI E1.31-2018 Section 6.2.3
    ///
    /// # Arguments
    /// u8: the priority value that was invalid.
    #[error(
        "Attempted to parse a priority value that is outwith the allowed range of [0, 200]: {0}"
    )]
    ParseInvalidPriority(u8),

    /// Attempted to parse a page value that is invalid - e.g. the page value is higher than the last_page value.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the page was invalid.
    #[error("Error when parsing page value, msg: {0}")]
    ParseInvalidPage(String),

    /// Attempted to parse a sync address value that is outwith the allowed range of [0, E131_MAX_MULTICAST_UNIVERSE].
    /// As per ANSI E1.31-2018 Section 9.1.1.
    ///
    /// # Arguments
    /// u16: the synchronisation address that was invalid.
    #[error(
        "Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999]: {0}"
    )]
    ParseInvalidSyncAddr(u16),

    /// Attempted to parse a universe value that is outwith the allowed range of [1, E131_MAX_MULTICAST_UNIVERSE].
    /// As per ANSI E1.31-2018 Section 9.1.1.
    ///
    /// # Arguments
    /// u16: the universe value that was invalid.
    #[error(
        "Attempted to parse a universe value that is outwith the allowed range of [1, 63999]: {0}"
    )]
    ParseInvalidUniverse(u16),

    /// Attempted to parse a packet with an invalid ordering of universes.
    /// For example a discovery packet where the universes aren't correctly ordered in assending order.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the universe ordering was invalid.
    #[error("Attempted to parse a packet with an invalid ordering of universes, msg: {0}")]
    ParseInvalidUniverseOrder(String),

    /// When packing a packet into a buffer invalid data encountered.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the data couldn't be packed.
    #[error("When packing a packet into a buffer invalid data encountered, msg: {0}")]
    PackInvalidData(String),

    /// Supplied buffer is not large enough to pack packet into.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the pack buffer is insufficient.
    #[error("Supplied buffer is not large enough to pack packet into, msg: {0}")]
    PackBufferInsufficient(String),

    /// Supplied buffer does not contain enough data.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why there was insufficient data for parsing.
    #[error("Supplied buffer does not contain enough data, msg: {0}")]
    ParseInsufficientData(String),

    /// Received PDU flags are invalid for parsing.
    ///
    /// # Arguments
    /// flags: The flags that were found which are invalid.
    #[error("PDU Flags {0:#b} are invalid for parsing")]
    ParsePduInvalidFlags(u8),

    /// Received PDU length is invalid.
    ///
    /// # Arguments
    /// len: The length provided in the Pdu which is invalid.
    #[error("PDU Length {0} is invalid")]
    PduInvalidLength(usize),

    /// Received PDU vector is invalid/unsupported by this library.
    ///
    /// # Arguments
    /// vec: The vector parsed which is invalid / cannot be used.
    #[error("Vector {0:#x} not supported")]
    PduInvalidVector(u32),

    /// Error parsing the received UUID.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the uuid (used for CID) couldn't be parsed.
    #[error("Error parsing the received UUID: {0}")]
    UuidError(Uuid),

    /// Error parsing received UTF8 string.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the string couldn't be parsed.
    #[error("Error parsing received UTF8 string, msg: {0}")]
    Utf8Error(String),

    /// Source name in packet was not null terminated.
    ///
    /// # Arguments
    /// msg: A message providing further details (if any) as to why the source name was invalid.
    #[error("Source name in packet was not null terminated.")]
    SourceNameNotNullTerminated(),
}
