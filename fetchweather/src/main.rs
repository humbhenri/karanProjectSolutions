use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;
use std::io::{Error, ErrorKind};

fn get_api_key() -> Result<String, io::Error> {
    let mut f = try!(File::open("apikey"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn get_ip_address() -> Result<String, io::Error> {
    let host = "ipinfo.io";
    let mut stream = try!(TcpStream::connect((host, 80)));
    let header = format!("GET /ip HTTP/1.0\r\nHost: {}\r\n\r\n", host);
    try!(stream.write(header.as_bytes()));
    let mut s = String::new();
    try!(stream.read_to_string(&mut s));
    match s.split("\r\n\r\n").nth(1) { // response body is the external ip
        Some(m) => Ok(m.trim().to_owned()),
        None => Err(Error::new(ErrorKind::Other, "ip addr not found"))
    }
}

fn main() {
    println!("{}", get_ip_address().unwrap());
}
