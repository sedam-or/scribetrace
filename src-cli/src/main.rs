//! `scribetrace-cli`: thin entry point; everything lives in the library so it can be tested.

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let code = scribetrace_cli::run(&args, &mut stdout.lock(), &mut stderr.lock());
    // A closed pipe must not turn into a panic or a lost exit code.
    let _ = std::io::stdout().flush();
    ExitCode::from(code)
}
