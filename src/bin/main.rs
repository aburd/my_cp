use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let (source_path, target_path) = get_command()?;
    match my_cp::execute(source_path, target_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            print_help();
            std::process::exit(1);
        }
    }
}

fn get_command() -> Result<(String, String)> {
    let mut args = env::args();
    args.next();
    let source_path = args.next().expect("TARGET_PATH is a required argument");
    let target_path = match args.next() {
        Some(p) => p,
        None => "./".to_owned(),
    };
    Ok((source_path, target_path))
}

fn print_help() {
    println!("usage: my_cp source_file target_file");
}
