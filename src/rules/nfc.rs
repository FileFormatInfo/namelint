use crate::rules::{is_rule_disabled_param, RuleCheckFn, RuleSpec};
use unicode_normalization::is_nfc;

pub fn nfc(input: &str) -> bool {

	return is_nfc(input);
}

pub fn nfc_builder(param: Option<String>) -> Option<RuleCheckFn> {
	if is_rule_disabled_param(param.as_deref()) {
		return None;
	}

	Some(Box::new(nfc))
}

pub fn rule() -> RuleSpec {
	RuleSpec {
		slug: "nfc",
		no_arg: "on",
		missing_value: "on",
		values: &["on", "off"],
		short_description: "Filename must be NFC-normalized Unicode.",
		long_description_markdown: "Ensures the filename is in Unicode NFC form.\n\nThis helps avoid visually identical names being stored with different byte representations across filesystems and tools.",
		check: nfc_builder,
	}
}

#[cfg(test)]
mod tests {
	use super::nfc;

	#[test]
	fn accepts() {
		assert!(nfc("report.txt"));
	}

	#[test]
	fn rejects() {
		let nfd = "e\u{301}.txt";
		assert!(!nfc(nfd));
	}
}
