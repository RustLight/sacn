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
/// 
/// Boolean parse errors from Std str ParseBoolError are wrapped in BoolStr(::std::str::ParseBoolError).
pub mod errors {
    error_chain! {
        foreign_links {
            Sacn(::sacn::error::errors::Error);
            Io(::std::io::Error);
            BoolStr(::std::str::ParseBoolError);
        }
        
        errors {}
    }
}

