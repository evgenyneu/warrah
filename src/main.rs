mod build_mappings;
mod languages;
mod test_utils;

use languages::language_maps::{
    get_comments_by_language, get_language_by_extension, get_language_by_filename,
};
use std::time::Instant;

/// Main entry point for the application.
fn main() {
    println!("First calls (includes HashMap initialization):");

    // Example 1: Get language by extension
    let extension = ".rs";
    let start = Instant::now();
    if let Some(language) = get_language_by_extension(extension) {
        println!("File with extension {} is a {} file", extension, language);
    }
    println!(
        "Time taken for first extension lookup: {:?}",
        start.elapsed()
    );

    // Example 2: Get language by filename
    let filename = "Dockerfile";
    let start = Instant::now();
    if let Some(language) = get_language_by_filename(filename) {
        println!("File named {} is a {} file", filename, language);
    }
    println!(
        "Time taken for first filename lookup: {:?}",
        start.elapsed()
    );

    // Example 3: Get comments for a language
    let language = "python";
    let start = Instant::now();
    if let Some(comments) = get_comments_by_language(language) {
        println!("{} comments:", language);
        println!("  Single-line: {:?}", comments.single_line);
        println!("  Multi-line: {:?}", comments.multi_line);
    }
    println!(
        "Time taken for first comments lookup: {:?}",
        start.elapsed()
    );

    println!("\nSecond calls (should be faster):");

    // Second calls with different parameters
    let extension = ".py";
    let start = Instant::now();
    if let Some(language) = get_language_by_extension(extension) {
        println!("File with extension {} is a {} file", extension, language);
    }
    println!(
        "Time taken for second extension lookup: {:?}",
        start.elapsed()
    );

    let filename = "Makefile";
    let start = Instant::now();
    if let Some(language) = get_language_by_filename(filename) {
        println!("File named {} is a {} file", filename, language);
    }
    println!(
        "Time taken for second filename lookup: {:?}",
        start.elapsed()
    );

    let language = "rust";
    let start = Instant::now();
    if let Some(comments) = get_comments_by_language(language) {
        println!("{} comments:", language);
        println!("  Single-line: {:?}", comments.single_line);
        println!("  Multi-line: {:?}", comments.multi_line);
    }
    println!(
        "Time taken for second comments lookup: {:?}",
        start.elapsed()
    );
}
