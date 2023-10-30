use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the JavaScript code you want to run
    let js_code = r#"
        const transformer = require('./transformer.js');
        const result = transformer.transform('');
        console.log(result);
    "#;

    // Create a child process for Node.js
    let mut child = Command::new("node")
        .arg("-e")
        .arg(js_code)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    // Read and print the Node.js output
    let stdout = child.stdout.take().unwrap();
    let mut output = String::new();
    stdout
        .read_to_string(&mut output)
        .expect("Failed to read output");
    print!("{}", output);

    Ok(())
}
