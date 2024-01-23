use torut::{Error, TorClient, util::stream::DataStream};

fn main() -> Result<(), Error> {
    // Connect to the Tor network
    let mut tor = TorClient::new()?;

    // Establish a circuit through the Tor network
    let circuit = tor.create_circuit()?;
    let stream = tor.create_stream(circuit, ":80")?;

    // Send a simple HTTP GET request to the Onion service
    let request = b"GET / HTTP/1.0\r\n\r\n";
    let mut data_stream = DataStream::new(stream);
    data_stream.write_all(request)?;

    // Read and print the response
    let mut buffer = Vec::new();
    data_stream.read_to_end(&mut buffer)?;
    println!("Response:\n{}", String::from_utf8_lossy(&buffer));

    Ok(())
}
