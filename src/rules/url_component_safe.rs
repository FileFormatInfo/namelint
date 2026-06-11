use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};

/// Characters safe as a URL path segment or query parameter value without
/// percent-encoding and without ambiguity.  More restrictive than url_safe:
/// only alphanumerics, hyphen and underscore — no dot (path traversal risk)
/// and no tilde (sometimes treated specially by servers).
pub fn url_component_safe(input: &str) -> bool {
	!input.is_empty()
		&& input
			.chars()
			.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub fn url_component_safe_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}
	Some(Box::new(url_component_safe))
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "url_component_safe",
		no_arg: "on",
		missing_value: "on",
		values: &["on", "off"],
		short_description: "Only characters safe as a URL path/query component (A-Za-z0-9-_).",
		long_description_markdown: "Restricts filenames to characters that are unambiguously safe as a URL path segment or query parameter value: ASCII letters, digits, hyphen (`-`), and underscore (`_`).\n\nMore restrictive than `url_safe`: excludes dot (`.`) to avoid path-traversal ambiguity and tilde (`~`) which some servers treat specially.",
		check: url_component_safe_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::url_component_safe;

	#[test]
	fn accepts() {
		assert!(url_component_safe("my-report"));
		assert!(url_component_safe("my_report_2026"));
		assert!(url_component_safe("A1-b2"));
	}

	#[test]
	fn rejects() {
		assert!(!url_component_safe("file.txt"));
		assert!(!url_component_safe("~backup"));
		assert!(!url_component_safe("bad name"));
		assert!(!url_component_safe("bad$name"));
		assert!(!url_component_safe("bad/name"));
		assert!(!url_component_safe(""));
	}
}
