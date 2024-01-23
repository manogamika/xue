use regex::Regex;
use std::io::{self, Write};
use torut::{Error, TorClient, util::stream::DataStream};
use tor_control::{TorControlClient, TorControlClientV3, TorControlError, types::CircuitStatus};
fn tokenize(input: &str) -> Vec<&str> {
    let word_pattern = Regex::new(r"\b\w+\b").unwrap();
    let tokens: Vec<&str> = word_pattern.find_iter(input).map(|mat| mat.as_str()).collect();
    tokens
}
fn main() {
    let tokens = tokenize(sentence);
    println!("Tokens: {:?}", tokens);
}
fn main() -> Result<(), Error> {
    let mut tor = TorClient::new()?;
    let control_socket = torut::ControlSocket::default();
    let tor_control_client = TorControlClientV3::connect(&control_socket)?;

    loop {
        print!("User: ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        if user_input.trim().to_lowercase() == "exit" {
            break;
        }
        println!("Bot: {}", user_input.trim());
        print_circuit_info(&tor_control_client)?;
        let circuit = tor.create_circuit()?;
        let stream = tor.create_stream(circuit, ":80")?;
        let request = format!("GET /?input={} HTTP/1.0\r\n\r\n", user_input.trim());
        let mut data_stream = DataStream::new(stream);
        data_stream.write_all(request.as_bytes())?;

        // Read and print the response
        let mut buffer = Vec::new();
        data_stream.read_to_end(&mut buffer)?;
        let bot_response = String::from_utf8_lossy(&buffer);
        println!("Onion Service Response:\n{}", bot_response);
    }

    Ok(())
}
fn print_circuit_info(tor_control_client: &TorControlClientV3) -> Result<(), TorControlError> {
    let circuit_status_list = tor_control_client.get_circuit_status_list()?;
    println!("Current Circuits:");
    for status in circuit_status_list {
        if let CircuitStatus::Extended { id, path } = status {
            println!("Circuit ID: {}, Path: {:?}", id, path);
        }
    }
            if user_input == "exit" {
            break;
        }
        print_circuit_info(&tor_control_client)?;
        let circuit = tor.create_circuit()?;
        let stream = tor.create_stream(circuit, ":80")?;
        let request = format!("GET /?input={} HTTP/1.0\r\n\r\n", user_input);
        let mut data_stream = DataStream::new(stream);
        data_stream.write_all(request.as_bytes())?;
        let mut buffer = Vec::new();
        data_stream.read_to_end(&mut buffer)?;
        let onion_service_response = String::from_utf8_lossy(&buffer);
        println!("Onion Service Response:\n{}", onion_service_response);
    }
    Ok(())
}
