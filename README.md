**INSTALLATION**
The library is currently waiting for approval to upload to the public central rust cargo repository once submission marking is finished. 

In the meantime the library can be downloaded directly from here and placed in your project folder.

To add the library to a project edit the project Cargo.toml file to add the following line to the dependencies section.

    [dependencies]
    sacn = { path = "<PATH>"}
Where \<PATH\> is the path to the location of the library Lib folder.

Once the library has been uploaded to the cargo repository it can be installed by instead using the line below in the Cargo.toml file. Note the version will change as further updates are made.
 

    [dependencies]
    sacn = "1.0.0"

**USAGE**
The cargo documentation for this library (generation instructions in usage.pdf) describes how to use the library with examples for common use cases.  This documentation will also be uploaded to the cargo repository once submission marking is finished. 

If the documentation cannot be generated then it is also included as a webpage within the "Code Documentation" folder with the entry point being the "index.html" file within the "sacn" subfolder.

**REQUIREMENTS**
The requirements should automatically be handled by cargo once the library is imported. This library was developed on rustc 1.41.1 (f3e1a954d 2020-02-24) and does not guarantee support for earlier versions.

Note that previous rust version will not work, errors talking about 'rust dbghelp.rs:110' are due to using an old version of rust, see https://github.com/rust-lang/backtrace-rs/issues/276 .
