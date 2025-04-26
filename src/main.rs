mod languages;
mod process;
mod test_utils;

use crate::process::file_path::process_from_file_path;
use std::path::PathBuf;

fn print_help() {
    println!("warrah - removes comments from code files");
    println!("\nUsage: warrah [PATH]");
    println!("\nArguments:");
    println!("  PATH    The path to the code file to strip comments from");
    println!("\nThe program will automatically detect the language based on the file extension or file name.");
    println!("The output is sent to stdout. To save to a file, use the '>' operator:");
    println!("\n  warrah code.py > code_no_comments.py");
}

/// Main entry point for the application.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        print_help();
        std::process::exit(1);
    }

    let file_path = PathBuf::from(&args[1]);

    match process_from_file_path(file_path) {
        Ok(content) => print!("{}", content),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
