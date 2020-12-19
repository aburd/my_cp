use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let (source_path, target_path, options) = match get_command() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            print_help();
            std::process::exit(1);
        }
    };

    if options.contains(&"-R".to_owned()) {
        my_cp::copy_dir(&source_path, &target_path)?
    } else {
        let byte_count = my_cp::copy_file(&source_path, &target_path)?;
        println!(
            "Copied {} bytes from {} to {}",
            byte_count,
            source_path.to_str().unwrap(),
            target_path.to_str().unwrap(),
        );
    };

    Ok(())
}

fn get_command() -> Result<(PathBuf, PathBuf, Vec<String>)> {
    let mut args = env::args();
    args.next();

    let options: Vec<String> = get_options().unwrap_or(Vec::new());
    let source_path: PathBuf = args
        .next()
        .expect("TARGET_PATH is a required argument")
        .into();

    let target_path: PathBuf = match args.next() {
        Some(p) => p.into(),
        None => source_path.file_name().unwrap().into(),
    };
    Ok((source_path, target_path, options))
}

fn print_help() {
    println!("usage: my_cp source_file target_file");
}

fn get_options() -> Option<Vec<String>> {
    let valid_options: Vec<_> = ["-R"].to_vec();
    let options: Vec<String> = env::args().collect();

    for option in &options {
        if !valid_options.contains(&option.as_str()) {
            return None;
        }
    }

    Some(options)
}
