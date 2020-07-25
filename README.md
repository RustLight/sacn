# RUST Sacn
A Rust implementation of the ANSI E1.31 Streaming ACN protocol, tested against protocol version ANSI E1.31-2018. 

### Support for
* Sending and receiving data using the ANSI E1.31-2018 protocol over IPv4 and IPv6
* Unicast, Multicast and Broadcast Supported
* Tested on Windows and Linux
* Universe Synchronisation
* Universe Discovery

Originally forked from https://github.com/lschmierer/sacn and then further developed as part of a final year project at the University of St Andrews.


## INSTALLATION
The library is currently waiting for approval to upload to the public central rust cargo repository once submission marking is finished. 

In the meantime the library can be downloaded directly from here and placed in your project folder.

To add the library to a project edit the project Cargo.toml file to add the following line to the dependencies section.

    [dependencies]
    sacn = { path = "<PATH>"}
Where \<PATH\> is the path to the location of the library Lib folder.

Once the library has been uploaded to the cargo repository it can be installed by instead using the line below in the Cargo.toml file. Note the version will change as further updates are made.
 

    [dependencies]
    sacn = "1.0.0"

## Getting Started
The cargo documentation for this library (generation instructions in usage.pdf) describes how to use the library with examples for common use cases.  This documentation will also be uploaded to the cargo repository once submission marking is finished. 

If the documentation cannot be generated then it is also included as a webpage within the "Code Documentation" folder with the entry point being the "index.html" file within the "sacn" subfolder.

The requirements should automatically be handled by cargo once the library is imported. This library was developed on rustc 1.41.1 (f3e1a954d 2020-02-24) and does not guarantee support for earlier versions.

Note that previous rust version will not work, errors talking about 'rust dbghelp.rs:110' are due to using an old version of rust, see https://github.com/rust-lang/backtrace-rs/issues/276 .

## Testing

## Demo Implementation
A demo sender and receiver are included which demonstrate usage of the code and are used for many of the integration/interoperability tests. These programs can be run from within the sACN library folder. Details of how to run these programs are included in the usage.pdf document.
