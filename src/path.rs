/// Returns the stem: the part of the filename before the last period.
/// Returns None if the input is empty.
/// Returns the entire filename if there are no dots.
/// Returns the entire filename if it begins with . and has no other dots within.
/// Otherwise returns the portion before the final dot.
pub fn file_stem(input: &str) -> Option<&str> {
	if input.is_empty() {
		return None;
	}

	// If no dots, return the whole thing
	if !input.contains('.') {
		return Some(input);
	}

	// If starts with . and has only one dot, return the whole thing (hidden file)
	if input.starts_with('.') {
		let dot_count = input.matches('.').count();
		if dot_count == 1 {
			return Some(input);
		}
	}

	// Otherwise, return the part before the last dot
	input.rfind('.').map(|pos| &input[..pos])
}

/// Returns the extension: the part of the filename after the last period,
/// or None if there is no period or if the filename is all extension (starts with . and has one dot).
pub fn file_extension(input: &str) -> Option<&str> {
	if input.is_empty() {
		return None;
	}

	// If starts with . and has only one dot, there's no extension (it's a hidden file)
	if input.starts_with('.') {
		let dot_count = input.matches('.').count();
		if dot_count == 1 {
			return None;
		}
	}

	input.rfind('.').map(|pos| &input[pos + 1..])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn stem_empty() {
		assert_eq!(file_stem(""), None);
	}

	#[test]
	fn stem_no_extension() {
		assert_eq!(file_stem("report"), Some("report"));
		assert_eq!(file_stem("ABC123"), Some("ABC123"));
		assert_eq!(file_stem("my-file-name"), Some("my-file-name"));
	}

	#[test]
	fn stem_with_single_extension() {
		assert_eq!(file_stem("report.txt"), Some("report"));
		assert_eq!(file_stem("archive.tar"), Some("archive"));
		assert_eq!(file_stem("README.md"), Some("README"));
	}

	#[test]
	fn stem_with_multiple_extensions() {
		assert_eq!(file_stem("archive.tar.gz"), Some("archive.tar"));
		assert_eq!(file_stem("file.backup.old"), Some("file.backup"));
		assert_eq!(file_stem("data.json.bak"), Some("data.json"));
	}

	#[test]
	fn stem_hidden_file_no_other_dots() {
		// Hidden files with only the leading dot return the whole name
		assert_eq!(file_stem(".hidden"), Some(".hidden"));
		assert_eq!(file_stem(".bashrc"), Some(".bashrc"));
		assert_eq!(file_stem(".gitignore"), Some(".gitignore"));
	}

	#[test]
	fn stem_hidden_file_with_other_dots() {
		// Hidden files with other dots: return before the last dot
		assert_eq!(file_stem(".hidden.file"), Some(".hidden"));
		assert_eq!(file_stem("..double"), Some("."));
		assert_eq!(file_stem(".backup.old"), Some(".backup"));
		assert_eq!(file_stem("...three"), Some(".."));
	}

	#[test]
	fn stem_edge_cases() {
		assert_eq!(file_stem("."), Some("."));
		assert_eq!(file_stem(".."), Some("."));
		assert_eq!(file_stem("....."), Some("...."));
	}

	#[test]
	fn extension_empty() {
		assert_eq!(file_extension(""), None);
	}

	#[test]
	fn extension_no_extension() {
		assert_eq!(file_extension("report"), None);
		assert_eq!(file_extension("ABC123"), None);
		assert_eq!(file_extension("my-file-name"), None);
	}

	#[test]
	fn extension_with_single_extension() {
		assert_eq!(file_extension("report.txt"), Some("txt"));
		assert_eq!(file_extension("archive.tar"), Some("tar"));
		assert_eq!(file_extension("README.md"), Some("md"));
	}

	#[test]
	fn extension_with_multiple_extensions() {
		assert_eq!(file_extension("archive.tar.gz"), Some("gz"));
		assert_eq!(file_extension("file.backup.old"), Some("old"));
		assert_eq!(file_extension("data.json.bak"), Some("bak"));
	}

	#[test]
	fn extension_hidden_file_no_other_dots() {
		// Hidden files with only the leading dot have no extension
		assert_eq!(file_extension(".hidden"), None);
		assert_eq!(file_extension(".bashrc"), None);
		assert_eq!(file_extension(".gitignore"), None);
	}

	#[test]
	fn extension_hidden_file_with_other_dots() {
		// Hidden files with other dots: return after the last dot
		assert_eq!(file_extension(".hidden.file"), Some("file"));
		assert_eq!(file_extension("..double"), Some("double"));
		assert_eq!(file_extension(".backup.old"), Some("old"));
		assert_eq!(file_extension("...three"), Some("three"));
	}

	#[test]
	fn extension_edge_cases() {
		assert_eq!(file_extension("."), None);
		assert_eq!(file_extension(".."), Some(""));
		assert_eq!(file_extension("....."), Some(""));
	}

	#[test]
	fn stem_and_extension_consistency() {
		let test_names = vec![
			"report.txt",
			"archive.tar.gz",
			"file.backup.old",
			".hidden.file",
			"nodots",
			".hidden",
		];

		for name in test_names {
			let stem = file_stem(name);
			let ext = file_extension(name);

			// Verify that stem + dot + ext == original (when ext exists)
			match (stem, ext) {
				(Some(s), Some(e)) => {
					let reconstructed = format!("{}.{}", s, e);
					assert_eq!(
						reconstructed, name,
						"Reconstruction failed for {}: {} + . + {} = {}",
						name, s, e, reconstructed
					);
				}
				(Some(s), None) => {
					assert_eq!(
						s, name,
						"Without extension, stem should equal original: {}",
						name
					);
				}
				(None, _) => {
					assert_eq!(
						name, "",
						"None stem only valid for empty input: {}",
						name
					);
				}
			}
		}
	}
}
