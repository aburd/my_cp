use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

pub fn copy_dir(source_path: impl Into<PathBuf>, target_path: impl Into<PathBuf>) -> Result<()> {
    let source_path = source_path.into();
    let target_path = target_path.into();

    if source_path.is_file() {
      panic!("SOURCE_PATH must be a directory");
    }
    if target_path.is_file() {
      panic!("TARGET_PATH must be a directory");
    }

    for entry in fs::read_dir(source_path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_file() {
        let new_target_path = target_path.join(path.file_name().unwrap());
        copy_file(path, new_target_path)?;
      }
    }

    Ok(())
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
