pub enum Key {
	Character(char),
	Up,
	Down,
	Left,
	Right,
	Enter,
	Backspace,
	Escape,
	// Add more keys
	Unknown
}

pub fn byte_to_key(byte: u8) -> Key {
	match byte {
		byte if byte.is_ascii_graphic() => Key::Character(byte as char),
		b'\x0A' => Key::Up,
		b'\x0B' => Key::Down,
		b'\x0C' => Key::Right,
		b'\x0D' => Key::Left,
		b'\x0a' | b'\n' => Key::Enter,
		b'\x1B' => Key::Escape,
		b'\x7F' => Key::Backspace,
		_ => Key::Unknown
	}
}
