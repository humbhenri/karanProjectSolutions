use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;
use std::io::{Error, ErrorKind};

fn get_api_key() -> Result<String, io::Error> {
    let mut f = try!(File::open("apikey"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s.trim().to_owned())
}

fn get_body_http_request(host: &str, header: &str) -> Result<String, io::Error> {
    let mut stream = try!(TcpStream::connect((host, 80)));
    let header = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", header, host);
    try!(stream.write(header.as_bytes()));
    println!("request = {}", header);
    let mut s = String::new();
    try!(stream.read_to_string(&mut s));
    match s.split("\r\n\r\n").nth(1) { 
        Some(m) => Ok(m.trim().to_owned()),
        None => Err(Error::new(ErrorKind::Other, "ip addr not found"))
    }

}

fn get_ip_address() -> Result<String, io::Error> {
    let host = "ipinfo.io";
    let header = "/ip";
    let s = try!(get_body_http_request(host, header));
    Ok(s)
}

fn get_weather() -> Result<String, io::Error> {
    let host = "api.worldweatheronline.com";
    let header = format!("/free/v2/weather.ashx?key={}&q={}&num_of_days=0&format=json", try!(get_api_key()), try!(get_ip_address()));
    let s = try!(get_body_http_request(host, &header));
    Ok(s)
}

fn main() {
    println!("{}", get_weather().unwrap());
}
