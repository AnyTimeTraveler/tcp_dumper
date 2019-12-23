use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    loop {
        let mut received = String::new();
        if stream.read_to_string(&mut received)? == 0 {
            return Ok(());
        }
        print!("{}", received);
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
