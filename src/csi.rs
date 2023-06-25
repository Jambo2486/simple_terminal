pub(crate) const ESCAPE_CHAR: &str = "\x1B";

/// Control sequence introducer generator
pub(crate) fn csi(command: char, values: &[String]) -> String {
	format!("{}[{}{}", ESCAPE_CHAR, values.join(";"), command)
}

#[test]
fn test_csi_fn() {
	assert_eq!(
		csi('A', &["1".to_string(), "2".to_string(), "three".to_string()]),
		"\x1B[1;2;threeA".to_string()
	)
}

#[macro_export]
macro_rules! known_csi {
    ($command:expr $(, $arg:expr)*) => {
        format!("{}[{}{}", crate::csi::ESCAPE_CHAR, [$($arg),*].join(";"), $command)
    };
}

#[test]
fn test_csi_macro() {
	assert_eq!(
		known_csi!('A', "1", "2", "three"),
		"\x1B[1;2;threeA".to_string()
	)
}
