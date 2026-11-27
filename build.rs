// SPDX-License-Identifier: MIT OR Apache-2.0
fn main() {
    if std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("cargo:rustc-cfg=ci");
    }
}
