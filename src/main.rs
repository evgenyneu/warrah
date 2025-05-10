mod comment_remover;
mod languages;
mod process;
mod test_utils;

use crate::process::file_path::remove_comments;
use std::path::PathBuf;

pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

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

    let path = PathBuf::from(&args[1]);

    match remove_comments(path, MAX_FILE_SIZE, true) {
        Ok(content) => print!("{}", content),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
