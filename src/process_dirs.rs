use std::{fs, path::PathBuf};

pub fn process_dirs<F>(mut dirs: Vec<PathBuf>, mut meta_rule: F) -> Result<usize, String>
where
	F: FnMut(&str) -> usize,
{
	let mut error_count = 0usize;

	while let Some(dir) = dirs.pop() {
		let entries = fs::read_dir(&dir)
			.map_err(|e| format!("cannot read directory {}: {}", dir.display(), e))?;

		for entry in entries {
			let entry = entry.map_err(|e| format!("cannot read directory entry in {}: {}", dir.display(), e))?;
			let path = entry.path();

			if path.is_dir() {
				dirs.push(path);
				continue;
			}

			let file_name = entry.file_name();
			let name = file_name.to_string_lossy();
			error_count += meta_rule(&name);
		}
	}

	Ok(error_count)
}
