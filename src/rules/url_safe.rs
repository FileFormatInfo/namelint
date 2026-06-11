use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};

/// RFC 3986 unreserved characters: A-Za-z0-9 - . _ ~
/// These are safe anywhere in a URL without percent-encoding.
/// Reference: https://datatracker.ietf.org/doc/html/rfc3986#section-2.3
pub fn url_safe(input: &str) -> bool {
	!input.is_empty()
		&& input
			.chars()
			.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~')
}

pub fn url_safe_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}
	Some(Box::new(url_safe))
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "url_safe",
		no_arg: "off",
		missing_value: "on",
		values: &["on", "off"],
		short_description: "Only RFC 3986 unreserved characters (A-Za-z0-9-._~).",
		long_description_markdown: "Restricts filenames to the [RFC 3986 unreserved characters](https://datatracker.ietf.org/doc/html/rfc3986#section-2.3): ASCII letters, digits, hyphen (`-`), dot (`.`), underscore (`_`), and tilde (`~`).\n\nThese characters are safe anywhere in a URL without percent-encoding.",
		check: url_safe_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::url_safe;

	#[test]
	fn accepts() {
		assert!(url_safe("file.txt"));
		assert!(url_safe("my-report_2026.log"));
		assert!(url_safe("~backup"));
		assert!(url_safe("A1-b_2.c"));
	}

	#[test]
	fn rejects() {
		assert!(!url_safe("bad name.txt"));
		assert!(!url_safe("bad$name.txt"));
		assert!(!url_safe("bad#name.txt"));
		assert!(!url_safe("bad?name.txt"));
		assert!(!url_safe("bad/name.txt"));
		assert!(!url_safe(""));
	}
}
