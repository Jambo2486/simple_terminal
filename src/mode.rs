use std::io;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use libc::{tcgetattr, tcsetattr, termios, TCSANOW, ECHO, ICANON}; // c_int, 
use crate::known_csi;

pub fn enable_raw() -> io::Result<()> {
	let stdin_fd = io::stdin().as_raw_fd();
	let mut termios_attr: termios = unsafe { std::mem::zeroed() };

	// Get the current terminal attributes
	if unsafe { tcgetattr(stdin_fd, &mut termios_attr) } != 0 {
		return Err(io::Error::last_os_error());
	}

	// Disable canonical mode and echo
	termios_attr.c_lflag &= !(ICANON | ECHO);

	// Set the new terminal attributes
	if unsafe { tcsetattr(stdin_fd, TCSANOW, &termios_attr) } != 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

pub fn disable_raw() -> io::Result<()> {
	let stdin_fd = io::stdin().as_raw_fd();
	let mut termios_attr: termios = unsafe { std::mem::zeroed() };

	// Get the current terminal attributes
	if unsafe { tcgetattr(stdin_fd, &mut termios_attr) } != 0 {
		return Err(io::Error::last_os_error());
	}

	// Restore the original terminal attributes
	if unsafe { tcsetattr(stdin_fd, TCSANOW, &termios_attr) } != 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

pub fn enable_alternate() -> io::Result<()> {
	let mut stdout = io::stdout();

	// Switch to the alternate screen
	write!(stdout, "{}", known_csi!('h', "?1049"))?;
	stdout.flush()?;

	Ok(())
}

pub fn disable_alternate() -> io::Result<()> {
	let mut stdout = io::stdout();

	// Switch back to the main screen
	write!(stdout, "{}", known_csi!('l', "?1049"))?;
	stdout.flush()?;
	
	Ok(())
}
