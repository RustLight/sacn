// SPDX-License-Identifier: MIT OR Apache-2.0
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// Use the error-chain system to allow handling IO and sACN errors chained together.
/// `error_chain!` macro automatically creates the Error / `ErrorKind` / Result required to use the Errors/external errors below with error-chain.
///
/// Sacn create errors are wrapped in `Sacn(::sacn::error::errors::Error)`.
///
/// Std io errors are wrapped in `Io(::std::io::Error)`.
pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DemoError {
        #[error(transparent)]
        Sacn(#[from] ::sacn::error::errors::SacnError),

        #[error(transparent)]
        Io(#[from] ::std::io::Error),
    }

    pub type Result<T> = std::result::Result<T, DemoError>;
}
