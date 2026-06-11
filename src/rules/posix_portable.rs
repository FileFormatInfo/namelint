use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};

/// Accepts only characters in the POSIX Portable Filename Character Set:
/// A-Z  a-z  0-9  .  _  -
/// Reference: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_265
pub fn posix_portable(input: &str) -> bool {
	!input.is_empty()
		&& input
			.chars()
			.all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
}

pub fn posix_portable_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}
	Some(Box::new(posix_portable))
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "posix_portable",
		no_arg: "on",
		missing_value: "on",
		values: &["on", "off"],
		short_description: "Only POSIX portable filename characters (A-Za-z0-9._-).",
		long_description_markdown: "Restricts filenames to the [POSIX Portable Filename Character Set](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_265): ASCII letters, digits, dot (`.`), underscore (`_`), and dash (`-`).",
		check: posix_portable_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::posix_portable;

	#[test]
	fn accepts() {
		assert!(posix_portable("test"));
		assert!(posix_portable("test.txt"));
		assert!(posix_portable("my_file-2026.log"));
		assert!(posix_portable("A1_b-2.c"));
	}

	#[test]
	fn rejects() {
		assert!(!posix_portable("test.txt~"));
		assert!(!posix_portable("test.txt#"));
		assert!(!posix_portable("test.txt@"));
		assert!(!posix_portable("test.txt!"));
		assert!(!posix_portable("test.txt$"));
		assert!(!posix_portable("test.txt%"));
		assert!(!posix_portable("bad name.txt"));
		assert!(!posix_portable(""));
	}
}
