use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(rs: String, gs: String, bs: String) -> Color {
        Color {
            r: rs.parse::<u8>().unwrap(),
            g: gs.parse::<u8>().unwrap(),
            b: bs.parse::<u8>().unwrap(),
        }
    }
}

struct Nucleus {
    z: u8,
    n: u8,
}

impl Nucleus {
    fn new(zs: String, ns: String) -> Nucleus {
        Nucleus {
            z: zs.parse::<u8>().unwrap(),
            n: ns.parse::<u8>().unwrap(),
        }
    }

    fn a(&self) -> u16 {
        (self.z as u16)+(self.n as u16)
    }
}

fn get_col(fname: String) -> HashMap<String, Color> {
    let mut col = HashMap::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let name = x[0].to_string();
        let c = Color::new(x[1].to_string(), x[2].to_string(), x[3].to_string());
        col.insert(name, c);
    }

    col
}

fn get_nucl(fname: String) -> HashMap<String, Nucleus> {
    let mut nucl = HashMap::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let name = x[0].to_string();
        let n = Nucleus::new(x[1].to_string(), x[2].to_string());
        nucl.insert(name, n);
    }

    nucl
}

fn get_elem(fname: String) -> HashMap<u8, String> {
    let mut elem = HashMap::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let z = x[0].to_string().parse::<u8>().unwrap();
        let name = x[1].to_string();
        elem.insert(z, name);
    }

    elem
}

fn get_nuccol(fname: String) -> HashMap<String, String> {
    let mut nuccol = HashMap::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let n = x[0].to_string();
        let c = x[1].to_string();
        nuccol.insert(n, c);
    }

    nuccol
}

fn get_magic(fname: String) -> HashSet<u8> {
    let mut magic = HashSet::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let m = x[0].to_string().parse::<u8>().unwrap();
        magic.insert(m);
    }

    magic
}

fn main() {
    let col = get_col("data/colors".to_string());
    let nucl = get_nucl("data/nuclei".to_string());
    let nuccol = get_nuccol("data/nuccol".to_string());
    let elem = get_elem("data/elements".to_string());
    let magic = get_magic("data/magic".to_string());

    for (name, n) in nucl {
        let cname = nuccol.get(&name).unwrap();
        let c = col.get(cname).unwrap();
        println!("{},{},{},{},{},{},{},{}", name, n.z, n.n, n.a(), cname, c.r, c.g, c.b);
    }
}
