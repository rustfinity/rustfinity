use std::io::{self, Read, Write};

/// Ask a yes/no question and return `true` for yes, `false` for no.
/// The user only needs to press a single key (y/n) — no Enter required.
/// `default_yes` controls what happens when the user presses Enter without typing.
pub fn confirm(prompt: &str, default_yes: bool) -> io::Result<bool> {
    let hint = if default_yes { "Y/n" } else { "y/N" };
    print!("{} [{}] ", prompt, hint);
    io::stdout().flush()?;

    let result = read_single_key();

    match result {
        Ok(ch) => {
            // Echo the character and newline
            if ch == '\n' || ch == '\r' {
                println!();
            } else {
                println!("{}", ch);
            }

            match ch {
                'y' | 'Y' => Ok(true),
                'n' | 'N' => Ok(false),
                '\n' | '\r' => Ok(default_yes),
                _ => Ok(default_yes),
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(unix)]
fn read_single_key() -> io::Result<char> {
    use std::os::unix::io::AsRawFd;

    let stdin_fd = io::stdin().as_raw_fd();
    let mut old_termios: libc::termios = unsafe { std::mem::zeroed() };

    // Save current terminal settings
    if unsafe { libc::tcgetattr(stdin_fd, &mut old_termios) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut new_termios = old_termios;

    // Disable canonical mode (line buffering) and echo
    new_termios.c_lflag &= !(libc::ICANON | libc::ECHO);
    // Read one character at a time
    new_termios.c_cc[libc::VMIN] = 1;
    new_termios.c_cc[libc::VTIME] = 0;

    if unsafe { libc::tcsetattr(stdin_fd, libc::TCSANOW, &new_termios) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut buf = [0u8; 1];
    let result = io::stdin().read_exact(&mut buf);

    // Always restore terminal settings
    unsafe {
        libc::tcsetattr(stdin_fd, libc::TCSANOW, &old_termios);
    }

    result?;
    Ok(buf[0] as char)
}

#[cfg(windows)]
fn read_single_key() -> io::Result<char> {
    // On Windows, fall back to reading a line
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.chars().next().unwrap_or('\n'))
}
