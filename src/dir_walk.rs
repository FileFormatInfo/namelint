
use std::{collections::{HashSet, VecDeque}, ffi::OsString, fs, path::{Path, PathBuf}};

use crate::structs::FileData;

pub fn dir_walk(passed_dir: String, ignore_dirs: &HashSet<OsString>, files: &mut Vec<FileData>) -> bool {

	let mut dir = passed_dir;

	if dir == "." {
		println!("DEBUG: using current directory");
		let current_dir = std::env::current_dir();
		if current_dir.is_err() {
			println!("ERROR: Unable to get current directory: {}", current_dir.err().unwrap());
			return false;
		}
		let current_dir = current_dir.unwrap();
		let current_dir = current_dir.to_str();
		if current_dir.is_none() {
			println!("ERROR: Unable to convert current directory to string");
			return false;
		}
		dir = current_dir.unwrap().to_string();
	} else if dir == "" {
		println!("WARNING: skipping empty directory name");
		return true;
	}
	let path = Path::new(&dir);
	if path.exists() == false {
		println!("ERROR: path does not exist: '{}'", dir);
		return false;
	}
	if path.is_dir() == false {
		//DISCUSS: maybe a warning and process it?
		println!("ERROR: path is not a directory: '{}'", dir);
		return false;
	}

	let cpath = path.canonicalize();
	if cpath.is_err() {
		println!("ERROR: Unable to canonicalize directory {}: {}", dir, cpath.err().unwrap());
		return false;
	}
	let cpath = cpath.unwrap();
	if cpath != path {
		println!("WARNING: canonicalized path is different: '{}'", cpath.to_string_lossy());
	}
	let cpath_str = cpath.to_str();
	if cpath_str.is_none() {
		println!("ERROR: canonical directory for {} is not valid UTF-8: '{}'", dir, cpath.to_string_lossy());
		return false;
	}
	let cpath_len = cpath_str.unwrap().len() + 1;   // +1 for the trailing slash

	let mut path_queue: VecDeque<PathBuf> = VecDeque::new();
	path_queue.push_back(path.to_path_buf());

	while path_queue.len() > 0 {
		let next_path = path_queue.pop_front();
		if next_path.is_none() {
			break;
		}
		let next_path = next_path.unwrap();

		let new_paths = visit_dir(cpath_len, &next_path, ignore_dirs, files);

		for new_path in new_paths.iter() {
			path_queue.push_back(new_path.clone());
		}
	}
	return true;
}

fn fatal_file(dir: &PathBuf, message: &str) -> FileData {
	return FileData {
		path: dir.to_path_buf(),
		lintpath: dir.to_string_lossy().to_string(),
		file_name: "".to_string(),
		passed: Vec::new(),
		failed: vec![message.to_string()],
		fatal: true,
	};
}

/* load all files in a directory, and return a list (possibly empty) of subdirectories */
fn visit_dir(root: usize, dir: &PathBuf, ignore_dirs: &HashSet<OsString>, files: &mut Vec<FileData>) -> Vec<PathBuf> {

	let mut new_dirs: Vec<PathBuf> = Vec::new();

	let dir_str = dir.to_str();
	if dir_str.is_none() {
		files.push(fatal_file(dir, "not-utf8"));
		return new_dirs;
	}

	let path = dir.as_path();

	if path.exists() == false {
		// this should never occur, unless someone messed with the directory while we were reading it
		files.push(fatal_file(dir, "does-not-exist"));
		return new_dirs;
	}

	if path.is_dir() == false {
		// this should never occur, unless someone messed with the directory while we were reading it
		files.push(fatal_file(dir, "not-a-directory"));
		return new_dirs;
	}

	let dir_entry = fs::read_dir(path);
	if dir_entry.is_err() {
		files.push(fatal_file(dir, "unable-to-read-directory"));
		return new_dirs;
	}
	let dir_entry = dir_entry.unwrap();

	for entry in dir_entry {
		if entry.is_err() {
			println!("ERROR: Unable to read directory entry: {}", entry.err().unwrap());
			//LATER: should dir be something else?
			files.push(fatal_file(dir, "unable-to-read-directory-entry"));
			continue;
		}
		let entry = entry.unwrap();
		let entry_path = entry.path();
		if entry_path.is_dir() {
			let file_name = entry_path.file_name();
			if file_name.is_none() {
				files.push(fatal_file(&entry_path, "unable-to-get-directory-entry-name"));
				continue;
			}
			let file_name = file_name.unwrap();
			if ignore_dirs.contains(file_name) {
				println!("DEBUG: ignoring directory {}", entry_path.display());
				continue;
			} else {
				new_dirs.push(entry_path);
			}
		} else {
			let entry_path_str = entry_path.to_str();
			if entry_path_str.is_none() {
				files.push(fatal_file(&entry_path, "path-not-str"));
			} else {
				let entry_path_str = entry_path_str.unwrap();
				let file_data = FileData {
					path: entry_path.clone(),
					lintpath: entry_path_str[root..].to_string(),
					file_name: entry_path.file_name().unwrap().to_str().unwrap().to_string(),
					passed: Vec::new(),
					failed: Vec::new(),
					fatal: false,
				};
				files.push(file_data);
			}
		}
	}
	return new_dirs;
}
