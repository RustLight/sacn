Command line used to find this crash:

/home/paul/.local/share/afl.rs/rustc-1.41.1-f3e1a95/afl.rs-0.6.0/afl/bin/afl-fuzz -i fuzz_in/ -o out target/debug/sacn-parse-fuzz-target

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 50.0 MB.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please drop
me a mail at <lcamtuf@coredump.cx> once the issues are fixed - I'd love to
add your finds to the gallery at:

  http://lcamtuf.coredump.cx/afl/

Thanks :-)
