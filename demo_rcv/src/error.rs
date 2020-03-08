/// Use the error-chain system to allow handling IO and sACN errors chained together.
pub mod errors {
    error_chain! {
        foreign_links {
            Sacn(::sacn::error::errors::Error);
            Io(::std::io::Error);
        }
        
        errors {}
    }
}

