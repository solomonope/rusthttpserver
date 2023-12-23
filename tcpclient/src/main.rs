use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let _s = stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];

    stream.read(&mut buffer).unwrap();

    print!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
