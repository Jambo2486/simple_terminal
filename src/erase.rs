use crate::csi::csi;
use crate::known_csi;

/// Does not move cursor
pub fn line() {
	known_csi!('K', "2");
}

/// Erases from the cursor to the end of the current line
/// Does not move cursor
pub fn line_after_cursor() {
	csi('K', &["0".to_string()]);
}

/// Erases from the beginning of the current line to the cursor
/// Does not move cursor
pub fn line_before_cursor() {
	csi('K', &["1".to_string()]);
}

/// Moves cursor to (0,0)
/// Does not clear scrollback buffer
pub fn screen() {
	csi('J', &["2".to_string()]);
}