use crate::rules::{file_extension, is_rule_disabled_param, RuleCheckFn, RuleSpec};

/// Only the first segment (before the first dot) must be ASCII alphanumeric.
/// Any extensions are allowed but not checked.
pub fn ascii_alphanumeric_base(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}
	let first_segment = input.split('.').next().unwrap_or("");
	!first_segment.is_empty() && first_segment.chars().all(|c| c.is_ascii_alphanumeric())
}

/// ASCII alphanumeric first segment plus exactly one extension.
/// No period is also fine.
pub fn ascii_alphanumeric_on(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}
	// Check that there is at most one dot in the entire name
	if let (Some(first), Some(last)) = (input.find('.'), input.rfind('.')) {
		if first != last {
			return false; // multiple dots, not allowed
		}
	}
	let first_segment = input.split('.').next().unwrap_or("");
	if first_segment.is_empty() || !first_segment.chars().all(|c| c.is_ascii_alphanumeric()) {
		return false;
	}
	match file_extension(input) {
		None => true,
		Some(ext) => {
			!ext.is_empty()
				&& ext.chars().all(|c| c.is_ascii_alphanumeric())
		}
	}
}

/// All dot-separated segments must be ASCII alphanumeric.
pub fn ascii_alphanumeric_ext(input: &str) -> bool {
	if input.is_empty() {
		return false;
	}
	input.split('.').all(|seg| !seg.is_empty() && seg.chars().all(|c| c.is_ascii_alphanumeric()))
}

pub fn ascii_alphanumeric_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}

	let mode = param
		.unwrap_or_else(|| "on".to_string())
		.trim()
		.to_ascii_lowercase();

	let checker: RuleCheckFn = match mode.as_str() {
		"base" => Box::new(ascii_alphanumeric_base),
		"ext" => Box::new(ascii_alphanumeric_ext),
		"on" => Box::new(ascii_alphanumeric_on),
		_ => return None,
	};

	Some(checker)
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "ascii_alphanumeric",
		no_arg: "ext",
		missing_value: "on",
		values: &["off", "base", "ext", "on"],
		short_description: "Only ASCII alphanumeric characters, with optional dot separator.",
		long_description_markdown: "Restricts filenames to ASCII letters and digits.\n\n- `base`: no dots — alphanumerics only in the full name\n- `on`: allows a single dot separating an alphanumeric stem from an alphanumeric extension\n- `ext`: allows multiple dot-separated alphanumeric segments",
		check: ascii_alphanumeric_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::{ascii_alphanumeric_base, ascii_alphanumeric_ext, ascii_alphanumeric_on};
	use crate::rules::{file_extension, file_stem};

	#[test]
	fn helpers() {
		assert_eq!(file_stem("report.txt"), Some("report"));
		assert_eq!(file_stem("report"), Some("report"));
		assert_eq!(file_stem("archive.tar.gz"), Some("archive.tar"));
		assert_eq!(file_stem(".txt"), Some(".txt"));

		assert_eq!(file_extension("report.txt"), Some("txt"));
		assert_eq!(file_extension("archive.tar.gz"), Some("gz"));
		assert_eq!(file_extension("report"), None);
		assert_eq!(file_extension(".txt"), None);
	}

	#[test]
	fn base_accepts() {
		assert!(ascii_alphanumeric_base("report"));
		assert!(ascii_alphanumeric_base("ABC123"));
		assert!(ascii_alphanumeric_base("ABC123.log"));
		assert!(ascii_alphanumeric_base("myfile.$$$"));
		assert!(ascii_alphanumeric_base("report.tar.gz"));
	}

	#[test]
	fn base_rejects() {
		assert!(!ascii_alphanumeric_base("my-file.txt"));
		assert!(!ascii_alphanumeric_base(".txt"));
		assert!(!ascii_alphanumeric_base("bad$.txt"));
		assert!(!ascii_alphanumeric_base(""));
	}

	#[test]
	fn on_accepts() {
		assert!(ascii_alphanumeric_on("report.txt"));
		assert!(ascii_alphanumeric_on("report"));
		assert!(ascii_alphanumeric_on("ABC123.log"));
	}

	#[test]
	fn on_rejects() {
		assert!(!ascii_alphanumeric_on("report.tar.gz"));
		assert!(!ascii_alphanumeric_on("my-file.txt"));
		assert!(!ascii_alphanumeric_on("report."));
		assert!(!ascii_alphanumeric_on(".txt"));
		assert!(!ascii_alphanumeric_on(""));
	}

	#[test]
	fn ext_accepts() {
		assert!(ascii_alphanumeric_ext("report.txt"));
		assert!(ascii_alphanumeric_ext("archive.tar.gz"));
		assert!(ascii_alphanumeric_ext("report"));
	}

	#[test]
	fn ext_rejects() {
		assert!(!ascii_alphanumeric_ext("my-file.txt"));
		assert!(!ascii_alphanumeric_ext("report."));
		assert!(!ascii_alphanumeric_ext(".txt"));
		assert!(!ascii_alphanumeric_ext(""));
	}
}
