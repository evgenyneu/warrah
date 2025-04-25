mod common;
use assert_cmd::Command;
use common::run_in_terminal;
use expectrl::{spawn, Eof};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn test_main_success_run() {
    // let td: TempDir = TempDir::new().unwrap();
    //     td.mkfile_with_contents("file1.txt", "Hello");
    //     td.mkfile_with_contents("file2.txt", "World!");

    //     let output: String = run_in_terminal(td.path().display().to_string());

    //     let expected_output = r#"Hello
    // World!
    // "#;

    //     assert_eq!(output, expected_output);
}
