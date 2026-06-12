use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};
use crate::path::{file_extension, file_stem};

fn _ascii_and(input: Option<&str>) -> bool {
	let input = match input {
		Some(s) => s,
		None => return true,
	};
	input.is_empty() || input.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Only the first segment (before the first dot) must be ASCII alphanumeric.
/// Any extensions are allowed but not checked.
pub fn ascii_and_base(input: &str) -> bool {
	return _ascii_and(file_stem(input));
}

/// ASCII alphanumeric first segment plus exactly one extension.
/// No period is also fine.
pub fn ascii_and_both(input: &str) -> bool {
	return _ascii_and(file_stem(input)) && _ascii_and(file_extension(input));
}

/// All dot-separated segments must be ASCII alphanumeric.
pub fn ascii_and_ext(input: &str) -> bool {
	return _ascii_and(file_extension(input));
}

pub fn ascii_and_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}

	let mode = param
		.unwrap_or_else(|| "both".to_string())
		.trim()
		.to_ascii_lowercase();

	let checker: RuleCheckFn = match mode.as_str() {
		"base" => Box::new(ascii_and_base),
		"ext" => Box::new(ascii_and_ext),
		"both" => Box::new(ascii_and_both),
		_ => return None,
	};

	Some(checker)
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "ascii_and",
		no_arg: "ext",
		missing_value: "both",
		values: &["off", "base", "ext", "both"],
		short_description: "Only ASCII alphanumeric and dash characters",
		long_description_markdown: "Restricts filenames to ASCII letters, digits, and dashes.\n\n- `base`: no dots — alphanumerics only in the full name\n- `both`: allows a single dot separating an alphanumeric stem from an alphanumeric extension\n- `ext`: allows multiple dot-separated alphanumeric segments",
		check: ascii_and_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::{ascii_and_base, ascii_and_both, ascii_and_ext};

	#[test]
	fn base_accepts() {
		assert!(ascii_and_base("report"));
		assert!(ascii_and_base("ABC123"));
		assert!(ascii_and_base("ABC123.log"));
		assert!(ascii_and_base("myfile.$$$"));
		assert!(ascii_and_base("report."));		// not desireable, but catch in a separate rule
		assert!(ascii_and_base(""));
	}

	#[test]
	fn base_rejects() {
		assert!(!ascii_and_base("my-file.txt"));
		assert!(!ascii_and_base(".txt"));
		assert!(!ascii_and_base("bad$.txt"));
		assert!(!ascii_and_base("report.tar.gz"));
	}

	#[test]
	fn both_accepts() {
		assert!(ascii_and_both("report.txt"));
		assert!(ascii_and_both("report"));
		assert!(ascii_and_both("ABC123.log"));
		assert!(ascii_and_both("report."));		// not desireable, but catch in a separate rule
		assert!(ascii_and_both(""));
	}

	#[test]
	fn both_rejects() {
		assert!(!ascii_and_both("report.tar.gz"));
		assert!(!ascii_and_both("my-file.txt"));
		assert!(!ascii_and_both(".txt"));
	}

	#[test]
	fn ext_accepts() {
		assert!(ascii_and_ext("report.txt"));
		assert!(ascii_and_ext("report.tar.gz"));
		assert!(ascii_and_ext("ABC123.log"));
		assert!(ascii_and_ext(""));
	}

	#[test]
	fn ext_rejects() {
		assert!(!ascii_and_ext(".txt"));
		assert!(!ascii_and_ext("report.tar.gz."));
		assert!(!ascii_and_ext("my-file.txt"));
	}
}
