use std::{fs, path::PathBuf};
use unicase::UniCase;

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
	ext_case_insensitive: &[String],
	ext_case_sensitive: &[String],
	ext_unicode_case_insensitive: &[String],
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

			if !extension_matches(
				&path,
				ext_case_insensitive,
				ext_case_sensitive,
				ext_unicode_case_insensitive,
			) {
				continue;
			}

			if entry_type == EntryType::File || entry_type == EntryType::Both {
				error_count += meta_rule(&name);
			}
		}
	}

	Ok(error_count)
}

fn extension_matches(
	path: &PathBuf,
	ext_case_insensitive: &[String],
	ext_case_sensitive: &[String],
	ext_unicode_case_insensitive: &[String],
) -> bool {
	if ext_case_insensitive.is_empty()
		&& ext_case_sensitive.is_empty()
		&& ext_unicode_case_insensitive.is_empty()
	{
		return true;
	}

	let Some(ext) = path.extension().and_then(|value| value.to_str()) else {
		return false;
	};

	ext_case_sensitive.iter().any(|candidate| ext == candidate)
		|| ext_case_insensitive
			.iter()
			.any(|candidate| ext.eq_ignore_ascii_case(candidate))
		|| ext_unicode_case_insensitive
			.iter()
			.any(|candidate| UniCase::new(ext) == UniCase::new(candidate.as_str()))
}
