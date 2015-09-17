use std::io::{BufReader,BufRead};
use std::fs::File;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(rs: &str, gs: &str, bs: &str) -> Color {
        Color {
            r: rs.parse::<u8>().unwrap(),
            g: gs.parse::<u8>().unwrap(),
            b: bs.parse::<u8>().unwrap(),
        }
    }
}

fn main() {
    let f = BufReader::new(File::open("data/colors").unwrap());
    for l in f.lines() {
        let l = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();
        let n = x[0];
        let c = Color::new(x[1], x[2], x[3]);
        println!("{},{},{},{}",n,c.r,c.g,c.b);
    }
    println!("Hello, world!");
}
