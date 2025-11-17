// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// Use the error-chain system to allow handling IO and sACN errors chained together.
/// error_chain! macro automatically creates the Error / ErrorKind / Result required to use the Errors/external errors below with error-chain.
///
/// Sacn create errors are wrapped in Sacn(::sacn::error::errors::Error).
///
/// Std io errors are wrapped in Io(::std::io::Error).

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
