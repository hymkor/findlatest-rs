extern crate chrono;
extern crate glob;

use chrono::{DateTime, Local};
use std::fs;
use std::io;
use std::path::Path;

fn report(path: &Path, t: &std::time::SystemTime) {
    let datetime: DateTime<Local> = (*t).into();
    println!(
        "{} {}",
        datetime.format("%Y-%m-%d %H:%M:%S"),
        path.display()
    );
}

fn trydir(dir: &Path, latest: &mut std::time::SystemTime) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(fname) = path.file_name() {
                if fname != "." && fname != ".." {
                    trydir(&path, latest)?;
                }
            }
            continue;
        }
        let modified = path.metadata()?.modified()?;
        if modified > *latest {
            *latest = modified;
            report(&path, &modified);
        }
    }
    return Ok(());
}

fn tryone(filename: &Path, latest: &mut std::time::SystemTime) -> io::Result<()> {
    let meta = fs::metadata(filename)?;
    if meta.is_dir() {
        trydir(filename, latest)?;
    }
    let modified = meta.modified()?;
    if modified > *latest {
        *latest = modified;
        report(&filename, latest);
    }
    return Ok(());
}

fn mains(args: std::env::Args) -> io::Result<()> {
    let mut latest = std::time::SystemTime::UNIX_EPOCH;
    let mut done = false;
    for arg in args.skip(1) {
        done = true;
        if let Ok(filenames) = glob::glob(&arg) {
            for filename in filenames {
                if let Ok(filename) = filename {
                    tryone(Path::new(&filename), &mut latest)?
                }
            }
        } else {
            tryone(Path::new(&arg), &mut latest)?;
        }
    }
    if !done {
        tryone(Path::new("."), &mut latest)?;
    }
    return Ok(());
}

fn main() {
    if let Err(err) = mains(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
