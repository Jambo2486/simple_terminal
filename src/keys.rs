pub enum Key {
	Character(char),
	Enter,
	Backspace,
	Escape,
	// Add more keys
	Unknown
}

pub fn byte_to_key(byte: u8) -> Option<Key> {
	match byte {
		byte if byte.is_ascii_graphic() => Some(Key::Character(byte as char)),
		b'\n' => Key::Enter,
		b'\x0a' => Key::Enter,
		b'\x1B' => Key::Escape,
		b'\x7F' => Key::Backspace,
		_ => Key::Unknown,
	}
}
