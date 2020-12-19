use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

pub fn copy_dir(
    source_path: impl Into<PathBuf>,
    target_path: impl Into<PathBuf>,
    verbose: bool,
) -> Result<()> {
    let source_path = source_path.into();
    let target_path = target_path.into();

    if source_path.is_file() {
        panic!("SOURCE_PATH must be a directory");
    }
    if target_path.is_file() {
        panic!("TARGET_PATH must be a directory");
    }
    fs::create_dir_all(&target_path)?;

    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();
        let new_target_path = target_path.join(path.file_name().unwrap());
        if path.is_file() {
            copy_file(path, new_target_path, verbose)?;
        } else {
            copy_dir(path, new_target_path, verbose)?;
        }
    }

    Ok(())
}

pub fn copy_file(
    source_path: impl Into<PathBuf>,
    target_path: impl Into<PathBuf>,
    verbose: bool,
) -> Result<usize> {
    let source_path = source_path.into();
    let target_path = target_path.into();

    if source_path == target_path {
        return Ok(0);
    }
    if source_path.is_dir() {
        panic!("SOURCE_PATH may not be a directory when copying a file");
    }
    if target_path.is_dir() {
        panic!("TARGET_PATH may not be a directory when copying a file");
    }
    let bytes = fs::read(&source_path)?;

    let target_file = File::create(&target_path).expect("Could not create SOURCE_FILE");
    let mut writer = BufWriter::new(target_file);

    writer.write_all(&bytes)?;
    let byte_count = bytes.len();

    if verbose {
        println!(
            "Copied {} bytes from {} to {}",
            byte_count,
            source_path.to_str().unwrap(),
            target_path.to_str().unwrap(),
        );
    }

    Ok(byte_count)
}
