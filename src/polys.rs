pub fn tetrahedron() -> Vec<Vec<[f64; 3]>> {
    let v = [
	[-1.,-1.,-1.],
	[-1., 1., 1.],
	[ 1., 1.,-1.],
	[ 1.,-1., 1.] ];
    [ vec![v[0], v[1], v[2] ],
      vec![v[0], v[2], v[3] ],
      vec![v[3], v[2], v[1] ],
      vec![v[3], v[1], v[0] ],
    ].to_vec()
}

pub fn cube() -> Vec<Vec<[f64; 3]>> {
    let v = [
	[-1.,-1.,-1.],
	[ 1.,-1.,-1.],
	[-1., 1.,-1.],
	[ 1., 1.,-1.],
	[-1.,-1., 1.],
	[ 1.,-1., 1.],
	[-1., 1., 1.],
	[ 1., 1., 1.] ];
    [ vec![v[4], v[5], v[7], v[6] ],
      vec![v[5], v[1], v[3], v[7] ],
      vec![v[1], v[0], v[2], v[3] ],
      vec![v[0], v[4], v[6], v[2] ],
      vec![v[6], v[7], v[3], v[2] ],
      vec![v[0], v[1], v[5], v[4] ],
    ].to_vec()
}

pub fn octahedron() -> Vec<Vec<[f64; 3]>> {
    let v = [
	[-1., 0., 0.],
	[ 1., 0., 0.],
	[ 0.,-1., 0.],
	[ 0., 1., 0.],
	[ 0., 0.,-1.],
	[ 0., 0., 1.] ];

    [ vec![v[2], v[1], v[5] ],
      vec![v[2], v[4], v[1] ],
      vec![v[2], v[0], v[4] ],
      vec![v[2], v[5], v[0] ],
      vec![v[1], v[3], v[5] ],
      vec![v[5], v[3], v[0] ],
      vec![v[0], v[3], v[4] ],
      vec![v[4], v[3], v[1] ],
    ].to_vec()
}


pub fn icosahedron() -> Vec<Vec<[f64; 3]>> {
    let p = (1. + (5 as f64).sqrt()) / 2.;
    let v = [
	[ 0.,-1., -p],
	[ 0., 1., -p],
	[ 0.,-1.,  p],
	[ 0., 1.,  p],
	[-1., -p, 0.],
	[ 1., -p, 0.],
	[-1.,  p, 0.],
	[ 1.,  p, 0.],
	[ -p, 0.,-1.],
	[ -p, 0., 1.],
	[  p, 0.,-1.],
	[  p, 0., 1.] ];

    [ vec![v[ 3], v[ 2], v[11] ],
      vec![v[ 3], v[11], v[ 7] ],
      vec![v[ 3], v[ 7], v[ 6] ],
      vec![v[ 3], v[ 6], v[ 9] ],
      vec![v[ 3], v[ 9], v[ 2] ],
      vec![v[ 2], v[ 5], v[11] ],
      vec![v[ 5], v[10], v[11] ],
      vec![v[11], v[10], v[ 7] ],
      vec![v[10], v[ 1], v[ 7] ],
      vec![v[ 7], v[ 1], v[ 6] ],
      vec![v[ 1], v[ 8], v[ 6] ],
      vec![v[ 6], v[ 8], v[ 9] ],
      vec![v[ 8], v[ 4], v[ 9] ],
      vec![v[ 9], v[ 4], v[ 2] ],
      vec![v[ 4], v[ 5], v[ 2] ],
      vec![v[ 0], v[ 1], v[10] ],
      vec![v[ 0], v[10], v[ 5] ],
      vec![v[ 0], v[ 5], v[ 4] ],
      vec![v[ 0], v[ 4], v[ 8] ],
      vec![v[ 0], v[ 8], v[ 1] ],
    ].to_vec()
}

pub fn dodecahedron() -> Vec<Vec<[f64; 3]>> {
    let p = (1. + (5. as f64).sqrt()) / 2.;
    let i = 1.0 / p;

    let v = [
	[-1.,-1.,-1. ], // v0
	[ 1.,-1.,-1. ], // v1
	[-1., 1.,-1. ], // v2
	[ 1., 1.,-1. ], // v3
	[-1.,-1., 1. ], // v4
	[ 1.,-1., 1. ], // v5
	[-1., 1., 1. ], // v6
	[ 1., 1., 1. ], // v7
	[ 0., -p, -i ], // v8
	[ 0.,  p, -i ], // v9
	[ 0., -p,  i ], // v10
	[ 0.,  p,  i ], // v11
	[ -i, 0., -p ], // v12
	[ -i, 0.,  p ], // v13
	[  i, 0., -p ], // v14
	[  i, 0.,  p ], // v15
	[ -p, -i, 0. ], // v16
	[  p, -i, 0. ], // v17
	[ -p,  i, 0. ], // v18
	[  p,  i, 0. ] ]; // v19


    [ vec![v[10], v[ 5], v[15], v[13], v[ 4]],
      vec![v[15], v[ 7], v[11], v[ 6], v[13]],
      vec![v[ 5], v[17], v[19], v[ 7], v[15]],
      vec![v[16], v[ 4], v[13], v[ 6], v[18]],
      vec![v[14], v[ 1], v[ 8], v[ 0], v[12]],
      vec![v[ 8], v[ 1], v[17], v[ 5], v[10]],
      vec![v[ 1], v[14], v[ 3], v[19], v[17]],
      vec![v[ 8], v[10], v[ 4], v[16], v[ 0]],
      vec![v[ 9], v[ 3], v[14], v[12], v[ 2]],
      vec![v[11], v[ 7], v[19], v[ 3], v[ 9]],
      vec![v[ 6], v[11], v[ 9], v[ 2], v[18]],
      vec![v[ 2], v[12], v[ 0], v[16], v[18]],
    ].to_vec()
}

pub fn rhombic_dodecahedron() -> Vec<Vec<[f64; 3]>> {
    let v = [
	[-1.0, 0.0, 0.0],
	[ 1.0, 0.0, 0.0],
	[ 0.0,-1.0, 0.0],
	[ 0.0, 1.0, 0.0],
	[ 0.0, 0.0,-1.0],
	[ 0.0, 0.0, 1.0],
	[-0.5, 0.5, 0.5],
	[ 0.5,-0.5, 0.5],
	[ 0.5, 0.5,-0.5],
	[ 0.5,-0.5,-0.5],
	[-0.5, 0.5,-0.5],
	[-0.5,-0.5, 0.5],
	[-0.5,-0.5,-0.5],
	[ 0.5, 0.5, 0.5] ];
    [ vec![v[ 0], v[ 6], v[ 3], v[10] ],
      vec![v[11], v[ 5], v[ 6], v[ 0] ],
      vec![v[ 5], v[13], v[ 3], v[ 6] ],
      vec![v[ 8], v[ 4], v[10], v[ 3] ],
      vec![v[ 4], v[12], v[ 0], v[10] ],
      vec![v[ 1], v[ 8], v[ 3], v[13] ],
      vec![v[ 2], v[11], v[ 0], v[12] ],
      vec![v[ 2], v[ 7], v[ 5], v[11] ],
      vec![v[ 7], v[ 1], v[13], v[ 5] ],
      vec![v[ 1], v[ 9], v[ 4], v[ 8] ],
      vec![v[ 9], v[ 2], v[12], v[ 4] ],
      vec![v[ 1], v[ 7], v[ 2], v[ 9] ],
    ].to_vec()
}

pub fn rhombic_triacontahedron() -> Vec<Vec<[f64; 3]>> {
    let s5 = (5. as f64).sqrt();
    let c0 = s5 / 4.;
    let c1 = (5. + s5) / 8.;
    let c2 = (5. + 3. * s5)/ 8.;

    let v = [
	[ c1, 0., c2],
	[ c1, 0.,-c2],
	[-c1, 0., c2],
	[-c1, 0.,-c2],
	[ c2, c1, 0.],
	[ c2,-c1, 0.],
	[-c2, c1, 0.],
	[-c2,-c1, 0.],
	[ 0., c2, c1],
	[ 0., c2,-c1],
	[ 0.,-c2, c1],
	[ 0.,-c2,-c1],
	[ 0., c0, c2],
	[ 0., c0,-c2],
	[ 0.,-c0, c2],
	[ 0.,-c0,-c2],
	[ c2, 0., c0],
	[ c2, 0.,-c0],
	[-c2, 0., c0],
	[-c2, 0.,-c0],
	[ c0, c2, 0.],
	[ c0,-c2, 0.],
	[-c0, c2, 0.],
	[-c0,-c2, 0.],
	[ c1, c1, c1],
	[ c1, c1,-c1],
	[ c1,-c1, c1],
	[ c1,-c1,-c1],
	[-c1, c1, c1],
	[-c1, c1,-c1],
	[-c1,-c1, c1],
	[-c1,-c1,-c1] ];

    [ vec![v[ 0], v[12], v[ 2], v[14] ],
      vec![v[ 0], v[14], v[10], v[26] ],
      vec![v[ 0], v[26], v[ 5], v[16] ],
      vec![v[ 1], v[13], v[ 9], v[25] ],
      vec![v[ 1], v[25], v[ 4], v[17] ],
      vec![v[ 1], v[17], v[ 5], v[27] ],
      vec![v[ 2], v[28], v[ 6], v[18] ],
      vec![v[ 2], v[18], v[ 7], v[30] ],
      vec![v[ 2], v[30], v[10], v[14] ],
      vec![v[ 3], v[19], v[ 6], v[29] ],
      vec![v[ 3], v[29], v[ 9], v[13] ],
      vec![v[ 3], v[13], v[ 1], v[15] ],
      vec![v[ 4], v[20], v[ 8], v[24] ],
      vec![v[ 4], v[24], v[ 0], v[16] ],
      vec![v[ 4], v[16], v[ 5], v[17] ],
      vec![v[ 7], v[18], v[ 6], v[19] ],
      vec![v[ 7], v[19], v[ 3], v[31] ],
      vec![v[ 7], v[31], v[11], v[23] ],
      vec![v[ 8], v[22], v[ 6], v[28] ],
      vec![v[ 8], v[28], v[ 2], v[12] ],
      vec![v[ 8], v[12], v[ 0], v[24] ],
      vec![v[ 9], v[29], v[ 6], v[22] ],
      vec![v[ 9], v[22], v[ 8], v[20] ],
      vec![v[ 9], v[20], v[ 4], v[25] ],
      vec![v[10], v[30], v[ 7], v[23] ],
      vec![v[10], v[23], v[11], v[21] ],
      vec![v[10], v[21], v[ 5], v[26] ],
      vec![v[11], v[31], v[ 3], v[15] ],
      vec![v[11], v[15], v[ 1], v[27] ],
      vec![v[11], v[27], v[ 5], v[21] ],
    ].to_vec()
}

pub fn triangulate(src: &Vec<[f64; 3]>) -> Vec<[f64; 3]> {
    let n = src.len();
    if n == 3 {
	src.to_vec()
    } else if n > 3 {
	let mut dst = Vec::new();

	for i in 1..n-1 {
	    let v0 = 0;
	    let v1 = i;
	    let v2 = i+1;
	    dst.push(src[v0]);
	    dst.push(src[v1]);
	    dst.push(src[v2]);
	}

	dst
    } else {
	panic!("Bad polygon");
    }
}

fn centroid(src: &Vec<[f64; 3]>) -> [f64; 3] {
    let mut c = cgmath::Vector3::new(0.,0.,0.);

    for v in src {
	c = c + cgmath::Vector3::new(v[0],v[1],v[2]);
    }
    let scale = 1. / src.len() as f64;
    [scale * c[0], scale * c[1], scale * c[2]]
}

fn normal(src: &Vec<[f64; 3]>) -> [f64; 3] {
    let mut n = cgmath::Vector3::new(0.,0.,0.);

    for i in 0..src.len() {
	let j = (i+1) % src.len();
	n = n + cgmath::Vector3::new((src[i][1] - src[j][1]) * (src[i][2] + src[j][2]),
				     (src[i][2] - src[j][2]) * (src[i][0] + src[j][0]),
				     (src[i][0] - src[j][0]) * (src[i][1] + src[j][1]));
    }
    [n[0], n[1], n[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
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
    fn test_tetrahedron_orientation() {
	let verts = tetrahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
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

    #[test]
    fn test_cube_orientation() {
	let verts = cube();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }

    #[test]
    // 8 polys with 3 sides each
    fn test_octahedron_size() {
	let verts = octahedron();
	assert_eq!(verts.len(), 8);
	for p in verts {
	    assert_eq!(p.len(), 3);
	}
    }

    #[test]
    fn test_octahedron_orientation() {
	let verts = octahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }

    #[test]
    // 20 polys with 3 sides each
    fn test_icosahedron_size() {
	let verts = icosahedron();
	assert_eq!(verts.len(), 20);
	for p in verts {
	    assert_eq!(p.len(), 3);
	}
    }

    #[test]
    fn test_icoshedron_orientation() {
	let verts = icosahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }

    #[test]
    // 12 polys with 5 sides each
    fn test_dodecahedron_size() {
	let verts = dodecahedron();
	assert_eq!(verts.len(), 12);
	for p in verts {
	    assert_eq!(p.len(), 5);
	}
    }

    #[test]
    fn test_dodecahedron_orientation() {
	let verts = dodecahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }

    #[test]
    // 12 polys with 4 sides each
    fn test_rhombic_dodecahedron_size() {
	let verts = rhombic_dodecahedron();
	assert_eq!(verts.len(), 12);
	for p in verts {
	    assert_eq!(p.len(), 4);
	}
    }

    #[test]
    fn test_rhombic_dodecahedron_orientation() {
	let verts = rhombic_dodecahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }

    #[test]
    // 30 polys with 4 sides each
    fn test_rhombic_triacontahedron_size() {
	let verts = rhombic_triacontahedron();
	assert_eq!(verts.len(), 30);
	for p in verts {
	    assert_eq!(p.len(), 4);
	}
    }

    #[test]
    fn test_rhombic_triacontahedron_orientation() {
	let verts = rhombic_triacontahedron();
	for p in verts {
	    let c = centroid(&p);
	    let n = normal(&p);
	    assert!(dot(c,n) > 0.);
	}
    }
}

