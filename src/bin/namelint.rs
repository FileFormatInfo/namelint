use clap::{Arg, Command};
use namelint::collect_cli_dirs::collect_cli_dirs;
use namelint::rules::{builtin_rules, RuleCheckFn};

/// An active checker: a rule slug paired with its compiled check function.
struct ActiveRule {
    slug: &'static str,
    check: RuleCheckFn,
}

fn main() {
    let rules = builtin_rules();

    let mut command = Command::new("namelint")
        .version("1.0")
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
        );

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

    let raw_paths = matches
        .get_many::<String>("paths")
        .map(|vals| vals.cloned().collect::<Vec<String>>());

    let dirs = collect_cli_dirs(raw_paths).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    });

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

    // Build active rules: rules whose builder returns Some(checker)
    let active: Vec<ActiveRule> = rules
        .iter()
        .filter_map(|rule| {
            let param = matches.get_one::<String>(rule.slug).cloned()
                .or_else(|| Some(rule.no_arg.to_string()));
            let check = (rule.check)(param)?;
            Some(ActiveRule { slug: rule.slug, check })
        })
        .collect();

	let mut any_failure = false;

    for dir in &dirs {
        let entries = std::fs::read_dir(dir).unwrap_or_else(|e| {
            eprintln!("ERROR: cannot read directory {}: {}", dir.display(), e);
            std::process::exit(1);
        });

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            for active_rule in &active {
                if !(active_rule.check)(&name) {
                    eprintln!("{}: {}", name, active_rule.slug);
                    any_failure = true;
                }
            }
        }
    }

    if any_failure {
        std::process::exit(1);
    }
}

