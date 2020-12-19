use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let command = get_command()?;
    println!("From: {:?}, To: {:?}", command.from_path, command.to_path);
    Ok(())
}

#[derive(Debug)]
struct Command {
    from_path: PathBuf,
    to_path: PathBuf,
}

fn get_command() -> Result<Command> {
    let mut args = env::args();
    let from_path: PathBuf = args
        .nth(1)
        .expect("From path is a required argument")
        .into();
    let to_path = match args.nth(2) {
        Some(p) => p.into(),
        None => Path::new("./").to_path_buf(),
    };
    // from_path
    Ok(Command { from_path, to_path })
}
