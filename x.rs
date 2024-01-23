use std::process::{Command, Stdio};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Define a more complex JavaScript code
    let js_code = r#"
        console.log(result);
    "#;

    // Create a child process for Node.js
    let mut child = Command::new("node")
        .arg("-e")
        .arg(js_code)
        .stdout(Stdio::piped())
        .spawn()?;

    // Read and print the Node.js output
    let mut output = String::new();
    child.stdout.take().unwrap().read_to_string(&mut output)?;

    // Perform regex matching on the output
    let regex_pattern = r"Result: (.+)";
    let captures = regex::Regex::new(regex_pattern)
        .unwrap()
        .captures(&output)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "?"))?;

    // Extract and print the captured group
    if let Some(result) = captures.get(1) {
        println!("Transformed Result: {}", result.as_str());
    } else {
        println!("No match found");
    }

    Ok(())
}
