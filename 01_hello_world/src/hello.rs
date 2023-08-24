pub fn say_hello() -> Result<String, Box<dyn std::error::Error>> {
    let hello_world = "Hello world!, from EzekTec-Inc.".to_owned();

    Ok(hello_world)
}
