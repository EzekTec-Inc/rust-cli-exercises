use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn prints_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("hello_world")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let hello_world_text = "Hello world!, from EzekTec-Inc.\n".to_owned();
    assert_eq!(output_str, hello_world_text);
    // say_hello();

    Ok(())
}
