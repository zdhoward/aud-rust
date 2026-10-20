//! `aud` binary. M0 smoke build: prints its version only.

#![forbid(unsafe_code)]

fn main() {
    let arg = std::env::args().nth(1);
    if let Some("--version" | "-V") = arg.as_deref() {
        println!("aud {}", aud_core::version());
    } else {
        eprintln!(
            "aud {}: not implemented yet (use --version)",
            aud_core::version()
        );
        std::process::exit(2);
    }
}
