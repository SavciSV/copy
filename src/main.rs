use std::{env, fs::{copy, create_dir_all, read_dir}, io::Result, path::Path, process::exit};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Error: Bad arguments");
        println!("Usage: {} <source> <destination> [-r]", args[0]);
        exit(1);
    }

    let rec = args.iter().any(|arg| arg == "-r");
    let paths: Vec<&String> = args.iter().filter(|arg| *arg != "-r").collect();

    if paths.len() < 3 {
        eprintln!("{}: Need a source and a destination.", paths[0]);
        exit(3);
    }

    let src: &Path = Path::new(&paths[1]);
    let dst = Path::new(&paths[2]);

    if !src.exists() {
        eprintln!("{}: File doesn't exist.", paths[0]);
        exit(2);
    }
    
    if dst.is_dir() && !src.is_dir() {
        copy(&paths[1], format!("{}/{}", paths[2], paths[1]))?;
        return Ok(());
    }

    if src.is_dir() && !rec {
        eprintln!("{}: Please consider passing '-r'", paths[0]);
        exit(4);
    }

    if src.is_dir() && rec {
        copy_dir(src, dst, rec)?;
        return Ok(());
    }

    if !dst.exists() && paths[2].ends_with('/') {
        eprintln!("{}: Cannot create a file. '{}': Not a directory", paths[0], paths[2]);
        exit(5);
    }

    if paths.len() == 3 {
        copy(&paths[1], &paths[2])?;
    } else {
        for i in 2..paths.len() {
            copy(&paths[1], &paths[i])?;
        }
    }
    Ok(())

    
}


fn copy_dir(src: &Path, dst: &Path, rec: bool) -> Result<()> {
    if !dst.exists() {
        create_dir_all(dst)?;
    }

    for every in read_dir(src)? {
        let entry = every?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            if rec {
                copy_dir(&path, &dest_path, rec)?;
            }
        } else {
            copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
