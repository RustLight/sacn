#![recursion_limit="1024"]
#[macro_use]
extern crate error_chain;

/// Use the error-chain system to allow handling IO and sACN errors chained together.
pub mod errors {
    error_chain! {
        foreign_links {
            Sacn(Sacn::error::errors);
            Io(::std::io::Error);      
        }
    }
}