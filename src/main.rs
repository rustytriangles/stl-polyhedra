#[macro_use]
extern crate approx;
extern crate clap;
use clap::{Arg, App};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[path = "polys.rs"]
mod poly;

mod stl {
    use super::*;

    pub fn write_ascii(polys: Vec<Vec<[f64; 3]>>,
                       filename: &String,
                       solidname: &String) -> std::io::Result<()> {

        let path = Path::new(&filename);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't open {}: {}",
                               path.display(),
                               why.description()),
            Ok(file) => file,
        };

        let hdr = format!("solid {}\n", solidname);
        file.write(hdr.as_bytes())?;
        for p in polys {
            let t = poly::triangulate(&p);

            file.write_all(b"facet normal 0 0 1\n")?;
            for i in (0..t.len()).step_by(3) {
                file.write_all(b" outer loop\n")?;
                for j in 0..3 {
                    let v = format!("  vertex {0} {1} {2}\n",
                                    t[i+j][0],
                                    t[i+j][1],
                                    t[i+j][2]);
                    file.write_all(v.as_bytes())?;
                }
                file.write_all(b" endloop\n")?;
            }
            file.write_all(b"endfacet\n")?;
        }
        let ftr = format!("endsolid {}\n", solidname);
        file.write(ftr.as_bytes())?;
        Ok(())
    }
}

// Returns a HashMap from name of poly to function that creates it.
fn build_function_table() -> HashMap<String, fn() -> Vec<Vec<[f64;3]>>> {
    let mut poly_fcns: HashMap<String, fn() -> Vec<Vec<[f64;3]>>> = HashMap::new();

    // Platonic
    poly_fcns.insert("tetrahedron".to_string(), poly::tetrahedron);
    poly_fcns.insert("cube".to_string(), poly::cube);
    poly_fcns.insert("octahedron".to_string(), poly::octahedron);
    poly_fcns.insert("icosahedron".to_string(), poly::icosahedron);
    poly_fcns.insert("dodecahedron".to_string(), poly::dodecahedron);

    // Rhombic
    poly_fcns.insert("rhombic_dodecahedron".to_string(), poly::rhombic_dodecahedron);
    poly_fcns.insert("rhombic_triacontahedron".to_string(), poly::rhombic_triacontahedron);

    // Archimeden
    poly_fcns.insert("truncated_tetrahedron".to_string(), poly::truncated_tetrahedron);
    poly_fcns.insert("cuboctahedron".to_string(), poly::cuboctahedron);
    poly_fcns.insert("truncated_cube".to_string(), poly::truncated_cube);
    poly_fcns.insert("truncated_octahedron".to_string(), poly::truncated_octahedron);
    poly_fcns.insert("rhombicuboctahedron".to_string(), poly::rhombicuboctahedron);
    // truncated cuboctahedron
    // snub cube (2?)
    // icosidodecahedron
    // truncated dodecahedron
    // truncated icosahedron
    // rhombicosidodecahedron
    // truncated icosidodecahedron
    // snub dodecahedron

    poly_fcns
}

fn process_args() -> clap::ArgMatches<'static> {
    // @todo
    // - ASCII vs Binary
    // - size
    // - size mode (e.g. edge length)
    let matches = App::new("stl-polyhedra")
        .version("1.0")
        .arg(Arg::with_name("poly")
             .short("p")
             .long("poly")
             .takes_value(true)
             .help("Selects which polyhedra"))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
             .takes_value(true)
             .help("Name of the STL file"))
        .arg(Arg::with_name("list")
             .short("l")
             .long("list")
             .help("Prints list of supported polyhedra"))
        .get_matches();
    matches
}

fn main() {

    let poly_fcns = build_function_table();

    let matches = process_args();

    if matches.is_present("list") {
        println!("Supported polyhedra");
        for name in poly_fcns.keys() {
            println!("  {}",name);
        }
    } else {
        let poly_name = matches.value_of("poly").unwrap_or("cube").to_string();
        let filename = matches.value_of("output").unwrap_or("foo.stl").to_string();

        let mut verts = Vec::new();

        match poly_fcns.get(&poly_name) {
            Some(fcn) => {
                verts = fcn();
            }
            _ => {
                println!("Unrecognized polyhedron: {}", poly_name);
            }
        };

        let _ = stl::write_ascii(verts, &filename, &poly_name);
    }
}
