use std::fs::read_dir;
use std::env::{current_dir, args};

use colored::Colorize;
use chrono::prelude::*;
use chrono::DateTime;
use is_executable::IsExecutable;


// TODO: add ability to list recursively

fn main() -> std::io::Result<()> {
    ls()?;
    Ok(())
}

fn ls() -> std::io::Result<()> {    
    let mut arguments: Vec<String> = args().collect();
    let mut dir = current_dir()?;
    let mut all = false;
    let mut color = false;
    let mut vertical = false;
    let mut long = false;
    let mut key = false;

    let mut longest = 0;

    arguments.remove(0);
    for arg in arguments {
	if arg == "lz" {
	    continue
	} else if arg == "--color" || arg == "-c" {
	    color = true;
	} else if arg == "--all" || arg == "-a" {
	    all = true;
	} else if arg == "--long" || arg == "-l" {
	    long = true;
	} else if arg == "--key" || arg == "-k" {
	    key = true;
	} else if arg == "--vertical" || arg == "-v" {
	    vertical = true;
	} else {
	    dir = arg.into();
	}
    }
  
    let mut dirs = Vec::new();
    let paths = read_dir(&dir)?;

    for path in paths {
	dirs.push(path?.path());
    }
    dirs.sort();
  
    if key == true && long == true {
	println!("{: <12} {: <10} {: <18} {: <18} {: <10} {}",
		 "name",
		 "bytes",
		 "modified",
		 "accessed",
		 "writeable",
		 "executable");
    }
    
    let mut grid = Vec::new();
    for path in dirs {
	let metadata = std::fs::metadata(&path)?;
	let size = metadata.len();
	let modified = DateTime::<Local>::from(metadata.modified()?);
	let modified_fmted = modified.format("%a %b %-d %-H:%m").to_string();
	let accessed = DateTime::<Local>::from(metadata.accessed()?);
	let accessed_fmted = accessed.format("%a %b %-d %-H:%m").to_string();
	let mut permissions = metadata.permissions().readonly();
	let executable = path.is_executable();
	
	if permissions == true {
	    permissions = false
	} else {
	    permissions = true
	}
	
        let path_name = path.file_name().and_then(|path| path.to_str()).unwrap();

	if path_name.len() <= longest {
	    longest = path_name.len()
	}    
	
	if path_name.starts_with(".") && all == false {                    
	    continue
	}

	let path_name_colored;
	if metadata.is_dir() && color == true {
	    path_name_colored = path_name.blue();
	} else if executable && color == true {
	    path_name_colored = path_name.green();
	} else {
	    path_name_colored = path_name.normal();
	}

	if long == true {
	    println!("{: <12} {: <10} {: <18} {: <18} {: <10} {}",
		     path_name_colored,
		     size,
		     modified_fmted,
		     accessed_fmted,
		     permissions,
		     executable);
	} else if vertical == false {
	    grid.push(path_name_colored.to_string());
	} else {
	    println!("{}", path_name_colored);
	}
    }

    // TODO
    // print paths in a neat grid
    if vertical == false && long == false {
	println!("{}", &grid.join("  "));
    }
    Ok(())
}


// fn ls_dr {
    
   
    
