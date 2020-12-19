use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Command {
    source_path: PathBuf,
    target_path: PathBuf,
}

fn main() {
    match execute() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            print_help();
            std::process::exit(1);
        }
    }
}

fn execute() -> Result<usize> {
    let command = get_command()?;
    let bytes = fs::read(command.source_path)?;

    let target_file = File::create(command.target_path)?;
    let mut writer = BufWriter::new(target_file);
    writer.write_all(&bytes[..])?;

    Ok(0)
}

fn get_command() -> Result<Command> {
    let mut args = env::args();
    let source_path: PathBuf = args
        .nth(1)
        .expect("TARGET_PATH is a required argument")
        .into();
    let target_path = match args.nth(2) {
        Some(p) => p.into(),
        None => Path::new("./").to_path_buf(),
    };
    // source_path
    Ok(Command {
        source_path,
        target_path,
    })
}

fn print_help() {
    println!("usage: my_cp source_file target_file");
}
