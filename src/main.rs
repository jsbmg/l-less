use std::fs::read_dir;
use std::env::current_dir;

use colored::Colorize;
use chrono::prelude::*;
use chrono::DateTime;

fn main() -> std::io::Result<()> {
    ls()?;
    Ok(())
}

fn ls() -> std::io::Result<()> {
    let mut dirs = Vec::new();
    let dir = current_dir()?;
    let paths = read_dir(&dir)?;


    // sort the vector of directories
    for path in paths {
	dirs.push(path?.path());
    }
    dirs.sort();

    println!("{: <12} {: <12} {: <12}", "name", "bytes", "modified");
    for path in dirs {
	// let path = path?.path();
	let metadata = std::fs::metadata(&path)?;

	// unpack metadata
	let size = metadata.len();
	let modified: DateTime<Local> = metadata.modified()?.into();
	let modified_fmted = modified.format("%a %b %-d %-H:%m").to_string();
	let accessed = metadata.accessed()?;
        let path_name = path.file_name().and_then(|path| path.to_str()).unwrap();
	let path_name_colored;

	// color directories
	if metadata.is_dir() {
	    path_name_colored = path_name.blue();
	} else {
	    path_name_colored = path_name.normal();
	}

	// print
	println!("{: <12} {: <12} {: <12}", path_name_colored, size, modified_fmted);
    }
    Ok(())
}
