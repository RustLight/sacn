/// Use the error-chain system to allow handling IO and sACN errors chained together.
/// error_chain! macro automatically creates the Error / ErrorKind / Result required to use the Errors/external errors below with error-chain.
pub mod errors {
    error_chain! {
        foreign_links {
            Sacn(::sacn::error::errors::Error);
            Io(::std::io::Error);
        }
        
        errors {}
    }
}

