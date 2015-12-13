pub struct Nucleus {
    pub z: u8,
    pub n: u8,
}

impl Nucleus {
    pub fn new(zs: String, ns: String) -> Nucleus {
        Nucleus {
            z: zs.parse::<u8>().unwrap(),
            n: ns.parse::<u8>().unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn a(&self) -> u16 {
        (self.z as u16) + (self.n as u16)
    }
}
