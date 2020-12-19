use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

pub fn copy_dir(source_path: impl Into<PathBuf>, target_path: impl Into<PathBuf>) -> Result<usize> {
    let bytes = fs::read(source_path.into())?;

    let target_file = File::create(target_path.into()).expect("Could not create SOURCE_FILE");
    let mut writer = BufWriter::new(target_file);

    Ok(0)
}

pub fn copy_file(
    source_path: impl Into<PathBuf>,
    target_path: impl Into<PathBuf>,
) -> Result<usize> {
    let bytes = fs::read(source_path.into())?;

    let target_file = File::create(target_path.into()).expect("Could not create SOURCE_FILE");
    let mut writer = BufWriter::new(target_file);

    writer.write_all(&bytes)?;

    Ok(bytes.len())
}
