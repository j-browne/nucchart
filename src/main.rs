extern crate getopts;

mod nucleus;
mod color;

use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
use std::env;
use getopts::Options;
use nucleus::Nucleus;
use color::Color;

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

fn get_elem(fname: String) -> Vec<(u8, String)> {
    let mut elem = Vec::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let z = x[0].parse::<u8>().unwrap();
        let name = x[1].to_string();
        elem.push((z, name));
    }

    elem
}

#[allow(dead_code)]
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

fn get_magic(fname: String) -> Vec<u8> {
    let mut magic = Vec::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let m = x[0].parse::<u8>().unwrap();
        magic.push(m);
    }

    magic
}

fn get_abun(fname: String) -> Vec<(String, f32)> {
    let mut abun = Vec::new();
    let f = BufReader::new(File::open(fname).unwrap());
    for l in f.lines() {
        const CHUNK_SIZE: usize = 15;
        let b = l.unwrap().into_bytes();
        let c = b.chunks(CHUNK_SIZE);
        for s in c {
            let s = std::str::from_utf8(s).unwrap();
            let x: Vec<&str> = s.split_whitespace().collect();

            let n = x[0].to_string();
            let a = x[1].parse::<f32>().unwrap();

            abun.push((n, a));
        }
    }

    abun
}

fn color_func(x: f32) -> Color {
    Color {
        r: (0.5f32 - x / 2f32) * 100f32,
        g: 0f32,
        b: x * 100f32,
    }
}

fn output_svg(out_fname: &String,
              abun: &Vec<(String, f32)>,
              nucl: &HashMap<String, Nucleus>,
              col: &HashMap<String, Color>,
              elem: &Vec<(u8, String)>,
              magic: &Vec<u8>) {
    let mut z_limits = HashMap::<u8, (u8, u8)>::new();
    let mut n_limits = HashMap::<u8, (u8, u8)>::new();
    let mut chart_z: Option<(u8, u8)> = None;
    let mut chart_n: Option<(u8, u8)> = None;
    let mut max_ab: f32;
    const SVG_SCALE: u32 = 10u32;

    // Determine the limits of the chart
    for &(ref name, _) in abun {
        if let Some(n) = nucl.get(name) {
            if chart_z == None {
                chart_z = Some((n.z, n.z));
            } else {
                let z0 = chart_z.unwrap().0;
                let z1 = chart_z.unwrap().1;
                if n.z < z0 {
                    chart_z = Some((n.z, z1));
                } else if n.z > z1 {
                    chart_z = Some((z0, n.z));
                }
            }
            if chart_n == None {
                chart_n = Some((n.n, n.n));
            } else {
                let n0 = chart_n.unwrap().0;
                let n1 = chart_n.unwrap().1;
                if n.n < n0 {
                    chart_n = Some((n.n, n1));
                } else if n.n > n1 {
                    chart_n = Some((n0, n.n));
                }
            }
        } else {
            let _ = writeln!(&mut std::io::stderr(), "{} is not in nucl", name);
        }
    }

    // Determine the lowest and highest Z for each N and lowest and highest N for
    // each Z
    for &(ref name, _) in abun {
        if let Some(n) = nucl.get(name) {
            let x = z_limits.entry(n.z).or_insert((n.n, n.n));
            if n.n < x.0 {
                *x = (n.n, x.1);
            } else if n.n > x.1 {
                *x = (x.0, n.n);
            }

            let x = n_limits.entry(n.n).or_insert((n.z, n.z));
            if n.z < x.0 {
                *x = (n.z, x.1);
            } else if n.z > x.1 {
                *x = (x.0, n.z);
            }
        }
    }

    // Determine max abundance
    max_ab = abun[0].1;
    for &(_, ab) in abun {
        max_ab = f32::max(max_ab, ab);
    }

    // Output the SVG
    let mut svgfile = File::create(out_fname).unwrap();

    // Header
    let w = ((chart_n.unwrap().1 as u32) - (chart_n.unwrap().0 as u32) + 4) * SVG_SCALE;
    let h = ((chart_z.unwrap().1 as u32) - (chart_z.unwrap().0 as u32) + 3) * SVG_SCALE;
    let _ = write!(svgfile, "<svg xmlns=\"http://www.w3.org/2000/svg\"");
    let _ = write!(svgfile, " xmlns:xlink=\"http://www.w3.org/1999/xlink\"");
    let _ = write!(svgfile, " width=\"{}\" height=\"{}\">\n", w, h);

    // Styling
    let _ = write!(svgfile, "<style>\n");
    let _ = write!(svgfile, ".nucBox{{stroke:black;stroke-width:.1;}}\n");
    let _ = write!(svgfile, ".elName{{text-anchor:end;}}\n");
    let _ = write!(svgfile,
                   ".magBox{{fill:none;stroke:black;stroke-width:.25;}}\n");

    for (name, c) in col {
        let _ = write!(svgfile, ".{}{{fill:{};}}\n", name, c.to_string_rgb_p());
    }

    let _ = write!(svgfile, "</style>\n");

    // Create Transform Group
    let _ = write!(svgfile,
                   "<g transform=\"scale({}) translate({},{}) scale(1,-1)\">\n",
                   SVG_SCALE,
                   2 - (chart_n.unwrap().0 as i32),
                   (chart_z.unwrap().1 as i32) + 2);

    // Nuclide Boxes
    for &(ref name, ab) in abun {
        if let Some(n) = nucl.get(name) {
            let x = n.n;
            let y = n.z;
            let c = color_func(f32::log2(ab / max_ab + 1f32));

            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", x, y);
            let _ = write!(svgfile, " width=\"1\" height=\"1\"");
            let _ = write!(svgfile, " fill=\"{}\" ", c.to_string_rgb_p());
            let _ = write!(svgfile, " class=\"nucBox\" />\n");
        }
    }

    // Element Symbols
    for &(ref z, ref e) in elem {
        // Determine x position
        // Only include element symbol if one of its isotopes is included
        if let Some(z1) = z_limits.get(&z) {
            let mut x = z1.0;
            if let Some(z2) = z_limits.get(&(z + 1)) {
                if z2 < z1 {
                    x = z2.0;
                }
            }

            let xpos = x;
            let ypos = z;
            let xoff = -0.25;
            let yoff = -0.25;
            let _ = write!(svgfile, "<text x=\"{}\" y=\"{}\"", xoff, yoff);
            let _ = write!(svgfile,
                           " transform=\"translate({},{}) scale(1,-1)\"",
                           xpos,
                           ypos);
            let _ = write!(svgfile, " font-size=\".9\" class=\"elName\">");
            let _ = write!(svgfile, "{}", e);
            let _ = write!(svgfile, "</text>\n");
        }
    }

    // Magic Number Outlines
    for m in magic {
        // Only include magic number outline if one of those isotones is include
        if let Some(nl) = n_limits.get(&m) {
            let xpos = m;
            let ypos = nl.0;
            let h = nl.1 - nl.0 + 1;
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", xpos, ypos);
            let _ = write!(svgfile, " width=\"1\" height=\"{}\"", h);
            let _ = write!(svgfile, " class=\"magBox\" />\n");
        }
        // Only include magic number outline if one of those isotopes is include
        if let Some(zl) = z_limits.get(&m) {
            let xpos = zl.0;
            let ypos = m;
            let w = zl.1 - zl.0 + 1;
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", xpos, ypos);
            let _ = write!(svgfile, " width=\"{}\" height=\"1\"", w);
            let _ = write!(svgfile, " class=\"magBox\" />\n");
        }
    }

    let _ = write!(svgfile, "</g>\n");
    let _ = write!(svgfile, "</svg>\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    // These are the command line options
    opts.optopt("o",
                "output",
                "The SVG output file",
                "FILE");
    opts.optopt("n",
                "nuclei",
                "The data file containing nuclide information",
                "FILE");
    opts.optopt("u",
                "nuccol",
                "The data file containing nuclide-color map",
                "FILE");
    opts.optopt("c",
                "colors",
                "The data file containing color definitions",
                "FILE");
    opts.optopt("e",
                "elements",
                "The data file containing element information",
                "FILE");
    opts.optopt("m",
                "magic",
                "The data file containing magic number information",
                "FILE");
    opts.optopt("a",
                "abun",
                "The file containing abundances",
                "FILE");

    // Parse the command line arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Apply defaults
    let out_fname = matches.opt_str("o").unwrap_or("out.svg".to_string());
    let nucl_fname = matches.opt_str("n").unwrap_or("data/nuclei".to_string());
    //let nuccol_fname = matches.opt_str("u").unwrap_or("data/nuccol".to_string());
    let col_fname = matches.opt_str("c").unwrap_or("data/colors".to_string());
    let elem_fname = matches.opt_str("e").unwrap_or("data/elements".to_string());
    let magic_fname = matches.opt_str("m").unwrap_or("data/magic".to_string());
    let abun_fname = matches.opt_str("a").unwrap_or("abun".to_string());

    // Read in data files
    let nucl = get_nucl(nucl_fname);
    //let nuccol = get_nuccol(nuccol_fname);
    let col = get_col(col_fname);
    let elem = get_elem(elem_fname);
    let magic = get_magic(magic_fname);
    let abun = get_abun(abun_fname);

    // Create the image
    output_svg(&out_fname, &abun, &nucl, &col, &elem, &magic);
}
