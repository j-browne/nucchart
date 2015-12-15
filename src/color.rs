pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[allow(dead_code)]
impl Color {
    pub fn from_rgb_p(r:f32, g:f32, b:f32) -> Option<Color> {
        if r >= 0. && r <= 1. && g >= 0. && g <= 1. && b >= 0. && b <= 1. {
            Some(Color {r: r, g: g, b: b,})
        } else {
            None
        }
    }

    pub fn from_string_rgb_p(rs: String, gs: String, bs: String) -> Option<Color> {
            if let (Ok(r), Ok(g), Ok(b)) = (rs.parse::<f32>(), gs.parse::<f32>(), bs.parse::<f32>()) {
                Color::from_rgb_p(r, g, b)
            } else {
                None
            }
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Option<Color> {
        let c = (1. - f32::abs(2. * l - 1.)) * s;
        let x = c * (1. - f32::abs((h/60.) % 2. - 1.));
        let m = l - c / 2.;

        let mut col = Color{r: m, g: m, b: m,};
        let mut r = None;

        // HSL is only valid for H=[0,360), S=[0,1], L=[0,1]
        if l >= 0. && l <= 1. && s >= 0. && s <= 1. {
            if h >= 0. && h < 60. {
                col.r += c;
                col.g += x;

                r = Some(col)
            } else if h >= 60. && h < 120. {
                col.r += x;
                col.g += c;

                r = Some(col);
            } else if h >= 120. && h < 180. {
                col.g += c;
                col.b += x;

                r = Some(col);
            } else if h >= 180. && h < 240. {
                col.g += x;
                col.b += c;

                r = Some(col);
            } else if h >= 240. && h < 300. {
                col.r += x;
                col.b += c;

                r = Some(col);
            } else if h >= 300. && h < 360. {
                col.r += c;
                col.b += x;

                r = Some(col);
            }
        }
        r
    }

    pub fn to_string_rgb_p(&self) -> String {
        let s = format!("rgb({:.1}%,{:.1}%,{:.1}%)", self.r * 100., self.g * 100., self.b * 100.);
        s
    }
}
