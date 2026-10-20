//! Command-line front end. All behavior lives in `aud-core`; this crate parses
//! arguments, prints documents and maps errors to exit codes.

#![forbid(unsafe_code)]

use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

use aud_core::contracts::{json_document, to_ascii_json};
use aud_core::{AudError, Dir, DirOptions};
use clap::{Args, Parser, Subcommand};
use serde_json::{Value, json};

const USAGE_EXIT: i32 = 2;

#[derive(Parser)]
#[command(name = "aud", version, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List selected file names.
    List(SelectArgs),
}

/// Options shared by every command (Python `_parser`).
#[derive(Args)]
struct SelectArgs {
    directory: PathBuf,
    /// Explicit selection; default selects nothing (Python API parity).
    #[arg(long, num_args = 1.., value_name = "EXT")]
    extensions: Vec<String>,
    /// Narrow selection with a filename glob.
    #[arg(long)]
    pattern: Option<String>,
    /// Emit one JSON document on stdout.
    #[arg(long)]
    json: bool,
    /// Opt into versioned --json output.
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=1))]
    json_version: Option<u8>,
    /// Write file-level progress to stderr.
    #[arg(long)]
    progress: bool,
    /// Per-process seconds (accepted for parity; `list` starts no processes).
    #[arg(long, default_value_t = 300.0)]
    timeout: f64,
    /// Media/package output bound in bytes (accepted for parity).
    #[arg(long)]
    output_bytes: Option<i64>,
}

/// Run the CLI with `args` (without the program name). Returns the exit code:
/// 0 success, 1 execution failure, 2 usage/configuration error, 130 cancelled.
pub fn run(args: &[OsString], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    if let Some(code) = legacy_form(args, out) {
        return code;
    }
    let json_requested = args.iter().any(|a| a == "--json");
    let full = std::iter::once(OsString::from("aud")).chain(args.iter().cloned());
    let cli = match Cli::try_parse_from(full) {
        Ok(cli) => cli,
        Err(e) if !e.use_stderr() => {
            let _ = write!(out, "{e}");
            return 0;
        }
        Err(e) => return usage_error(&clap_message(&e), json_requested, out, err),
    };
    let Command::List(args) = cli.command;
    if args.json_version.is_some() && !args.json {
        return usage_error("--json-version requires --json", json_requested, out, err);
    }
    match execute_list(&args) {
        Ok(data) => {
            emit(&data, args.json, args.json_version, "list", out);
            0
        }
        Err(e) => {
            let doc = error_data(&e.to_string(), USAGE_EXIT);
            if args.json {
                emit(&doc, true, args.json_version, "error", out);
            } else {
                let _ = writeln!(err, "{}", to_ascii_json(&doc, true));
            }
            USAGE_EXIT
        }
    }
}

const COMMANDS: [&str; 7] = [
    "list", "inspect", "convert", "rename", "effects", "export", "album",
];

/// The earlier read-only entry forms: no arguments prints usage; a single
/// non-option argument that is not a command counts files (always with the
/// default empty selection). Returns `None` for the normal command path.
fn legacy_form(args: &[OsString], out: &mut dyn Write) -> Option<i32> {
    match args {
        [] => {
            let _ = writeln!(out, "Usage: aud <command> <directory> [options]");
            Some(1)
        }
        [only]
            if !only.to_string_lossy().starts_with('-')
                && !COMMANDS.contains(&only.to_string_lossy().as_ref()) =>
        {
            let directory = PathBuf::from(only);
            if !directory.is_dir() {
                let quoted =
                    to_ascii_json(&Value::from(directory.to_string_lossy().as_ref()), false);
                let _ = writeln!(out, "Directory does not exist: {quoted}");
                return Some(1);
            }
            match Dir::open(&directory, &DirOptions::default()) {
                Ok(dir) => {
                    let _ = writeln!(out, "Found {} files", dir.files().len());
                    Some(0)
                }
                Err(e) => {
                    let _ = writeln!(out, "{e}");
                    Some(1)
                }
            }
        }
        _ => None,
    }
}

fn execute_list(args: &SelectArgs) -> Result<Value, AudError> {
    let dir = Dir::open(
        &args.directory,
        &DirOptions {
            extensions: args.extensions.clone(),
            pattern: args.pattern.clone(),
            ..DirOptions::default()
        },
    )?;
    Ok(json!({ "files": dir.get_all() }))
}

fn error_data(message: &str, exit_code: i32) -> Value {
    json!({"succeeded": false, "error": message, "exit_code": exit_code, "result": null})
}

/// First line of clap's message without its `error: ` prefix.
fn clap_message(e: &clap::Error) -> String {
    let text = e.to_string();
    let first = text.lines().next().unwrap_or_default();
    first.strip_prefix("error: ").unwrap_or(first).to_owned()
}

/// Argument errors keep the legacy unversioned shape (`JSON_CONTRACTS.md`).
fn usage_error(message: &str, json: bool, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    if json {
        let _ = writeln!(
            out,
            "{}",
            to_ascii_json(&error_data(message, USAGE_EXIT), false)
        );
    } else {
        let quoted = to_ascii_json(&Value::from(message), false);
        let _ = writeln!(err, "aud: {quoted}");
    }
    USAGE_EXIT
}

/// Print `data`, wrapped in a v1 envelope when `version` is set. Machine output
/// is compact; otherwise indented. Mirrors Python `_emit`.
fn emit(data: &Value, machine: bool, version: Option<u8>, kind: &str, out: &mut dyn Write) {
    let doc = match version {
        // `kind` is always one of our own literals and `data` always an object, so
        // `json_document` cannot fail here; the fallback only avoids a panic path.
        Some(_) => json_document(kind, data.clone()).unwrap_or_else(|_| data.clone()),
        None => data.clone(),
    };
    let _ = writeln!(out, "{}", to_ascii_json(&doc, !machine));
}
