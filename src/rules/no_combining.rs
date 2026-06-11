
use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};
use unicode_normalization::char::is_combining_mark;

pub fn no_combining(input: &str) -> bool {
	for c in input.chars() {
		if is_combining_mark(c) {
			return false;
		}
	}
	return true;
}

pub fn no_combining_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}
	Some(Box::new(no_combining))
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "no_combining",
		no_arg: "on",
		missing_value: "on",
		values: &["on", "off"],
		short_description: "Filename must not contain combining marks.",
		long_description_markdown: "Rejects filenames that include Unicode combining marks.\n\nCombining marks can make names hard to compare and reason about across terminals, shells, and filesystems.",
		check: no_combining_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::no_combining;

	#[test]
	fn accepts() {
		assert!(no_combining("normal-file.txt"));
	}

	#[test]
	fn rejects() {
		let with_combining = "n\u{0303}.txt";
		assert!(!no_combining(with_combining));
	}
}
