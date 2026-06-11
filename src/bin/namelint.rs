use clap::{arg, Command};
use namelint::collect_cli_dirs::collect_cli_dirs;

fn main() {
    let command = Command::new("namelint")
        .version("1.0")
        .about("Check file names for security, compatibility, best practices & standards.")
        .arg(arg!(-v --verbose "Print resolved directories").required(false))
        .arg(arg!([paths]... "Directories to check (default: current directory)")
            .required(false));

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
            println!("DEBUG: command line directory: {}", dir.display());
        }
    }
}
