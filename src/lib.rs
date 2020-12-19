use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

pub fn execute(source_path: impl Into<PathBuf>, target_path: impl Into<PathBuf>) -> Result<usize> {
    let bytes = fs::read(source_path.into())?;

    let target_file = File::create(target_path.into())?;
    let mut writer = BufWriter::new(target_file);
    writer.write_all(&bytes[..])?;

    Ok(0)
}
