mod comment_remover;
mod languages;
mod process;

#[doc(hidden)]
mod test_utils;

use crate::process::file_path::remove_comments;
use std::path::PathBuf;

pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

fn print_help() {
    println!("Sloppily remove comments from a code file.");
    println!("\n\u{1b}[1;4mUsage:\u{1b}[0m warrah [PATH]");
    println!("\n\u{1b}[1;4mArguments:\u{1b}[0m");
    println!("\n  PATH    The path to the code file to strip comments from");
    println!("\nThe language is auto-detected from the file name or extension.");
    println!("\n\u{1b}[1;4mExample:\u{1b}[0m");
    println!("\n  Output goes to stdout, use '>' to save it:");
    println!("\n    warrah code.py > code_no_comments.py");
}

/// Main entry point for the application.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let has_help_option =
        (args.len() == 2 && args[1] == "--help") || (args.len() == 2 && args[1] == "-h");

    if args.len() != 2 || has_help_option {
        print_help();

        if args.len() == 1 || has_help_option {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
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
