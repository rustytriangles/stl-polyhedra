use std::collections::HashMap;
use std::env;
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

fn main() {
    let args: Vec<String> = env::args().collect();

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
    // truncated tetrahedron
    // cuboctahedron
    poly_fcns.insert("cuboctahedron".to_string(), poly::cuboctahedron);
    // truncated cube
    // truncated octahedron
    // rhombicuboctahedron
    poly_fcns.insert("rhombicuboctahedron".to_string(), poly::rhombicuboctahedron);
    // truncated cuboctahedron
    // snub cube (2?)
    // icosidodecahedron
    // truncated dodecahedron
    // truncated icosahedron
    // rhombicosidodecahedron
    // truncated icosidodecahedron
    // snub dodecahedron

    let name = &args[1];

    let mut verts = Vec::new();

    match poly_fcns.get(name) {
	Some(fcn) => {
	    verts = fcn();
	}
	_ => {
	    println!("Unrecognized polyhedron: {}", name);
	}
    };

    let filename = "foo.stl".to_string();
    let _ = stl::write_ascii(verts, &filename, name);
}
