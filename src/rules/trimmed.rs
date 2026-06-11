use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};
use crate::path::file_extension;

/// Check if the stem (first segment) is trimmed: no leading/trailing whitespace.
pub fn trimmed_base(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}

	let first_segment = input.split('.').next().unwrap_or("");
	first_segment.trim() == first_segment
}

/// Check if the extension (last segment) is trimmed: no leading/trailing whitespace.
pub fn trimmed_ext(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}

	match file_extension(input) {
		None => true, // No extension, passes
		Some(ext) => ext.trim() == ext,
	}
}

/// Check if both stem and extension are trimmed.
pub fn trimmed_both(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}

	// Check entire name is trimmed
	if input.trim() != input {
		return false;
	}

	// Check extension is trimmed
	if let Some(ext) = file_extension(input) {
		if ext.chars().any(char::is_whitespace) {
			return false;
		}
	}

	// Check no whitespace immediately before any period
	let mut chars = input.chars();
	let mut prev = chars.next();

	for c in chars {
		if c == '.' && prev.is_some_and(char::is_whitespace) {
			return false;
		}
		prev = Some(c);
	}

	true
}

pub fn trimmed_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}

	let mode = param
		.unwrap_or_else(|| "both".to_string())
		.trim()
		.to_ascii_lowercase();

	let checker: RuleCheckFn = match mode.as_str() {
		"base" => Box::new(trimmed_base),
		"ext" => Box::new(trimmed_ext),
		"both" => Box::new(trimmed_both),
		_ => return None,
	};

	Some(checker)
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "trimmed",
		no_arg: "both",
		missing_value: "both",
		values: &["base", "ext", "both", "off"],
		short_description: "No leading or trailing whitespace.",
		long_description_markdown: "Ensures filename components are properly trimmed.\n\n- **base**: Stem (first dot-separated segment) must be trimmed (no leading/trailing whitespace)\n- **ext**: Extension (last dot-separated segment) must be trimmed\n- **both**: Both stem and extension must be trimmed, and no whitespace immediately before periods",
		check: trimmed_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::{trimmed_base, trimmed_ext, trimmed_both};

	#[test]
	fn base_accepts() {
		assert!(trimmed_base("clean.name.txt"));
		assert!(trimmed_base("clean name.txt"));
		assert!(trimmed_base("clean name txt"));
		assert!(trimmed_base("report.txt"));
		assert!(trimmed_base("name is okay... but test anyway.txt"));
	}

	#[test]
	fn base_rejects() {
		assert!(!trimmed_base(" leading.txt"));
		assert!(!trimmed_base("trailing .txt"));
		assert!(!trimmed_base(""));
	}

	#[test]
	fn ext_accepts() {
		assert!(trimmed_ext("clean.txt"));
		assert!(trimmed_ext("clean.name.txt"));
		assert!(trimmed_ext("no_extension"));
		assert!(trimmed_ext("clean name.txt"));
	}

	#[test]
	fn ext_rejects() {
		assert!(!trimmed_ext("file. txt")); // Space in extension
		assert!(!trimmed_ext(""));
	}

	#[test]
	fn both_accepts() {
		assert!(trimmed_both("clean.name.txt"));
		assert!(trimmed_both("clean name.txt"));
		assert!(trimmed_both("clean name txt"));
		assert!(trimmed_both("clean name. passing.txt"));
	}

	#[test]
	fn both_rejects() {
		assert!(!trimmed_both(" bad.txt"));
		assert!(!trimmed_both("bad.txt "));
		assert!(!trimmed_both("bad .txt"));
		assert!(!trimmed_both("clean name. txt"));
		assert!(!trimmed_both(""));
	}
}
