extern crate getopts;

mod nucleus;
mod color;

use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::vec::Vec;
use std::collections::{HashMap,HashSet};
use std::env;
use getopts::Options;
use nucleus::Nucleus;
use color::Color;

fn bufreader_from_name (fname: String) -> BufReader<File> {
    let file = match File::open(&fname) {
        Ok(f) => f,
        Err(_) => panic!{"ERROR: Error opening \'{}\'.", fname},
    };
    BufReader::new(file)
}

fn get_col(fname: String) -> HashMap<String, Color> {
    let mut col = HashMap::new();
    let f = bufreader_from_name(fname);
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let name = x[0].to_string();
        let c = Color::new(x[1].to_string(), x[2].to_string(), x[3].to_string());
        col.insert(name, c);
    }

    col
}

fn get_nucl(fname: String) -> Vec<(String, Nucleus)> {
    let mut nucl = Vec::new();
    let f = bufreader_from_name(fname);
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let name = x[0].to_string();
        let n = Nucleus::new(x[1].to_string(), x[2].to_string());
        nucl.push((name, n));
    }

    nucl
}

fn get_stable(fname: String) -> HashSet<String> {
    let mut stable = HashSet::new();
    let f = bufreader_from_name(fname);
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let n = x[0].to_string();
        stable.insert(n);
    }

    stable
}

fn get_elem(fname: String) -> Vec<(u8, String)> {
    let mut elem = Vec::new();
    let f = bufreader_from_name(fname);
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let z = x[0].to_string().parse::<u8>().unwrap();
        let name = x[1].to_string();
        elem.push((z, name));
    }

    elem
}

fn get_nuccol(fname: String) -> HashMap<String, String> {
    let mut nuccol = HashMap::new();
    let f = bufreader_from_name(fname);
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
    let f = bufreader_from_name(fname);
    for l in f.lines() {
        let l: String = l.unwrap();
        let x: Vec<_> = l.split("\t").collect();

        let m = x[0].to_string().parse::<u8>().unwrap();
        magic.push(m);
    }

    magic
}

fn output_svg(out_fname: &String,
              nucl: &Vec<(String, Nucleus)>,
              nuccol: &HashMap<String, String>,
              col: &HashMap<String, Color>,
              stable: &HashSet<String>,
              elem: &Vec<(u8, String)>,
              magic: &Vec<u8>) {
    let mut z_limits = HashMap::<u8, (u8, u8)>::new();
    let mut n_limits = HashMap::<u8, (u8, u8)>::new();
    let mut chart_z: Option<(u8, u8)> = None;
    let mut chart_n: Option<(u8, u8)> = None;
    let scale = 10;

    // Determine the limits of the chart
    for &(_, ref n) in nucl {
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
    }

    // Determine the lowest and highest Z for each N and lowest and highest N for
    // each Z
    for &(_, ref n) in nucl {
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

    // Output the SVG
    let mut svgfile = File::create(out_fname).expect(&format!("Error opening {}", out_fname));

    // Header
    let w = ((chart_n.unwrap().1 as u32) - (chart_n.unwrap().0 as u32) + 4) * scale;
    let h = ((chart_z.unwrap().1 as u32) - (chart_z.unwrap().0 as u32) + 3) * scale;
    let _ = write!(svgfile, "<svg xmlns=\"http://www.w3.org/2000/svg\"");
    let _ = write!(svgfile, " xmlns:xlink=\"http://www.w3.org/1999/xlink\"");
    let _ = write!(svgfile, " width=\"{}\" height=\"{}\">\n", w, h);

    // Styling
    let _ = write!(svgfile, "<style>\n");
    let _ = write!(svgfile, ".stableBox{{fill:none;stroke:black;stroke-width:.25;}}\n");
    let _ = write!(svgfile, ".unstableBox{{fill:none;stroke:black;stroke-width:.05;}}\n");
    let _ = write!(svgfile, ".magBox{{fill:none;stroke:black;stroke-width:.15;}}\n");
    let _ = write!(svgfile, ".elLabel{{text-anchor:end;}}\n");
    let _ = write!(svgfile, ".nLabel{{text-anchor:start;}}\n");

    for (name, c) in col {
        let _ = write!(svgfile, ".{}{{fill:rgb({:.1}%,{:.1}%,{:.1}%);}}\n", name, c.r, c.g, c.b);
    }

    let _ = write!(svgfile, "</style>\n");

    // Create Transform Group
    let _ = write!(svgfile,
                   "<g transform=\"scale({}) translate({},{}) scale(1,-1)\">\n",
                   scale,
                   2 - (chart_n.unwrap().0 as i32),
                   (chart_z.unwrap().1 as i32) + 2);

    // Nuclide Colors
    for &(ref name, ref n) in nucl {
        let x = n.n;
        let y = n.z;
        if let Some(c) = nuccol.get(name) {
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", x, y);
            let _ = write!(svgfile, " width=\"1\" height=\"1\"");
            let _ = write!(svgfile, " class=\"{}\" />\n", c);
        }
    }

    // Nuclide Outlines
    for &(ref name, ref n) in nucl {
        let x = n.n;
        let y = n.z;
        if let Some(_) = nuccol.get(name) {
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", x, y);
            let _ = write!(svgfile, " width=\"1\" height=\"1\"");
            if stable.contains(name) {
                let _ = write!(svgfile, " class=\"stableBox\" />\n");
            } else {
                let _ = write!(svgfile, " class=\"unstableBox\" />\n");
            }
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
            let _ = write!(svgfile, " font-size=\".9\" class=\"elLabel\">");
            let _ = write!(svgfile, "{}", e);
            let _ = write!(svgfile, "</text>\n");
        }
    }

    // Number of Neutrons
    for n in chart_n.unwrap().0..chart_n.unwrap().1 {
        // Determine y position
        // Only include element symbol if one of its isotopes is included
        if n % 2 == 0 {
            if let Some(n1) = n_limits.get(&n) {
                let mut y = n1.0;
                if let Some(n2) = n_limits.get(&(n + 1)) {
                    if n2 < n1 {
                        y = n2.0;
                    }
                }

                let xpos = n;
                let ypos = y;
                let xoff = 0.05;
                let yoff = 0.9;
                let _ = write!(svgfile, "<text x=\"{}\" y=\"{}\"", xoff, yoff);
                let _ = write!(svgfile,
                               " transform=\"translate({},{}) scale(1,-1)\"",
                               xpos,
                               ypos);
                let _ = write!(svgfile, " font-size=\".9\" class=\"nLabel\">");
                let _ = write!(svgfile, "{}", n);
                let _ = write!(svgfile, "</text>\n");
            }
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
    opts.optopt("s",
                "stable",
                "The data file containing stable nuclide information",
                "FILE");
    opts.optopt("e",
                "elements",
                "The data file containing element information",
                "FILE");
    opts.optopt("m",
                "magic",
                "The data file containing magic number information",
                "FILE");

    // Parse the command line arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Apply defaults
    let out_fname = matches.opt_str("o").unwrap_or("out.svg".to_string());
    let nucl_fname = matches.opt_str("n").unwrap_or("data/nuclei".to_string());
    let nuccol_fname = matches.opt_str("u").unwrap_or("data/nuccol".to_string());
    let col_fname = matches.opt_str("c").unwrap_or("data/colors".to_string());
    let stable_fname = matches.opt_str("s").unwrap_or("data/stable".to_string());
    let elem_fname = matches.opt_str("e").unwrap_or("data/elements".to_string());
    let magic_fname = matches.opt_str("m").unwrap_or("data/magic".to_string());

    // Read in data files
    let nucl = get_nucl(nucl_fname);
    let nuccol = get_nuccol(nuccol_fname);
    let col = get_col(col_fname);
    let elem = get_elem(elem_fname);
    let stable = get_stable(stable_fname);
    let magic = get_magic(magic_fname);

    // Create the image
    output_svg(&out_fname, &nucl, &nuccol, &col, &stable, &elem, &magic);
}
