use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;

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

fn main() {
    let mut nucl = HashMap::new();

    let f = BufReader::new(File::open("data/nuclei").unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let name = x[0].to_string();
        let n = Nucleus::new(x[1].to_string(), x[2].to_string());
        nucl.insert(name, n);
    }

    for (name, n) in nucl {
        println!("{},{},{},{}", name, n.z, n.n, n.a());
    }
}
