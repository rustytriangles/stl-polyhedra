#[macro_use]
extern crate approx;
extern crate clap;
use byteorder::{LittleEndian, WriteBytesExt};
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


    pub fn write_binary(polys: Vec<Vec<[f64; 3]>>,
                       filename: &String,
			solidname: &String) -> std::io::Result<()> {
        let path = Path::new(&filename);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't open {}: {}",
                               path.display(),
                               why.description()),
            Ok(file) => file,
        };

	// UINT8[80] - Header
	let hdr: [u8; 80] = ['x' as u8; 80];
        file.write_all(&hdr)?;

	let mut tris: Vec<[f32; 3]> = Vec::new();
        for p in polys {
            let t = poly::triangulate(&p);
            for i in (0..t.len()).step_by(3) {
                for j in 0..3 {
		    tris.push([t[i+j][0] as f32,
			       t[i+j][1] as f32,
			       t[i+j][2] as f32]);
		}
	    }
	}

	// UINT32 - Number of triangles
	let mut num_tris = vec![];
	num_tris.write_u32::<LittleEndian>((tris.len() / 3) as u32).unwrap();
        file.write_all(&num_tris)?;

	// foreach triangle
	// REAL32[3] - Normal vector
	// REAL32[3] - Vertex 1
	// REAL32[3] - Vertex 2
	// REAL32[3] - Vertex 3
	// UINT16 - Attribute byte count
	// end
        for i in (0..tris.len()).step_by(3) {
	    let mut normal = vec![];
	    normal.write_f32::<LittleEndian>(0.0).unwrap();
	    normal.write_f32::<LittleEndian>(0.0).unwrap();
	    normal.write_f32::<LittleEndian>(1.0).unwrap();
	    file.write_all(&normal)?;

	    let mut verts = vec![];
            for j in 0..3 {
		verts.write_f32::<LittleEndian>(tris[i+j][0]).unwrap();
		verts.write_f32::<LittleEndian>(tris[i+j][1]).unwrap();
		verts.write_f32::<LittleEndian>(tris[i+j][2]).unwrap();
	    }
	    file.write_all(&verts)?;

	    let attr_byte_count: [u8; 2] = [0, 0];
            file.write_all(&attr_byte_count)?;
	}
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

    // Archimedean
    poly_fcns.insert("truncated_tetrahedron".to_string(), poly::truncated_tetrahedron);
    poly_fcns.insert("truncated_cube".to_string(), poly::truncated_cube);
    poly_fcns.insert("truncated_octahedron".to_string(), poly::truncated_octahedron);
    poly_fcns.insert("truncated_icosahedron".to_string(), poly::truncated_icosahedron);
    poly_fcns.insert("truncated_dodecahedron".to_string(), poly::truncated_dodecahedron);
    poly_fcns.insert("cuboctahedron".to_string(), poly::cuboctahedron);
    poly_fcns.insert("rhombicuboctahedron".to_string(), poly::rhombicuboctahedron);
    poly_fcns.insert("truncated_cuboctahedron".to_string(), poly::truncated_cuboctahedron);
    poly_fcns.insert("snub_cube".to_string(), poly::snub_cube);
    poly_fcns.insert("icosidodecahedron".to_string(), poly::icosidodecahedron);
    poly_fcns.insert("rhombicosidodecahedron".to_string(), poly::rhombicosidodecahedron);
    // truncated icosidodecahedron
    // snub dodecahedron (2?)

    poly_fcns.insert("stellated_dodecahedron".to_string(), poly::stellated_dodecahedron);
    poly_fcns.insert("great_dodecahedron".to_string(), poly::great_dodecahedron);
    poly_fcns.insert("small_triambic_icosahedron".to_string(), poly::small_triambic_icosahedron);

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
        .arg(Arg::with_name("binary")
             .short("b")
             .long("binary")
             .help("Create binary STL file"))
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

	if matches.is_present("binary") {
            let _ = stl::write_binary(verts, &filename, &poly_name);
	} else {
            let _ = stl::write_ascii(verts, &filename, &poly_name);
	}
    }
}
