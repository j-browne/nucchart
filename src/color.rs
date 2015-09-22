pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(rs: String, gs: String, bs: String) -> Color {
        Color {
            r: rs.parse::<u8>().unwrap(),
            g: gs.parse::<u8>().unwrap(),
            b: bs.parse::<u8>().unwrap(),
        }
    }
}
