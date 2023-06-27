pub enum Key {
	Character(char),
	Enter,
	Backspace,
	Escape,
	// Add more keys
}

pub fn byte_to_key(byte: u8) -> Option<Key> {
	match byte {
		byte if byte.is_ascii_graphic() => Some(Key::Character(byte as char)),
		b'\n' => Some(Key::Enter),
		b'\x0a' => Some(Key::Enter),
		b'\x1B' => Some(Key::Escape),
		b'\x7F' => Some(Key::Backspace),
		_ => None,
	}
}
