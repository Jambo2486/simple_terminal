use crate::known_csi;

/// Does not move cursor
pub fn line() -> String {
	known_csi!('K', "2")
}

/// Erases from the cursor to the end of the current line
/// Does not move cursor
pub fn line_after_cursor() -> String {
	known_csi!('K', "0")
}

/// Erases from the beginning of the current line to the cursor
/// Does not move cursor
pub fn line_before_cursor() -> String {
	known_csi!('K', "1")
}

/// Moves cursor to (0,0)
/// Does not clear scrollback buffer
pub fn screen() -> String {
	known_csi!('J', "2")
}
