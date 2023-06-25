use crate::csi::csi;

/// Parse position to 0-based index.
fn parse_pos(position: usize) -> String {
	(position - 1).to_string()
}

/// Parse 2 dimensional position to 0-based index.
fn parse_pos_2d(column: usize, line: usize) -> [String; 2] {
	[parse_pos(column), parse_pos(line)]
}

pub fn set_position(column: usize, line: usize) {
	csi('H', &parse_pos_2d(column, line));
}

pub fn move_to_column(column: usize) {
	csi('G', &[parse_pos(column)]);
}

pub fn move_right(columns: usize) {
	csi('C', &[columns.to_string()]);
}

pub fn move_left(columns: usize) {
	csi('D', &[columns.to_string()]);
}

pub fn move_up(lines: usize) {
	csi('A', &[lines.to_string()]);
}

pub fn move_down(lines: usize) {
	csi('B', &[lines.to_string()]);
}

pub fn move_by(columns: isize, lines: isize) {
	if columns > 0 {
		move_right(columns as usize);
	} else if columns < 0 {
		move_left(-columns as usize);
	}
	if lines > 0 {
		move_up(lines as usize);
	} else if lines < 0 {
		move_down(-lines as usize);
	}
}

/// To the beginning of the line, `lines` down
pub fn next_line(lines: usize) {
	csi('E', &[lines.to_string()]);
}

/// To the beginning of the line, `lines` up
pub fn previous_line(lines: usize) {
	csi('F', &[lines.to_string()]);
}

// TODO:
// * Add ability to hide/show cursor
// * Add ability to save/restore cursor position
// * Add ability to get cursor position

// `csi` won't work for all codes; some do not require "[", which denotes a CSI
// For example: save/restore cursor position