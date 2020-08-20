// use wasmer::wasmer_runtime_core::pkg::Pkg
use std::io;
use std::io::BufRead;
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use wasmer::executor::{run, Run};

fn handle_client(stream: TcpStream) -> io::Result<()> {
    // Read the computation request from the TCP stream.
    // WARNING: blocking
    let mut zip_buffer = Vec::new();
    let mut reader = io::BufReader::new(stream);
    loop {
        let mut buffer = reader.fill_buf()?.to_vec();
        if buffer.len() == 0 {
            break;
        }
        reader.consume(buffer.len());
        zip_buffer.append(&mut buffer);
    }
    println!("read {} bytes", zip_buffer.len());

    // TODO: Decompress the request package into a temporary directory.

    // Replay the packaged computation.
    let path = Path::new("package/");
    let mut options = Run::new(path.to_path_buf());
    options.replay = true;
    run(&mut options);

    // TODO: Parse the return options from the TCP stream.
    // TODO: Serialize and return the requested results.
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;
    println!("listening on port 3333");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
