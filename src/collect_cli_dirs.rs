use std::path::PathBuf;

pub fn collect_cli_dirs(raw_paths: Option<Vec<String>>) -> Result<Vec<PathBuf>, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Cannot get current directory: {}", e))?;

    match raw_paths {
        None => Ok(vec![current_dir]),
        Some(paths) => {
            if paths.is_empty() {
                return Ok(vec![current_dir]);
            }

            let mut dirs = Vec::new();

            for s in paths {
                let path = if s == "." {
                    current_dir.clone()
                } else {
                    PathBuf::from(s)
                };

                if !path.exists() {
                    return Err(format!("path does not exist: '{}'", path.display()));
                }
                if !path.is_dir() {
                    return Err(format!("path is not a directory: '{}'", path.display()));
                }

                dirs.push(path);
            }

            Ok(dirs)
        }
    }
}
