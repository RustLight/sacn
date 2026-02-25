// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file is based on an earlier error.rs created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! The errors used within the SacnLibrary. The ErrorKind subsection of this within the documentation contains details of all the errors.
//!
//! Errors from external sources are wrapped within thiserror.
//!
//! Io errors from std::io::Error are wrapped within Io(::std::io::Error)
//!
//! String errors from std::str::Utf8Error are wrapped within Str(::std::str::Utf8Error)
//!
//! Uuid errors from uuid::ParseError are wrapped within Uuid(uuid::ParseError)
//!
//! ParsePack related errors come within their own family wrapped inside this error to allow easy matching (can just match for SacnParsePackError rather than a specific).
//!
//! SacnParsePackError(sacn_parse_pack_error::Error, sacn_parse_pack_error::ErrorKind)

pub mod errors {
    use crate::sacn_parse_pack_error::ParsePacketError;
    use thiserror::Error;
    use uuid::Uuid;

    /// A specialized [`Result`] type for sACN operations.
    ///
    /// This type is used throughout the sACN crate for any operation which
    /// can produce an error.
    pub type Result<T> = std::result::Result<T, SacnError>;

    #[derive(Debug, Error)]
    pub enum SacnError {
        // Allow IO errors to be used with the error system.
        #[error("Io error occurred: {0}")]
        Io(#[from] std::io::Error),
        // Allow standard string library errors to be used with the error system.
        #[error("String error occurred: {0}")]
        Str(#[from] std::str::Utf8Error),
        // Allow UUID library to be used with error system.
        #[error("Uuid error occurred: {0}")]
        Uuid(#[from] uuid::Error),

        /// Returned to indicate that too many bytes were read to fit into supplied buffer.
        ///
        /// # Arguments
        /// usize: number of bytes read.
        ///
        /// usize: size of buffer.
        #[error("The given buffer fits {0} bytes, but {1} bytes were read.")]
        TooManyBytesRead(usize, usize),

        // All parse/pack errors live within the same SacnError group as described in sacn_parse_packet_error.
        #[error("SacnParsePack error occurred: {0}")]
        SacnParsePackError(#[from] ParsePacketError),

        /// Returned to indicate that an invalid or malformed source name was used.
        ///
        /// # Arguments
        /// String: A string describing why the source name is malformed.
        #[error("The given source name was malformed and couldn't be used, msg: {0}")]
        MalformedSourceName(String),

        /// Attempted to perform an action using a priority value that is invalid. For example sending with a priority > 200.
        /// This is distinct from the SacnParsePackError(ParseInvalidPriority) as it is for a local use of an invalid priority
        /// rather than receiving an invalid priority from another source.
        ///
        /// # Arguments
        /// The provided priority
        #[error(
            "Priority must be within allowed range of [0-E131_MAX_PRIORITY], priority provided: {0}"
        )]
        InvalidPriority(u8),

        /// Used to indicate that the limit for the number of supported sources has been reached.
        /// This is based on unique CID values.
        /// as per ANSI E1.31-2018 Section 6.2.3.3.
        ///
        /// # Arguments
        /// Number of sources
        #[error("Limit for the number of supported sources has been reached: {0}")]
        SourcesExceededError(usize),

        /// A source was discovered by a receiver with the `announce_discovery_flag` set to true.
        ///
        /// # Arguments
        /// The name of the source discovered.
        #[error("Source discovered with announce_discovery_flag set to true: {0}")]
        SourceDiscovered(String),

        /// Attempted to exceed the capacity of a single universe (`packet::UNIVERSE_CHANNEL_CAPACITY`).
        ///
        /// # Arguments
        /// Length of data provided.
        #[error("Attempted to exceed the capacity of a single universe, data len: {0}")]
        ExceedUniverseCapacity(usize),

        /// Attempted to use illegal universe, outwith allowed range of [`E131_MIN_MULTICAST_UNIVERSE`, `E131_MAX_MULTICAST_UNIVERSE`]
        /// + `E131_DISCOVERY_UNIVERSE` inclusive
        ///
        /// # Arguments
        /// u16: The provided universe.
        #[error("Attempted to use an illegal universe: {0}")]
        IllegalUniverse(u16),

        /// Attempted to use illegal universe as the sync universe, outwith allowed range of [`E131_MIN_MULTICAST_UNIVERSE`, `E131_MAX_MULTICAST_UNIVERSE`]
        /// + `E131_DISCOVERY_UNIVERSE` inclusive
        ///
        /// # Arguments
        /// u16: The provided synchronization universe.
        #[error("Attempted to use an illegal synchronization universe: {0}")]
        IllegalSyncUniverse(u16),

        /// Attempted to use a universe that wasn't first registered for use.
        /// To send from a universe with a sender it must first be registered. This allows universe discovery adverts to include the universe.
        ///
        /// # Arguments
        /// u16: The universe that was not registered.
        #[error("Attempted to use a universe that wasn't registered: {0}")]
        UniverseNotRegistered(u16),

        /// Ip version (ipv4 or ipv6) used when the other is expected.
        #[error("Multicast address and interface_addr not same IP version.")]
        IpVersionError(),

        /// Attempted to use an unsupported (not Ipv4 or Ipv6) IP version.
        ///
        /// # Arguments
        /// A string describing the situation where an unsupported IP version is used.
        #[error("Unsupported IP version used: {0}")]
        UnsupportedIpVersion(String),

        /// Attempted to use a sender which has already been terminated.
        ///
        /// # Arguments
        /// Name of terminated sender.
        #[error("Attempted to use a sender which has already been terminated: {0}")]
        SenderAlreadyTerminated(String),

        /// An error was encountered when attempting to merge DMX data together.
        #[error(
            "Error when merging DMX data. Attempted DMX merge on dmx data with different universes, synchronisation universes or data with no values"
        )]
        DmxMergeError(),

        /// Packet was received out of sequence and so should be discarded.
        ///
        /// # Arguments
        /// u8: The sequence number of the packet received.
        ///
        /// u8: The last sequence number received.
        ///
        /// isize: The difference between the last and current sequence numbers.
        #[error(
            "Packet received with sequence number {0} is out of sequence, last {1}, seq-diff {2}"
        )]
        OutOfSequence(u8, u8, isize),

        /// A source terminated a universe and this was detected when trying to receive data.
        /// This is only returned if the `announce_stream_termination` flag is set to true (default false).
        ///
        /// # Arguments
        /// Uuid: The CID of the source which sent the termination packet.
        ///
        /// u16: The universe that the termination packet is for.
        #[error("Source terminated universe, source cid: {0}, universe: {1}")]
        UniverseTerminated(Uuid, u16),

        /// A source universe timed out as no data was received on that universe within `E131_NETWORK_DATA_LOSS_TIMEOUT` as per ANSI E1.31-2018 Section 6.7.1.
        ///
        /// # Arguments
        /// uuid: The CID of the source which timed out.
        ///
        /// u16: The universe that timed out.
        #[error("Source universe timed out, source cid: {0}, universe: {1}")]
        UniverseTimeout(Uuid, u16),

        /// When looking for a specific universe it wasn't found. This might happen for example if trying to mute a universe on a receiver that
        /// wasn't being listened to.
        ///
        /// # Arguments
        /// u16: The universe that was not found.
        #[error("When looking for a specific universe it wasn't found, universe: {0}")]
        UniverseNotFound(u16),

        /// Attempted to find a source and failed. This might happen on a receiver for example if trying to remove a source which was never
        /// registered or discovered.
        ///
        /// # Arguments
        /// Uuid: The uuid of the source that was not found.
        #[error("Source not found: {0}")]
        SourceNotFound(Uuid),

        /// Thrown to indicate that the operation attempted is unsupported on the current OS
        /// For example this is used to indicate that multicast-IPv6 isn't supported current on Windows.
        ///
        /// # Arguments
        /// String: A message describing why this error was returned / the operation that was not supported.
        #[error("Operation attempted is unsupported on the current OS: {0}")]
        OsOperationUnsupported(String),

        /// Thrown to indicate that the source has corrupted for the reason specified by the error chain.
        /// This is currently only thrown if the source mutex is poisoned by a thread with access panic-ing.
        /// This prevents the panic propagating to the user of this library and allows them to handle it appropriately
        /// such as by creating a new source.
        ///
        /// # Arguments
        /// String: A message providing further details (if any) as to why the `SourceCorrupt` error was returned.
        #[error(
            "The sACN source has corrupted due to an internal panic! and should no longer be used, {0}"
        )]
        SourceCorrupt(String),

        /// Returned if the data array has length 0
        #[error("Data array has length 0, must provide data to send")]
        DataArrayEmpty(),

        /// Returned if the universe list has length 0
        #[error("Universe list has length 0, must provide at least one universe")]
        UniverseListEmpty(),

        /// Returned if the receiver has a source limit of 0
        #[error(
            "Source_limit has a value of Some(0) which would indicate this receiver can never receive from any source"
        )]
        SourceLimitZero(),

        /// This indicates that the only universe that can be received is the discovery universe.
        /// This means that having no timeout may lead to no data ever being received and so this method blocking forever
        /// to prevent this likely unintended behaviour throw a universe not registered error.
        #[error(
            "Attempting to receive data with no data universes registered, an infinite timeout and no discovery announcements"
        )]
        NoDataUniversesRegistered(),
    }
}
