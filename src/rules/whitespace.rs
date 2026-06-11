use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};

pub fn whitespace_none(input: &str) -> bool {
	!input.chars().any(char::is_whitespace)
}

pub fn whitespace_space(input: &str) -> bool {
	if input.trim() != input {
		return false;
	}

	let mut prev_was_space = false;
	for c in input.chars() {
		if c.is_whitespace() {
			if c != ' ' {
				return false;
			}
			if prev_was_space {
				return false;
			}
			prev_was_space = true;
		} else {
			prev_was_space = false;
		}
	}

	true
}

pub fn whitespace_spaces(input: &str) -> bool {
	if input.trim() != input {
		return false;
	}

	for c in input.chars() {
		if c.is_whitespace() && c != ' ' {
			return false;
		}
	}

	true
}

pub fn whitespace_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}

	let mode = param
		.unwrap_or_else(|| "none".to_string())
		.trim()
		.to_ascii_lowercase();

	let checker: RuleCheckFn = match mode.as_str() {
		"none" => Box::new(whitespace_none),
		"space" => Box::new(whitespace_space),
		"spaces" => Box::new(whitespace_spaces),
		"any" => Box::new(|_| true),
		_ => Box::new(whitespace_none),
	};

	Some(checker)
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "whitespace",
		no_arg: "none",
		missing_value: "none",
		values: &["none", "space", "spaces", "any"],
		short_description: "Control how internal whitespace is allowed in filenames.",
		long_description_markdown: "Controls internal whitespace policy for filenames.\n\n- `none`: no internal whitespace allowed\n- `space`: allow single ASCII spaces only\n- `spaces`: allow one or more ASCII spaces\n- `any`: allow any internal whitespace",
		check: whitespace_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::{whitespace_none, whitespace_space, whitespace_spaces};

	#[test]
	fn none_accepts() {
		assert!(whitespace_none("clean-name.txt"));
		assert!(whitespace_none("clean_name"));
	}

	#[test]
	fn none_rejects() {
		assert!(!whitespace_none("clean name.txt"));
		assert!(!whitespace_none("clean\tname.txt"));
	}

	#[test]
	fn space_accepts() {
		assert!(whitespace_space("clean name.txt"));
		assert!(whitespace_space("clean name file.txt"));
		assert!(whitespace_space("clean-name.txt"));
	}

	#[test]
	fn space_rejects() {
		assert!(!whitespace_space("clean  name.txt"));
		assert!(!whitespace_space(" clean name.txt"));
		assert!(!whitespace_space("clean\tname.txt"));
	}

	#[test]
	fn spaces_accepts() {
		assert!(whitespace_spaces("clean name.txt"));
		assert!(whitespace_spaces("clean  name.txt"));
		assert!(whitespace_spaces("clean-name.txt"));
	}

	#[test]
	fn spaces_rejects() {
		assert!(!whitespace_spaces("clean\tname.txt"));
		assert!(!whitespace_spaces(" clean name.txt"));
		assert!(!whitespace_spaces("clean name.txt "));
	}
}
