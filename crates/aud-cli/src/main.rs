//! `aud` binary: forwards process arguments and streams to [`aud_cli::run`].

#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    let code = aud_cli::run(
        &args,
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    // Exit codes are 0, 1, 2 or 130, all of which fit in u8.
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}
