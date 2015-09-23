mod nucleus;
mod color;

use std::io::{BufReader,BufRead,Write};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use nucleus::Nucleus;
use color::Color;

fn get_col(fname: String) -> HashMap<String,Color> {
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

fn get_nucl(fname: String) -> HashMap<String,Nucleus> {
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

fn get_elem(fname: String) -> HashMap<u8,String> {
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

fn get_nuccol(fname: String) -> HashMap<String,String> {
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

fn output_svg(nucl: &HashMap<String,Nucleus>, nuccol: &HashMap<String,String>, col: &HashMap<String,Color>, elem: &HashMap<u8,String>, magic: &HashSet<u8>) {
    let mut z_limits = HashMap::<u8,(u8,u8)>::new();
    let mut n_limits = HashMap::<u8,(u8,u8)>::new();
    let mut max_z: u8 = 0;
    let mut max_n: u8 = 0;
    let scale = 10;

    //TODO: Min as well?
    // Determine the limits of the chart
    for (_,n) in nucl {
        if n.n > max_n {
            max_n = n.n;
        }
        if n.z > max_z {
            max_z = n.z;
        }
    }

    // Determine the lowest and highest Z for each N and lowest and highest N for each Z
    for (_,n) in nucl {
        let x = z_limits.entry(n.z).or_insert((n.n,n.n));
        if n.n < x.0 {
            *x = (n.n,x.1);
        } else if n.n > x.1 {
            *x = (x.0,n.n);
        }

        let x = n_limits.entry(n.n).or_insert((n.z,n.z));
        if n.z < x.0 {
            *x = (n.z,x.1);
        } else if n.z > x.1 {
            *x = (x.0,n.z);
        }
    }

    // Output the SVG
    let mut svgfile = File::create("out.svg").unwrap();
    // Header
    let w = ((max_n as u32)+4)*scale;
    let h = ((max_z as u32)+3)*scale;
    let _ = write!(svgfile, "<svg xmlns=\"http://www.w3.org/2000/svg\"");
    let _ = write!(svgfile, " xmlns:xlink=\"http://www.w3.org/1999/xlink\"");
    let _ = write!(svgfile, " width=\"{}\" height=\"{}\">\n", w, h);

    // Styling
    let _ = write!(svgfile, "<style>\n");
    let _ = write!(svgfile, ".nucBox{{stroke:black;stroke-width:.1;}}\n");
    let _ = write!(svgfile, ".elName{{text-anchor:end;}}\n");
    let _ = write!(svgfile, ".magBox{{fill:none;stroke:black;stroke-width:.25;}}\n");

    for (name,c) in col {
        let _ = write!(svgfile, ".{}{{fill:rgb({},{},{})}}\n", name, c.r, c.g, c.b);
    }

    let _ = write!(svgfile, "</style>\n");

    // Create Transform Group
    let _ = write!(svgfile, "<g transform=\"scale({}) translate(2,1)\">\n", scale);

    // Nuclide Boxes
    for (name,n) in nucl {
        let x = n.n;
        let y = max_z - n.z;
        let ref c = nuccol[name];
        let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", x, y);
        let _ = write!(svgfile, " width=\"1\" height=\"1\"");
        let _ = write!(svgfile, " class=\"nucBox {}\" />\n", c);
    }

    // Element Symbols
    for (z,e) in elem {
        // Determine x position
        // Only include element symbol if one of its isotopes is included
        if let Some(z1) = z_limits.get(&z) {
            let mut x = z1.0;
            if let Some(z2) = z_limits.get(&(z+1)) {
                if z2 < z1 {
                    x = z2.0;
                }
            }

            let xpos = (x as f32)-0.25;
            let ypos = (max_z as f32)-(*z as f32)+0.75;
            let _ = write!(svgfile, "<text x=\"{}\" y=\"{}\"", xpos, ypos);
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
            let ypos = max_z-nl.1;
            let h = nl.1-nl.0+1;
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", xpos, ypos);
            let _ = write!(svgfile, " width=\"1\" height=\"{}\"", h);
            let _ = write!(svgfile, " class=\"magBox\" />\n");
        }
        // Only include magic number outline if one of those isotopes is include
        if let Some(zl) = z_limits.get(&m) {
            let xpos = zl.0;
            let ypos = max_z-m;
            let w = zl.1-zl.0+1;
            let _ = write!(svgfile, "<rect x=\"{}\" y=\"{}\"", xpos, ypos);
            let _ = write!(svgfile, " width=\"{}\" height=\"1\"", w);
            let _ = write!(svgfile, " class=\"magBox\" />\n");
        }
    }

    let _ = write!(svgfile, "</g>\n");
    let _ = write!(svgfile, "</svg>\n");
}

fn main() {
    let nucl = get_nucl("data/nuclei".to_string());
    let nuccol = get_nuccol("data/nuccol".to_string());
    let col = get_col("data/colors".to_string());
    let elem = get_elem("data/elements".to_string());
    let magic = get_magic("data/magic".to_string());

    output_svg(&nucl, &nuccol, &col, &elem, &magic);
}
