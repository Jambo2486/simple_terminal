use crate::csi::csi;

// TODO: Make some shortcut functions like `embolden`, `italicise`, `highlight`, `reset`, etc.

pub enum Context {
	Foreground,
	Background,
}

pub enum Style {
	Reset,
	Bold,
	Italic,
	Underline,
}

impl Style {
	pub fn csi(&self) -> String {
		csi('m', &[match self {
			Style::Reset => '0', // Resets style and colour
			Style::Bold => '1',
			Style::Italic => '3',
			Style::Underline => '4',
		}.to_string()])
	}
}

pub enum Colour {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}

impl Colour {
	pub fn csi(&self, context: Context) -> String {
		csi('m', &[format!("{}{}",
			match context {
				Context::Foreground => '3',
				Context::Background => '4',
			},
			match self {
				Colour::Black => '0',
				Colour::Red => '1',
				Colour::Green => '2',
				Colour::Yellow => '3',
				Colour::Blue => '4',
				Colour::Magenta => '5',
				Colour::Cyan => '6',
				Colour::White => '7',
			}
		)])
	}
}

#[test]
fn test_style() {
	assert_eq!(format!("{}{}", Colour::Cyan.csi(Context::Foreground), Style::Bold.csi()), "\x1B[36m\x1B[1m".to_string())
}
