use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod poly {
    pub fn tetrahedron() -> Vec<Vec<[f64; 3]>> {
	[ vec![[-1.,-1.,-1.],
	       [ 1., 1.,-1.],
	       [-1., 1., 1.] ],
	  vec![[-1.,-1.,-1.],
	       [ 1.,-1., 1.],
	       [ 1., 1.,-1.] ],
	  vec![[ 1.,-1., 1.],
	       [-1., 1., 1.],
	       [ 1., 1.,-1.] ],
	  vec![[ 1.,-1., 1.],
	       [-1.,-1.,-1.],
	       [-1., 1., 1.] ],
	].to_vec()
    }

    pub fn cube() -> Vec<Vec<[f64; 3]>> {
	[ vec![[-1.,-1., 1.],
	       [ 1.,-1., 1.],
	       [ 1., 1., 1.],
	       [-1., 1., 1.] ],
	  vec![[ 1.,-1., 1.],
	       [ 1.,-1.,-1.],
	       [ 1., 1.,-1.],
	       [ 1., 1., 1.] ],
	  vec![[ 1.,-1.,-1.],
	       [-1.,-1.,-1.],
	       [-1., 1.,-1.],
	       [ 1., 1.,-1.] ],
	  vec![[-1.,-1.,-1.],
	       [-1.,-1., 1.],
	       [-1., 1., 1.],
	       [-1., 1.,-1.] ],
	  vec![[-1., 1., 1.],
	       [ 1., 1., 1.],
	       [ 1., 1.,-1.],
	       [-1., 1.,-1.] ],
	  vec![[-1.,-1.,-1.],
	       [ 1.,-1.,-1.],
	       [ 1.,-1., 1.],
	       [-1.,-1., 1.] ],
	].to_vec()
    }

    pub fn triangulate(src: &Vec<[f64; 3]>) -> Vec<[f64; 3]> {
	let n = src.len();
	if n == 3 {
	    src.to_vec()
	} else if n > 3 {
	    let mut dst = Vec::new();

	    for i in (0..n).step_by(2) {
		let v0 = i;
		let v1 = i+1;
		let v2 = src.len() - i - 1;
		dst.push(src[v0]);
		dst.push(src[v1]);
		dst.push(src[v2]);
	    }

	    dst
	} else {
	    panic!("Bad polygon");
	}
    }

    #[cfg(test)]
    mod tests {
	use super::*;

	#[test]
	// 4 polys with 3 sides each
	fn test_tetrahedron_size() {
	    let verts = tetrahedron();
	    assert_eq!(verts.len(), 4);
	    for p in verts {
		assert_eq!(p.len(), 3);
	    }
	}

	#[test]
	// 6 polys with 4 sides each
	fn test_cube_size() {
	    let verts = cube();
	    assert_eq!(verts.len(), 6);
	    for p in verts {
		assert_eq!(p.len(), 4);
	    }
	}

    }
}

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

	    file.write_all(b"facet normal 0 0 1\n");
	    for i in (0..t.len()).step_by(3) {
		file.write_all(b" outer loop\n");
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

    for i in 0..args.len() {
        println!("Hello, {:?}!", args[i]);
    }

    let mut poly_fcns: HashMap<String, fn() -> Vec<Vec<[f64;3]>>> = HashMap::new();

    poly_fcns.insert("tetrahedron".to_string(), poly::tetrahedron);
    poly_fcns.insert("cube".to_string(), poly::cube);

    let name = &args[1];

    let mut verts = Vec::new();

    match poly_fcns.get(name) {
	Some(fcn) => {
	    verts = fcn();
	}
	_ => {
	}
    };

    let filename = "foo.stl".to_string();
    stl::write_ascii(verts, &filename, name);
}
