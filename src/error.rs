// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// The errors used within the SacnLibrary. The ErrorKind subsection of this within the documentation contains details of all the errors.
///
/// Errors from external sources are wrapped within this error-chain.
///
/// Io errors from std::io::Error are wrapped within Io(::std::io::Error)
///
/// String errors from std::str::Utf8Error are wrapped within Str(::std::str::Utf8Error)
///
/// Uuid errors from uuid::ParseError are wrapped within Uuid(uuid::ParseError)
///
///
/// ParsePack related errors come within their own family wrapped inside this error to allow easy matching (can just match for SacnParsePackError rather than a specific).
///
/// SacnParsePackError(sacn_parse_pack_error::Error, sacn_parse_pack_error::ErrorKind)
///
/// Uses the error-chain crate to allow errors to allow more informative backtraces through error chaining.
/// https://docs.rs/error-chain/0.12.2/error_chain/
use thiserror::Error;
use crate::sacn_parse_pack_error::sacn_parse_pack_error::SacnParsePackError;
use uuid::Uuid;
use std::io;
use std::str::Utf8Error;

pub mod errors {
    use super::*;
    
    pub type Result<T> = std::result::Result<T, SacnError>;

    #[derive(Debug, Error)]
    pub enum SacnError {
        #[error("IO error")]
        Io(#[from] io::Error),

        #[error("UTF8 error")]
        Utf8(#[from] Utf8Error),

        #[error("UUID parse error")]
        Uuid(#[from] uuid::Error),

        #[error(transparent)]
        SacnParsePackError(#[from] SacnParsePackError),

        #[error("The given source name was malformed and couldn't be used, msg: {0}")]
        MalformedSourceName(String),

        #[error("Attempted to perform an action using a priority value that is invalid, msg: {0}")]
        InvalidPriority(String),

        #[error("Limit for the number of supported sources has been reached, msg: {0}")]
        SourcesExceededError(String),

        #[error("A source was discovered by a receiver with the announce_discovery_flag set to true, source name: {0}")]
        SourceDiscovered(String),

        #[error("Attempted to exceed the capacity of a single universe, msg: {0}")]
        ExceedUniverseCapacity(String),

        #[error("Illegal universe used, outwith allowed range, msg: {0}")]
        IllegalUniverse(String),

        #[error("Attempted to use a universe that wasn't first registered for use, msg: {0}")]
        UniverseNotRegistered(String),

        #[error("IP version (ipv4 or ipv6) used when the other is expected, msg: {0}")]
        IpVersionError(String),

        #[error("Unsupported IP version used, msg: {0}")]
        UnsupportedIpVersion(String),

        #[error("Attempted to use a sender which has already been terminated, msg: {0}")]
        SenderAlreadyTerminated(String),

        #[error("Error when merging DMX data, msg: {0}")]
        DmxMergeError(String),

        #[error("Packet was received out of sequence and should be discarded, msg: {0}")]
        OutOfSequence(String),

        #[error("Source cid: {0} terminated universe: {1}")]
        UniverseTerminated(Uuid, u16),

        #[error("(Source,Universe) timed out: ({0},{1})")]
        UniverseTimeout(Uuid, u16),

        #[error("Universe not found, msg: {0}")]
        UniverseNotFound(String),

        #[error("Source not found, msg: {0}")]
        SourceNotFound(String),

        #[error("Operation attempted is unsupported on the current OS, msg: {0}")]
        OsOperationUnsupported(String),

        #[error("The sACN source has corrupted due to an internal panic and should no longer be used, {0}")]
        SourceCorrupt(String),
    }
}
