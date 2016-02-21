extern crate chrono;
use chrono::*;
extern crate rss;
use rss::{Item, Rss};
extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use std::fmt;

extern crate getopts;
use getopts::Options;
use std::env;

const SIGNS: [&'static str; 12] = ["Aries",
                                   "Taurus",
                                   "Gemini",
                                   "Cancer",
                                   "Leo",
                                   "Virgo",
                                   "Libra",
                                   "Scorpio",
                                   "Sagittarius",
                                   "Capricorn",
                                   "Aquarius",
                                   "Pisces"];

#[derive(Debug)]
struct Horoscope {
    title: String,
    desc: String,
}

impl Horoscope {
    fn from_item(item: Item) -> Option<Horoscope> {
        let i = item.clone();
        let title = i.title.unwrap();
        let desc = i.description.unwrap();
        if SIGNS.into_iter().any(|s| title.contains(s)) {
            return Some(Horoscope {
                title: title,
                desc: desc,
            });
        }
        None
    }
}

impl fmt::Display for Horoscope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:\n{}\n", self.title, self.desc)
    }
}

fn get_horoscope() -> Result<Vec<Horoscope>, String> {
    let url = get_today_horoscope_url();
    let res = try!(read_from_url(&url).map_err(|e| e.to_string()));
    let Rss(chan) = try!(res.parse::<Rss>().map_err(|e| e.to_string()));
    Ok(chan.items.into_iter().filter_map(Horoscope::from_item).collect::<Vec<_>>())
}

fn get_today_horoscope_url() -> String {
    let now = Local::now();
    format!("http://www.findyourfate.com/rss/horoscope-astrology-feed.\
             asp?mode=view&todate={}/{}/{}",
            now.month(),
            now.day(),
            now.year())
}

fn read_from_url(url: &str) -> Result<String, hyper::Error> {
    let client = Client::new();

    let mut res = try!(client.get(url)
                             .header(Connection::close())
                             .send());

    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    Ok(body)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} SIGN", program);
    print!("{}", opts.usage(&brief));
}

fn print_horoscope(opt_sign: Option<&String>) {
    let sign = if let Some(s) = opt_sign {
        s.as_ref()
    } else {
        ""
    };
    if !sign.is_empty() && !SIGNS.iter().map(|s| s.to_lowercase()).any(|s| s == sign) {
        println!("{} is not a valid zodiac sign", opt_sign.unwrap());
        return;
    }
    match get_horoscope() {
        Ok(horoscopes) => {
            for h in horoscopes.into_iter()
                               .filter(|h| h.title.to_lowercase().contains(sign)) {
                println!("{}", h);
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    print_horoscope(matches.free.first().map(|x| x.to_lowercase()).as_ref());

}
