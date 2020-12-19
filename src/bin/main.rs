use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let (source_path, target_path, options) = get_command()?;

    if options.contains(&"-R".to_owned()) {
        if source_path.is_file() || target_path.is_file() {
            print_invalid_usage("Both SOURCE_PATH and TARGET_PATH must be directories.");
        }
        my_cp::copy_dir(&source_path, &target_path, true)?;
    } else {
        if source_path.is_dir() || target_path.is_dir() {
            print_invalid_usage("Both SOURCE_PATH and TARGET_PATH must be files.");
        }
        my_cp::copy_file(&source_path, &target_path, true)?;
    };

    Ok(())
}

fn get_command() -> Result<(PathBuf, PathBuf, Vec<String>)> {
    let mut args = env::args();
    args.next();

    let options: Vec<String> = get_options().unwrap_or(Vec::new());
    let non_options: Vec<_> = args
        .map(|a| a.to_string())
        .filter(|s| !s.starts_with('-'))
        .collect();
    let source_path: PathBuf = non_options
        .get(0)
        .expect("TARGET_PATH is a required argument")
        .into();

    let target_path: PathBuf = match non_options.get(1) {
        Some(p) => p.into(),
        None => source_path.file_name().unwrap().into(),
    };
    Ok((source_path, target_path, options))
}

fn get_options() -> Option<Vec<String>> {
    let valid_options: Vec<_> = ["-R"].to_vec();
    let options: Vec<String> = env::args()
        .map(|a| a.to_string())
        .filter(|s| s.starts_with('-'))
        .collect();

    for option in &options {
        if !valid_options.contains(&option.as_str()) {
            print_invalid_usage("Invalid argument given");
        }
    }

    Some(options)
}

fn print_help() {
    println!(
        "
        usage: my_cp source_file target_file
               my_cp -R source_dir target_dir
    "
    );
}

fn print_invalid_usage(msg: &str) {
    println!("Invalid usage: {}", msg);
    print_help();
    std::process::exit(1);
}
