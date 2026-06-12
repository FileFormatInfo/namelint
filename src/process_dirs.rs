use std::{fs, path::PathBuf};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntryType {
	Dir,
	File,
	Both,
}

pub fn process_dirs<F>(
	mut dirs: Vec<PathBuf>,
	entry_type: EntryType,
	skip_dirs: &[String],
	skip_files: &[String],
	mut meta_rule: F,
) -> Result<usize, String>
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
			let file_name = entry.file_name();
			let name = file_name.to_string_lossy();

			if path.is_dir() {
				if skip_dirs.iter().any(|skip| skip == &name) {
					continue;
				}
				if entry_type == EntryType::Dir || entry_type == EntryType::Both {
					error_count += meta_rule(&name);
				}
				dirs.push(path);
				continue;
			}

			if skip_files.iter().any(|skip| skip == &name) {
				continue;
			}

			if entry_type == EntryType::File || entry_type == EntryType::Both {
				error_count += meta_rule(&name);
			}
		}
	}

	Ok(error_count)
}
