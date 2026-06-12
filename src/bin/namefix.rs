use std::{collections::VecDeque, ffi::OsStr, fs, path::{Path, PathBuf}};
use clap::{arg, Command};

struct FixStats {
    total_checked: usize,
    total_fixed: usize,
    has_null_byte: usize,
    not_utf8: usize,
}

fn main() {
    let command = Command::new("namefix")
        .version(option_env!("VERSION").unwrap_or("(unknown)"))
        .about("Fix file names containing null bytes or invalid UTF-8")
        .arg(arg!(--"dry-run" "Report count without making changes").required(false))
        .arg(arg!(-v --verbose "Print fixed file names").required(false))
        .arg(arg!(<paths>... "Directories to scan").required(true));

    let matches = command.get_matches();
    let dry_run = matches.get_flag("dry-run");
    let verbose = matches.get_flag("verbose");
    let paths: Vec<&String> = matches.get_many::<String>("paths").unwrap().collect();

    let mut stats = FixStats {
        total_checked: 0,
        total_fixed: 0,
        has_null_byte: 0,
        not_utf8: 0,
    };

    for path_str in paths {
        walk_and_fix(path_str, dry_run, verbose, &mut stats);
    }

    println!("INFO: Count with null byte: {}", stats.has_null_byte);
    println!("INFO: Count with invalid UTF-8: {}", stats.not_utf8);
    println!("INFO: Total names checked: {}", stats.total_checked);
    println!("INFO: Total names fixed: {}", stats.total_fixed);
}

/// Recursively walk directories and fix problematic file/dir names.
fn walk_and_fix(start_path: &str, dry_run: bool, verbose: bool, stats: &mut FixStats) {
    let path = Path::new(start_path);
    if !path.exists() {
        eprintln!("ERROR: path does not exist: '{}'", start_path);
        return;
    }
    if !path.is_dir() {
        eprintln!("ERROR: path is not a directory: '{}'", start_path);
        return;
    }

    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(path.to_path_buf());

    while let Some(current_dir) = queue.pop_front() {
        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            let file_name_os = entry.file_name();

                            stats.total_checked += 1;

                            if let Some((new_name, issues)) = check_and_fix_name(&file_name_os) {
                                stats.has_null_byte += if issues.has_null { 1 } else { 0 };
                                stats.not_utf8 += if issues.has_non_utf8 { 1 } else { 0 };

                                let current_name = file_name_os.to_string_lossy().to_string();
                                if dry_run {
                                    stats.total_fixed += 1;
                                    if verbose {
                                        println!("Would fix: {} -> {}", current_name, new_name);
                                    }
                                } else {
                                    match apply_fix(&entry_path, &new_name) {
                                        Ok(actual_path) => {
                                            stats.total_fixed += 1;
                                            if verbose {
                                                println!("INFO: Fixed: {} -> {}", current_name, new_name);
                                            }
                                            // For recursion, use the new path if it's a directory
                                            if actual_path.is_dir() {
                                                queue.push_back(actual_path);
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("ERROR: Failed to fix {}: {}", current_name, e);
                                        }
                                    }
                                }
                            } else if entry_path.is_dir() {
                                queue.push_back(entry_path);
                            }
                        }
                        Err(e) => {
                            eprintln!("ERROR: Failed to read directory entry: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR: Failed to read directory {}: {}", current_dir.display(), e);
            }
        }
    }
}

struct Issues {
    has_null: bool,
    has_non_utf8: bool,
}

/// Check if a filename needs fixing and return the new name if so.
fn check_and_fix_name(file_name_os: &OsStr) -> Option<(String, Issues)> {
    let mut issues = Issues {
        has_null: false,
        has_non_utf8: false,
    };

    // Check if it's valid UTF-8
    let is_valid_utf8 = file_name_os.to_str().is_some();
    if !is_valid_utf8 {
        issues.has_non_utf8 = true;
    }

    // Convert to a lossy string representation
    let current_name = file_name_os.to_string_lossy();

    // Check for null bytes (by checking the bytes if available)
    if let Some(_bytes) = current_name.as_bytes().windows(1).find(|b| b[0] == 0) {
        issues.has_null = true;
    }

    if !issues.has_null && !issues.has_non_utf8 {
        return None;
    }

    // Replace problematic characters
    let fixed = current_name
        .replace('\0', "0")
        .replace(|c: char| !c.is_ascii_graphic() && c != ' ', "X");

    if fixed != current_name.as_ref() {
        Some((fixed, issues))
    } else {
        None
    }
}

/// Apply the fix: rename the file/dir, handling conflicts with numeric suffixes.
fn apply_fix(original_path: &Path, new_name: &str) -> Result<PathBuf, String> {
    let parent = original_path.parent().ok_or("No parent directory".to_string())?;
    let mut target_path = parent.join(new_name);

    // If target already exists, add numeric suffix
    if target_path.exists() {
        let (name, ext) = split_name_ext(new_name);
        let mut suffix = 1;
        loop {
            let candidate = if ext.is_empty() {
                format!("{}_{}", name, suffix)
            } else {
                format!("{}_{}_{}", name, suffix, ext)
            };
            target_path = parent.join(&candidate);
            if !target_path.exists() {
                break;
            }
            suffix += 1;
        }
    }

    fs::rename(original_path, &target_path)
        .map_err(|e| format!("fs::rename failed: {}", e))?;

    Ok(target_path)
}

/// Split filename into name and extension.
fn split_name_ext(file_name: &str) -> (String, String) {
    if let Some(dot_pos) = file_name.rfind('.') {
        if dot_pos > 0 && dot_pos < file_name.len() - 1 {
            let name = file_name[..dot_pos].to_string();
            let ext = file_name[dot_pos + 1..].to_string();
            return (name, ext);
        }
    }
    (file_name.to_string(), String::new())
}
