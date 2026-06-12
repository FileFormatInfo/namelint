use std::collections::HashMap;


pub mod ascii_an;
pub mod ascii_and;
pub mod nfc;
pub mod no_combining;
pub mod trimmed;
pub mod posix_portable;
pub mod url_component_safe;
pub mod url_safe;
pub mod whitespace;

pub type RuleCheckFn = Box<dyn Fn(&str) -> bool>;
pub type RuleCheckBuilderFn = fn(Option<String>) -> Option<RuleCheckFn>;

#[derive(Clone, Copy, Debug)]
pub struct RuleSpec {
	pub slug: &'static str,
	pub no_arg: &'static str,				// The value to use if the parameter was not specified at all
	pub missing_value: &'static str,		// The value to use if the rule is enabled but the parameter is explicitly missing (e.g. --slug=value without =value).
	pub values: &'static [&'static str],
	pub short_description: &'static str,
	pub long_description_markdown: &'static str,
	pub check: RuleCheckBuilderFn,
}

impl RuleSpec {
	pub fn run(&self, value: &str, param: Option<String>) -> Option<bool> {
		let checker = (self.check)(param)?;
		Some(checker(value))
	}
}

pub fn builtin_rules() -> Vec<RuleSpec> {
	vec![nfc::rule(), no_combining::rule(), trimmed::rule(), whitespace::rule(), posix_portable::rule(), url_safe::rule(), url_component_safe::rule(), ascii_an::rule(), ascii_and::rule()]
}

pub fn builtin_rules_by_slug() -> HashMap<&'static str, RuleSpec> {
	let mut rules = HashMap::new();
	for rule in builtin_rules() {
		rules.insert(rule.slug, rule);
	}
	rules
}

pub fn is_rule_disabled_param(param: Option<&str>) -> bool {
	let Some(raw) = param else {
		return false;
	};

	let normalized = raw.trim().to_ascii_lowercase();
	normalized == "off"
		|| normalized == "false"
		|| normalized == "0"
		|| normalized == "disable"
		|| normalized == "disabled"
}
