mod languages {
    include!("src/languages/build/mod.rs");
}

fn main() {
    println!("cargo:rerun-if-changed=docs/languages.md");

    let doc_path = "docs/languages.md";
    let output_path = "src/languages/generated.rs";

    // Create the generated directory if it doesn't exist
    std::fs::create_dir_all(std::path::Path::new(output_path).parent().unwrap())
        .expect("Failed to create generated directory");

    if let Err(e) = languages::map_languages::generate_and_save_all_mappings(doc_path, output_path)
    {
        panic!("Failed to generate language mappings: {}", e);
    }
}
