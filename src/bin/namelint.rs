use clap::{Arg, Command};
use namelint::collect_cli_dirs::collect_cli_dirs;
use namelint::process_dirs::process_dirs;
use namelint::rules::{builtin_rules, RuleCheckFn};
use serde_json::json;

/// An active checker: a rule slug paired with its compiled check function.
struct ActiveRule {
    slug: &'static str,
    check: RuleCheckFn,
}

fn emit_failure(filename: &str, rule_slug: &str, output_format: &str) {
    if output_format == "json" {
        println!("{}", json!({
            "filename": filename,
            "rule": rule_slug,
        }));
    } else {
        eprintln!("{}: {}", filename, rule_slug);
    }
}

fn main() {
    let rules = builtin_rules();

    let mut command = Command::new("namelint")
        .about("Check file names for security, compatibility, best practices & standards.")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Print resolved directories")
                .required(false),
        )
        .arg(
            Arg::new("paths")
                .action(clap::ArgAction::Append)
                .help("Directories to check (default: current directory)")
                .required(false),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("FORMAT")
                .value_parser(["plain", "json"])
                .default_value("plain")
                .help("Output format for failures: plain or json")
                .required(false),
		)
		// disable the built-in version flag; we handle --version manually
		.disable_version_flag(true)
		.arg(
			Arg::new("version")
				.long("version")
				.action(clap::ArgAction::SetTrue)
				.help("Print version")
			)
		;

    for rule in &rules {
        let values_hint = rule.values.join("|");
        command = command.arg(
            Arg::new(rule.slug)
                .long(&rule.slug.replace('_', "-"))
                .value_name("VALUE")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value(rule.missing_value)
                .help(format!("{} [{}]", rule.short_description, values_hint))
                .required(false),
        );
    }

    let matches = command.get_matches();

    if matches.get_flag("version") {
        if matches.get_flag("verbose") {
			println!("PROGRAM : {}", env!("CARGO_PKG_NAME"));
            println!("VERSION : {}", option_env!("VERSION").unwrap_or("(unknown)"));
            println!("COMMIT  : {}", option_env!("COMMIT").unwrap_or("(unknown)"));
            println!("LASTMOD : {}", option_env!("LASTMOD").unwrap_or("(unknown)"));
            println!("BUILTBY : {}", option_env!("BUILTBY").unwrap_or("(unknown)"));
        } else {
	        println!("{} {}", env!("CARGO_PKG_NAME"), option_env!("VERSION").unwrap_or("(unknown)"));
		}
        std::process::exit(0);
    }

    let raw_paths = matches
        .get_many::<String>("paths")
        .map(|vals| vals.cloned().collect::<Vec<String>>());

    let dirs = collect_cli_dirs(raw_paths).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    });

	let output_format = matches
		.get_one::<String>("output")
		.map(|value| value.as_str())
		.unwrap_or("plain");

    if matches.get_flag("verbose") {
        for dir in &dirs {
            println!("DEBUG: directory on command line: {}", dir.display());
        }
        for rule in &rules {
            let param = matches.get_one::<String>(rule.slug).cloned();
            let effective = param.as_deref().unwrap_or(rule.no_arg);
            println!("DEBUG: rule {} = {}", rule.slug, effective);
        }
    }

    let active: Vec<ActiveRule> = rules
        .iter()
        .filter_map(|rule| {
            let param = matches
                .get_one::<String>(rule.slug)
                .cloned()
                .or_else(|| Some(rule.no_arg.to_string()));
            let check = (rule.check)(param)?;
            Some(ActiveRule { slug: rule.slug, check })
        })
        .collect();

    let error_count = process_dirs(dirs, |name| {
        let mut file_error_count = 0usize;

        for active_rule in &active {
            if !(active_rule.check)(name) {
                emit_failure(name, active_rule.slug, output_format);
                file_error_count += 1;
            }
        }

        file_error_count
    })
    .unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    });

	if matches.get_flag("verbose") {
		println!("INFO: Total errors found: {}", error_count);
	}

    if error_count > 0 {
        std::process::exit(1);
    }
}

