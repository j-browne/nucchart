pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(rs: String, gs: String, bs: String) -> Color {
        Color {
            r: rs.parse::<f32>().unwrap(),
            g: gs.parse::<f32>().unwrap(),
            b: bs.parse::<f32>().unwrap(),
        }
    }

    pub fn to_string_rgb_p(&self) -> String {
        let s = format!("rgb({:.1}%,{:.1}%,{:.1}%)", self.r, self.g, self.b);
        s
    }
}
