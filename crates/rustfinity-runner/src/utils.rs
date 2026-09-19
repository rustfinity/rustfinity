use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use duct::cmd;

pub fn to_utf8(base64: &str) -> anyhow::Result<String> {
    let utf8 = BASE64_STANDARD.decode(base64)?;
    Ok(String::from_utf8(utf8)?)
}

pub struct CommandOutput {
    pub output: String,
    pub success: bool,
}

/// Runs a command with stderr merged into stdout, echoing every line to our own
/// stdout as it arrives and returning everything the command printed.
///
/// The output is streamed instead of captured-then-printed so the API can
/// forward `Compiling ...` progress to the browser while the build is still
/// running - a cold challenge build takes tens of seconds.
pub fn run_command_and_merge_output(
    command: &str,
    args: &[&str],
    cwd: Option<&str>,
) -> anyhow::Result<CommandOutput> {
    let cwd = cwd.unwrap_or(".");

    let reader = cmd(command, args)
        .stderr_to_stdout()
        .dir(cwd)
        // don't care about exit code
        .unchecked()
        .reader()?;

    let mut buffered = BufReader::new(reader);
    let mut output = String::new();
    let mut line = Vec::new();
    let mut stdout = std::io::stdout();

    loop {
        line.clear();
        if buffered.read_until(b'\n', &mut line)? == 0 {
            break;
        }

        let chunk = String::from_utf8_lossy(&line);
        write!(stdout, "{chunk}")?;
        stdout.flush()?;
        output.push_str(&chunk);
    }

    // The child is done once the pipe hits EOF, so this never blocks for long.
    let reader = buffered.into_inner();
    let success = reader
        .try_wait()?
        .is_some_and(|output| output.status.success());

    Ok(CommandOutput { output, success })
}

/// Prints text that is part of the streamed output, so ordering against the
/// command output above is preserved.
pub fn emit(text: &str) -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    write!(stdout, "{text}")?;
    stdout.flush()?;
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; // Ensure parent directories exist
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Replace the content if the file exists
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
