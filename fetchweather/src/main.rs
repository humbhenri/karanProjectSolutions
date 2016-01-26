use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Weather {}

fn get_api_key() -> Result<String, io::Error> {
    let mut f = try!(File::open("apikey"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn get_ip_address() -> Result<String, io::Error> {
    
}

fn get_weather() -> Weather {
    let end_point = String::new("http://api.worldweatheronline.com/free/v2/weather.ashx");
    end_point.push_str(get_api_key().ok().expect("api file not found"));
    end_point.push_str(get_ip_address().ok().expect("ip address not found"));
    end_point.push_str("&num_of_days=0&format=json");
}

fn main() {
    println!("{}", get_api_key().ok().expect("api file not found"));
}
