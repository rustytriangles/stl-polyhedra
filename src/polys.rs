enum SizeMode {
    EDGELEN,
    RADIUS
}

pub fn tetrahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2. / (3. as f64).sqrt();
    let edgelen_scale = (2. as f64).sqrt() / 2.;

    let (v, p) = generate_tetrahedron_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_tetrahedron_mesh(radius_scale, SizeMode::RADIUS);

    let mut ret = Vec::new();
    for i in p {
	let mut tmp = Vec::new();
	for j in i {
	    tmp.push(v[j]);
	}
	ret.push(tmp);
    }
    ret
}

pub fn cube() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2. / (3. as f64).sqrt();
    let edgelen_scale = 1.;

    let (v, p) = generate_cube_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_cube_mesh(radius_scale, SizeMode::RADIUS);

    let mut ret = Vec::new();
    for i in p {
	let mut tmp = Vec::new();
	for j in i {
	    tmp.push(v[j]);
	}
	ret.push(tmp);
    }
    ret
}

pub fn octahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2.;
    let edgelen_scale = (2. as f64).sqrt();

    let (v, p) = generate_octahedron_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_octahedron_mesh(radius_scale, SizeMode::RADIUS);

    let mut ret = Vec::new();
    for i in p {
	let mut tmp = Vec::new();
	for j in i {
	    tmp.push(v[j]);
	}
	ret.push(tmp);
    }
    ret
}

pub fn icosahedron() -> Vec<Vec<[f64; 3]>> {
    let edgelen_scale = 0.9510565162951535;

    let (v, p) = generate_icosahedron_mesh(1., SizeMode::EDGELEN);

    let mut ret = Vec::new();
    for i in p {
	let mut tmp = Vec::new();
	for j in i {
	    tmp.push(v[j]);
	}
	ret.push(tmp);
    }
    ret
}

pub fn dodecahedron() -> Vec<Vec<[f64; 3]>> {
    let edgelen_scale = 1.;

    let (v, p) = generate_dodecahedron_mesh(edgelen_scale, SizeMode::EDGELEN);

    let mut ret = Vec::new();
    for i in p {
	let mut tmp = Vec::new();
	for j in i {
	    tmp.push(v[j]);
	}
	ret.push(tmp);
    }
    ret
}

pub fn rhombic_dodecahedron() -> Vec<Vec<[f64; 3]>> {
    // edge length = (3. as f64).sqrt() / 2.
    let _radius_edgelen_ratio = 1.1547005383792517;
    let r = 2. / (3. as f64).sqrt();
    let p = r / 2.;
    let v = [
        [-r,0.,0.],
        [ r,0.,0.],
        [0.,-r,0.],
        [0., r,0.],
        [0.,0.,-r],
        [0.,0., r],
        [-p, p, p],
        [ p,-p, p],
        [ p, p,-p],
        [ p,-p,-p],
        [-p, p,-p],
        [-p,-p, p],
        [-p,-p,-p],
        [ p, p, p] ];
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
    // @todo fix this
    // edge length = 1.06331351044005;
    let _radius_edgelen_ratio = ((5. as f64).sqrt() + 1.) / 2.;
    // i.e.(c0*c0 + c1*c1).sqrt();
    let s5 = (5. as f64).sqrt();
    let c0 = s5 / 4.;
    let c1 = (5. + s5) / 8.;
    let c2 = (5. + 3. * s5)/ 8.;
    let d = (c0*c0 + c1*c1).sqrt();

    let v = [
        [ c1/d,   0., c2/d],
        [ c1/d,   0.,-c2/d],
        [-c1/d,   0., c2/d],
        [-c1/d,   0.,-c2/d],
        [ c2/d, c1/d,   0.],
        [ c2/d,-c1/d,   0.],
        [-c2/d, c1/d,   0.],
        [-c2/d,-c1/d,   0.],
        [   0., c2/d, c1/d],
        [   0., c2/d,-c1/d],
        [   0.,-c2/d, c1/d],
        [   0.,-c2/d,-c1/d],
        [   0., c0/d, c2/d],
        [   0., c0/d,-c2/d],
        [   0.,-c0/d, c2/d],
        [   0.,-c0/d,-c2/d],
        [ c2/d,   0., c0/d],
        [ c2/d,   0.,-c0/d],
        [-c2/d,   0., c0/d],
        [-c2/d,   0.,-c0/d],
        [ c0/d, c2/d,   0.],
        [ c0/d,-c2/d,   0.],
        [-c0/d, c2/d,   0.],
        [-c0/d,-c2/d,   0.],
        [ c1/d, c1/d, c1/d],
        [ c1/d, c1/d,-c1/d],
        [ c1/d,-c1/d, c1/d],
        [ c1/d,-c1/d,-c1/d],
        [-c1/d, c1/d, c1/d],
        [-c1/d, c1/d,-c1/d],
        [-c1/d,-c1/d, c1/d],
        [-c1/d,-c1/d,-c1/d] ];

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

pub fn truncated_tetrahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2. / (3. as f64).sqrt();
    let edgelen_scale = 3. * (2. as f64).sqrt() / 2.;

    let (v, p) = generate_tetrahedron_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_tetrahedron_mesh(radius_scale, SizeMode::RADIUS);

    let mut ret = Vec::new();
    for l in p {
	let mut hex = Vec::new();
	hex.push(lerp(v[l[0]], v[l[1]], 1. / 3.));
	hex.push(lerp(v[l[0]], v[l[1]], 2. / 3.));
	hex.push(lerp(v[l[1]], v[l[2]], 1. / 3.));
	hex.push(lerp(v[l[1]], v[l[2]], 2. / 3.));
	hex.push(lerp(v[l[2]], v[l[0]], 1. / 3.));
	hex.push(lerp(v[l[2]], v[l[0]], 2. / 3.));
	ret.push(hex);
    }

    // order of the edges for the tri at each corner
    let corners = [
	[1, 3, 2],
	[2, 3, 0],
	[3, 1, 0],
	[0, 1, 2] ];
    for i in 0..4 {
    	let mut tri = Vec::new();
    	tri.push(lerp(v[i],v[corners[i][0]],1./3.));
    	tri.push(lerp(v[i],v[corners[i][1]],1./3.));
    	tri.push(lerp(v[i],v[corners[i][2]],1./3.));
    	ret.push(tri);
    }
    ret
}

pub fn cuboctahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_edgelen_ratio = 1.;
    let p = 1. / (2. as f64).sqrt();
    let v = [
        [-p,-p,0.],
        [0.,-p, p],
        [ p,-p,0.],
        [0.,-p,-p],
        [-p,0., p],
        [ p,0., p],
        [ p,0.,-p],
        [-p,0.,-p],
        [0., p, p],
        [ p, p,0.],
        [0., p,-p],
        [-p, p,0.] ];

    [ vec![v[ 1], v[ 5], v[ 8], v[ 4]],
      vec![v[ 2], v[ 6], v[ 9], v[ 5]],
      vec![v[ 3], v[ 7], v[10], v[ 6]],
      vec![v[ 0], v[ 4], v[11], v[ 7]],
      vec![v[ 8], v[ 9], v[10], v[11]],
      vec![v[ 1], v[ 0], v[ 3], v[ 2]],
      vec![v[ 5], v[ 9], v[ 8]],
      vec![v[ 6], v[10], v[ 9]],
      vec![v[ 7], v[11], v[10]],
      vec![v[ 4], v[ 8], v[11]],
      vec![v[ 5], v[ 1], v[ 2]],
      vec![v[ 6], v[ 2], v[ 3]],
      vec![v[ 7], v[ 3], v[ 0]],
      vec![v[ 4], v[ 0], v[ 1]],
    ].to_vec()
}

pub fn truncated_cube() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2. / (3. as f64).sqrt();
    let edgelen_scale = 3. * (2. as f64).sqrt() / 2.;

    let (v, p) = generate_cube_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_cube_mesh(radius_scale, SizeMode::RADIUS);

    let s = (2. as f64).sqrt();
    let t1 = s / (2. + s + s);
    let t2 = 1. - t1;
    let mut ret = Vec::new();
    for l in p {
	let mut oct = Vec::new();
	oct.push(lerp(v[l[0]], v[l[1]], t1));
	oct.push(lerp(v[l[0]], v[l[1]], t2));
	oct.push(lerp(v[l[1]], v[l[2]], t1));
	oct.push(lerp(v[l[1]], v[l[2]], t2));
	oct.push(lerp(v[l[2]], v[l[3]], t1));
	oct.push(lerp(v[l[2]], v[l[3]], t2));
	oct.push(lerp(v[l[3]], v[l[0]], t1));
	oct.push(lerp(v[l[3]], v[l[0]], t2));
	ret.push(oct);
    }

    // order of the edges for the tri at each corner
    let corners = [
     	[1, 4, 2],
	[0, 3, 5],
	[0, 6, 3],
	[1, 2, 7],
	[0, 5, 6],
	[1, 7, 4],
	[2, 4, 7],
	[3, 6, 5],
    ];
    for i in 0..corners.len() {
	let mut tri = Vec::new();
	tri.push(lerp(v[i],v[corners[i][0]],t1));
	tri.push(lerp(v[i],v[corners[i][1]],t1));
	tri.push(lerp(v[i],v[corners[i][2]],t1));
	ret.push(tri);
    }
    ret
}

pub fn truncated_octahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_scale = 2. / (3. as f64).sqrt();
    let edgelen_scale = 3. * (2. as f64).sqrt() / 2.;

    let (v, p) = generate_octahedron_mesh(edgelen_scale, SizeMode::EDGELEN);
//    let (v, p) = generate_octahedron_mesh(radius_scale, SizeMode::RADIUS);

    let s = (2. as f64).sqrt();
    let t1 = 1. / 3.;
    let t2 = 1. - t1;
    let mut ret = Vec::new();
    for l in p {
	let mut hex = Vec::new();
	hex.push(lerp(v[l[0]], v[l[1]], t1));
	hex.push(lerp(v[l[0]], v[l[1]], t2));
	hex.push(lerp(v[l[1]], v[l[2]], t1));
	hex.push(lerp(v[l[1]], v[l[2]], t2));
	hex.push(lerp(v[l[2]], v[l[0]], t1));
	hex.push(lerp(v[l[2]], v[l[0]], t2));
	ret.push(hex);
    }

    let corners = [
     	[5, 3, 4, 2],
     	[2, 4, 3, 5],
	[0, 4, 1, 5],
	[5, 1, 4, 0],
	[3, 1, 2, 0],
	[0, 2, 1, 3],
    ];
    for i in 0..corners.len() {
	let mut tri = Vec::new();
	tri.push(lerp(v[i],v[corners[i][0]],t1));
	tri.push(lerp(v[i],v[corners[i][1]],t1));
	tri.push(lerp(v[i],v[corners[i][2]],t1));
	tri.push(lerp(v[i],v[corners[i][3]],t1));
	ret.push(tri);
    }
    ret
}

pub fn rhombicuboctahedron() -> Vec<Vec<[f64; 3]>> {
    let _radius_edgelen_ratio = 1.3989663259659066;
    let q = (1. + (2. as f64).sqrt()) / 2.;
    let r = 0.5;
    let v = [
        [-q,-r,-r], // v0
        [-q, r,-r],
        [-q,-r, r],
        [-q, r, r],
        [-r,-r,-q], // v4
        [-r, r,-q],
        [-r,-q,-r],
        [-r, q,-r],
        [-r,-q, r],
        [-r, q, r],
        [-r,-r, q],
        [-r, r, q],
        [ r,-r,-q], // v12
        [ r, r,-q],
        [ r,-q,-r],
        [ r, q,-r],
        [ r,-q, r],
        [ r, q, r],
        [ r,-r, q],
        [ r, r, q],
        [ q,-r,-r], // v20
        [ q, r,-r],
        [ q,-r, r],
        [ q, r, r],
    ];

    [vec![v[ 0], v[ 2], v[ 3], v[ 1]],
     vec![v[10], v[18], v[19], v[11]],
     vec![v[22], v[20], v[21], v[23]],
     vec![v[12], v[ 4], v[ 5], v[13]],
     vec![v[ 9], v[17], v[15], v[ 7]],
     vec![v[ 8], v[ 6], v[14], v[16]],
     vec![v[ 1], v[ 3], v[ 9], v[ 7]],
     vec![v[11], v[19], v[17], v[ 9]],
     vec![v[23], v[21], v[15], v[17]],
     vec![v[13], v[ 5], v[ 7], v[15]],
     vec![v[ 0], v[ 1], v[ 5], v[ 4]],
     vec![v[10], v[11], v[ 3], v[ 2]],
     vec![v[22], v[23], v[19], v[18]],
     vec![v[12], v[13], v[21], v[20]],
     vec![v[ 0], v[ 6], v[ 8], v[ 2]],
     vec![v[10], v[ 8], v[16], v[18]],
     vec![v[22], v[16], v[14], v[20]],
     vec![v[12], v[14], v[ 6], v[ 4]],
     vec![v[19], v[23], v[17]],
     vec![v[21], v[13], v[15]],
     vec![v[ 5], v[ 1], v[ 7]],
     vec![v[ 3], v[11], v[ 9]],
     vec![v[ 6], v[ 0], v[ 4]],
     vec![v[ 8], v[10], v[ 2]],
     vec![v[16], v[22], v[18]],
     vec![v[14], v[12], v[20]],

    ].to_vec()
}

// This one is kind of strange. It's not actually a truncated cuboctahedron, but
// that's the conventional name for it. If you actually do a trunction on a
// cuboctahedron, you'll see that you can't get things to work out so that the
// polygons are regular. But, you can arrange a set of regular polygons in the
// same pattern as a truncated cuboctahedron. It's just that the angles are all
// a bit different. That's this object.
pub fn truncated_cuboctahedron() -> Vec<Vec<[f64; 3]>> {
    let p = 1. / 2.;
    let q = (1. + (2. as f64).sqrt()) / 2.;
    let r = (1. + 2. * (2. as f64).sqrt()) / 2.;

    let v = [
	[-p,-q,-r],
	[ p,-q,-r],
	[-p, q,-r],
	[ p, q,-r],
	[-p,-q, r],
	[ p,-q, r],
	[-p, q, r],
	[ p, q, r],
	[-p,-r,-q],
	[ p,-r,-q],
	[-p,-r, q],
	[ p,-r, q],
	[-p, r,-q],
	[ p, r,-q],
	[-p, r, q],
	[ p, r, q],
	[-q,-p,-r],
	[-q, p,-r],
	[ q,-p,-r],
	[ q, p,-r],
	[-q,-p, r],
	[-q, p, r],
	[ q,-p, r],
	[ q, p, r],
	[-q,-r,-p],
	[-q,-r, p],
	[ q,-r,-p],
	[ q,-r, p],
	[-q, r,-p],
	[-q, r, p],
	[ q, r,-p],
	[ q, r, p],
	[-r,-p,-q],
	[-r, p,-q],
	[-r,-p, q],
	[-r, p, q],
	[ r,-p,-q],
	[ r, p,-q],
	[ r,-p, q],
	[ r, p, q],
	[-r,-q,-p],
	[-r,-q, p],
	[-r, q,-p],
	[-r, q, p],
	[ r,-q,-p],
	[ r,-q, p],
	[ r, q,-p],
	[ r, q, p],

    ];

    vec![
	vec![v[32], v[40], v[41], v[34], v[35], v[43], v[42], v[33]],
	vec![v[37], v[46], v[47], v[39], v[38], v[45], v[44], v[36]],
	vec![v[ 8], v[ 9], v[26], v[27], v[11], v[10], v[25], v[24]],
	vec![v[13], v[12], v[28], v[29], v[14], v[15], v[31], v[30]],
	vec![v[16], v[17], v[ 2], v[ 3], v[19], v[18], v[ 1], v[ 0]],
	vec![v[21], v[20], v[ 4], v[ 5], v[22], v[23], v[ 7], v[ 6]],
	vec![v[20], v[34], v[41], v[25], v[10], v[ 4]],
	vec![v[32], v[16], v[ 0], v[ 8], v[24], v[40]],
	vec![v[42], v[28], v[12], v[ 2], v[17], v[33]],
	vec![v[ 5], v[11], v[27], v[45], v[38], v[22]],
	vec![v[ 6], v[14], v[29], v[43], v[35], v[21]],
	vec![v[ 3], v[13], v[30], v[46], v[37], v[19]],
	vec![v[44], v[26], v[ 9], v[ 1], v[18], v[36]],
	vec![v[23], v[39], v[47], v[31], v[15], v[ 7]],
	vec![v[ 1], v[ 9], v[ 8], v[ 0]],
	vec![v[ 4], v[10], v[11], v[ 5]],
	vec![v[33], v[17], v[16], v[32]],
	vec![v[40], v[24], v[25], v[41]],
	vec![v[21], v[35], v[34], v[20]],
	vec![v[26], v[44], v[45], v[27]],
	vec![v[ 2], v[12], v[13], v[ 3]],
	vec![v[28], v[42], v[43], v[29]],
	vec![v[14], v[ 6], v[ 7], v[15]],
	vec![v[31], v[47], v[46], v[30]],
	vec![v[36], v[18], v[19], v[37]],
	vec![v[22], v[38], v[39], v[23]],
    ]

}

pub fn icosidodecahedron() -> Vec<Vec<[f64; 3]>> {
    let p = (1. + (5. as f64).sqrt())/2.;
    let q = 0.5;
    let r = p/2.;
    let s = p*p/2.;

    let v = [
	[-p,0.,0.],
	[ p,0.,0.],
	[0.,-p,0.],
	[0., p,0.],
	[0.,0.,-p],
	[0.,0., p],
	[-s,-r,-q],
	[ s,-r,-q],
	[-s, r,-q],
	[ s, r,-q],
	[-s,-r, q],
	[ s,-r, q],
	[-s, r, q],
	[ s, r, q],
	[-r,-q,-s],
	[-r,-q, s],
	[ r,-q,-s],
	[ r,-q, s],
	[-r, q,-s],
	[-r, q, s],
	[ r, q,-s],
	[ r, q, s],
	[-q,-s,-r],
	[-q, s,-r],
	[-q,-s, r],
	[-q, s, r],
	[ q,-s,-r],
	[ q, s,-r],
	[ q,-s, r],
	[ q, s, r],
    ];

    vec![
	vec![v[ 0], v[ 8], v[18], v[14], v[ 6]],
	vec![v[ 0], v[10], v[15], v[19], v[12]],
	vec![v[ 1], v[ 7], v[16], v[20], v[ 9]],
	vec![v[ 1], v[13], v[21], v[17], v[11]],
	vec![v[ 2], v[24], v[10], v[ 6], v[22]],
	vec![v[ 2], v[26], v[ 7], v[11], v[28]],
	vec![v[ 3], v[23], v[ 8], v[12], v[25]],
	vec![v[ 3], v[29], v[13], v[ 9], v[27]],
	vec![v[ 4], v[16], v[26], v[22], v[14]],
	vec![v[ 4], v[18], v[23], v[27], v[20]],
	vec![v[ 5], v[15], v[24], v[28], v[17]],
	vec![v[ 5], v[21], v[29], v[25], v[19]],
	vec![v[ 0], v[12], v[ 8]],
	vec![v[ 0], v[ 6], v[10]],
	vec![v[ 1], v[11], v[ 7]],
	vec![v[ 1], v[ 9], v[13]],
	vec![v[ 2], v[28], v[24]],
	vec![v[ 2], v[22], v[26]],
	vec![v[ 3], v[27], v[23]],
	vec![v[ 3], v[25], v[29]],
	vec![v[ 4], v[20], v[16]],
	vec![v[ 4], v[14], v[18]],
	vec![v[ 5], v[19], v[15]],
	vec![v[ 5], v[17], v[21]],

	vec![v[ 6], v[14], v[22]],
	vec![v[ 7], v[26], v[16]],
	vec![v[ 8], v[23], v[18]],
	vec![v[ 9], v[20], v[27]],
	vec![v[10], v[24], v[15]],
	vec![v[11], v[17], v[28]],
	vec![v[12], v[19], v[25]],
	vec![v[13], v[29], v[21]],
    ]
}

pub fn truncated_icosahedron() -> Vec<Vec<[f64; 3]>> {
    let edgelen_scale = 3.;

    let (v, p) = generate_icosahedron_mesh(edgelen_scale, SizeMode::EDGELEN);

    let mut ret = Vec::new();
    for l in p {
	let mut hex = Vec::new();
	hex.push(lerp(v[l[0]], v[l[1]], 1. / 3.));
	hex.push(lerp(v[l[0]], v[l[1]], 2. / 3.));
	hex.push(lerp(v[l[1]], v[l[2]], 1. / 3.));
	hex.push(lerp(v[l[1]], v[l[2]], 2. / 3.));
	hex.push(lerp(v[l[2]], v[l[0]], 1. / 3.));
	hex.push(lerp(v[l[2]], v[l[0]], 2. / 3.));
	ret.push(hex);
    }

    // order of the edges for the tri at each corner
    let corners = [
	[ 1,10, 5, 4, 8],
	[ 0, 8, 6, 7,10],
	[ 3, 9, 4, 5,11],
	[ 2,11, 7, 6, 9],
	[ 0, 5, 2, 9, 8],
	[ 0,10,11, 2, 4],
	[ 1, 8, 9, 3, 7],
	[ 1, 6, 3,11,10],
	[ 0, 4, 9, 6, 1],
	[ 2, 3, 6, 8, 4],
	[ 0, 1, 7,11, 5],
	[ 2, 5,10, 7, 3],
    ];
    for i in 0..corners.len() {
    	let mut tri = Vec::new();
    	tri.push(lerp(v[i],v[corners[i][0]],1./3.));
   	tri.push(lerp(v[i],v[corners[i][1]],1./3.));
    	tri.push(lerp(v[i],v[corners[i][2]],1./3.));
    	tri.push(lerp(v[i],v[corners[i][3]],1./3.));
    	tri.push(lerp(v[i],v[corners[i][4]],1./3.));
    	ret.push(tri);
    }
    ret
}

pub fn truncated_dodecahedron() -> Vec<Vec<[f64; 3]>> {
    let edgelen_scale = 1. / 0.44721359549995787;

    let (v, p) = generate_dodecahedron_mesh(edgelen_scale, SizeMode::EDGELEN);
    let t1 = 0.276393202250021; // 1 / 2*(1 + cos(36))
    let t2 = 1. - t1;

    let mut ret = Vec::new();
    for l in p {
	let mut hex = Vec::new();
	hex.push(lerp(v[l[0]], v[l[1]], t1));
	hex.push(lerp(v[l[0]], v[l[1]], t2));
	hex.push(lerp(v[l[1]], v[l[2]], t1));
	hex.push(lerp(v[l[1]], v[l[2]], t2));
	hex.push(lerp(v[l[2]], v[l[3]], t1));
	hex.push(lerp(v[l[2]], v[l[3]], t2));
	hex.push(lerp(v[l[3]], v[l[4]], t1));
	hex.push(lerp(v[l[3]], v[l[4]], t2));
	hex.push(lerp(v[l[4]], v[l[0]], t1));
	hex.push(lerp(v[l[4]], v[l[0]], t2));
	ret.push(hex);
    }

    // order of the edges for the tri at each corner
    let corners = [
	[ 8,16,12],
	[ 8,14,17],
	[18, 9,12],
	[19,14, 9],
	[16,10,13],
	[15,10,17],
	[13,11,18],
	[11,15,19],
	[ 0, 1,10],
	[ 2,11, 3],
	[ 4, 8, 5],
	[ 6, 7, 9],
	[ 2,14, 0],
	[ 4,15, 6],
	[ 3, 1,12],
	[13, 5, 7],
	[ 0, 4,18],
	[19, 5, 1],
	[16, 6, 2],
	[ 7,17, 3],
    ];

    for i in 0..corners.len() {
    	let mut tri = Vec::new();
    	tri.push(lerp(v[i],v[corners[i][0]],t1));
    	tri.push(lerp(v[i],v[corners[i][1]],t1));
    	tri.push(lerp(v[i],v[corners[i][2]],t1));
    	ret.push(tri);
    }
    ret
}

pub fn rhombicosidodecahedron() -> Vec<Vec<[f64; 3]>> {
    let phi = (1. + (5. as f64).sqrt())/2.;

    let q = 1. / 2.;
    let p2 = phi*phi / 2.;
    let p3 = phi*phi*phi / 2.;
    let r = 2.*phi / 2.;
    let s = (2. + phi) / 2.;
    let t = phi / 2.;

    let v = [
	[ -q, -q,-p3],
	[  q, -q,-p3],
	[ -q,  q,-p3],
	[  q,  q,-p3],
	[ -q, -q, p3],
	[  q, -q, p3],
	[ -q,  q, p3],
	[  q,  q, p3],
	[-p3, -q, -q],
	[-p3,  q, -q],
	[-p3, -q,  q],
	[-p3,  q,  q],
	[ p3, -q, -q],
	[ p3,  q, -q],
	[ p3, -q,  q],
	[ p3,  q,  q],
	[ -q,-p3, -q],
	[ -q,-p3,  q],
	[  q,-p3, -q],
	[  q,-p3,  q],
	[ -q, p3, -q],
	[ -q, p3,  q],
	[  q, p3, -q],
	[  q, p3,  q],
	[-p2, -t, -r],
	[ p2, -t, -r],
	[-p2,  t, -r],
	[ p2,  t, -r],
	[-p2, -t,  r],
	[ p2, -t,  r],
	[-p2,  t,  r],
	[ p2,  t,  r],
	[ -r,-p2, -t],
	[ -r, p2, -t],
	[ -r,-p2,  t],
	[ -r, p2,  t],
	[  r,-p2, -t],
	[  r, p2, -t],
	[  r,-p2,  t],
	[  r, p2,  t],
	[ -t, -r,-p2],
	[ -t, -r, p2],
	[  t, -r,-p2],
	[  t, -r, p2],
	[ -t,  r,-p2],
	[ -t,  r, p2],
	[  t,  r,-p2],
	[  t,  r, p2],
	[ -s,0.0,-p2],
	[  s,0.0,-p2],
	[ -s,0.0, p2],
	[  s,0.0, p2],
	[-p2, -s,0.0],
	[-p2,  s,0.0],
	[ p2, -s,0.0],
	[ p2,  s,0.0],
	[0.0,-p2, -s],
	[0.0,-p2,  s],
	[0.0, p2, -s],
	[0.0, p2,  s],
    ];

    vec![
	vec![v[ 2], v[ 3], v[ 1], v[ 0]],
	vec![v[ 5], v[ 7], v[ 6], v[ 4]],
	vec![v[10], v[11], v[ 9], v[ 8]],
	vec![v[13], v[15], v[14], v[12]],
	vec![v[18], v[19], v[17], v[16]],
	vec![v[21], v[23], v[22], v[20]],

	vec![v[ 1], v[56], v[ 0]],
	vec![v[ 2], v[58], v[ 3]],
	vec![v[ 7], v[59], v[ 6]],
	vec![v[ 4], v[57], v[ 5]],
	vec![v[ 9], v[48], v[ 8]],
	vec![v[10], v[50], v[11]],
	vec![v[12], v[49], v[13]],
	vec![v[15], v[51], v[14]],
	vec![v[19], v[18], v[54]],
	vec![v[16], v[17], v[52]],
	vec![v[22], v[23], v[55]],
	vec![v[21], v[20], v[53]],

	vec![v[24], v[48], v[26], v[ 2], v[ 0]],
	vec![v[27], v[49], v[25], v[ 1], v[ 3]],
	vec![v[ 6], v[30], v[50], v[28], v[ 4]],
	vec![v[ 5], v[29], v[51], v[31], v[ 7]],
	vec![v[32], v[52], v[34], v[10], v[ 8]],
	vec![v[35], v[53], v[33], v[ 9], v[11]],
	vec![v[12], v[14], v[38], v[54], v[36]],
	vec![v[15], v[13], v[37], v[55], v[39]],
	vec![v[40], v[56], v[42], v[18], v[16]],
	vec![v[17], v[19], v[43], v[57], v[41]],
	vec![v[46], v[58], v[44], v[20], v[22]],
	vec![v[23], v[21], v[45], v[59], v[47]],

    	vec![v[24], v[ 0], v[56], v[40]],
    	vec![v[ 1], v[25], v[42], v[56]],
	vec![v[ 3], v[58], v[46], v[27]],
	vec![v[ 2], v[26], v[44], v[58]],
	vec![v[43], v[19], v[54], v[38]],
	vec![v[23], v[47], v[39], v[55]],
	vec![v[54], v[18], v[42], v[36]],
	vec![v[52], v[17], v[41], v[34]],
	vec![v[32], v[40], v[16], v[52]],
	vec![v[25], v[49], v[12], v[36]],
	vec![v[37], v[13], v[49], v[27]],
	vec![v[29], v[38], v[14], v[51]],
	vec![v[44], v[33], v[53], v[20]],
	vec![v[46], v[22], v[55], v[37]],
	vec![v[33], v[26], v[48], v[ 9]],
	vec![v[48], v[24], v[32], v[ 8]],
	vec![v[35], v[45], v[21], v[53]],
	vec![v[30], v[35], v[11], v[50]],
	vec![v[47], v[59], v[ 7], v[31]],
	vec![v[28], v[50], v[10], v[34]],
	vec![v[43], v[29], v[ 5], v[57]],
	vec![v[ 6], v[59], v[45], v[30]],
	vec![v[31], v[51], v[15], v[39]],
	vec![v[41], v[57], v[ 4], v[28]],

	vec![v[24], v[40], v[32]],
	vec![v[33], v[44], v[26]],
	vec![v[46], v[37], v[27]],
	vec![v[42], v[25], v[36]],
	vec![v[43], v[38], v[29]],
	vec![v[45], v[35], v[30]],
	vec![v[39], v[47], v[31]],
	vec![v[28], v[34], v[41]],
]
}

pub fn snub_cube() -> Vec<Vec<[f64; 3]>> {
    let t = 1.839286755214161; // tribonacci constant

    let a = 1.;
    let b = 1. / t;
    let c = t;

    // This one uses the even permutations with an even number of
    // plus signs and the odd permutations with an odd number.
    // There's a 2nd snub cube that uses the other vertices.
    let v = vec![
	// even permutations
	[-a,-b,-c], //e ABC v00
	[ a,-b,-c], //o
	[-a, b,-c], //o
	[ a, b,-c], //e     v03
	[-a,-b, c], //o
	[ a,-b, c], //e     v05
	[-a, b, c], //e     v06
	[ a, b, c], //o
	[-c,-a,-b], //e CAB v08
	[-c, a,-b], //o
	[-c,-a, b], //o
	[-c, a, b], //e     v11
	[ c,-a,-b], //o
	[ c, a,-b], //e     v13
	[ c,-a, b], //e     v14
	[ c, a, b], //o
	[-b,-c,-a], //e BCA v16
	[-b,-c, a], //o
	[ b,-c,-a], //o
	[ b,-c, a], //e     v19
	[-b, c,-a], //o
	[-b, c, a], //e     v21
	[ b, c,-a], //e     v22
	[ b, c, a], //o
	// odd permutations
	[-b,-a,-c], //e BAC
	[-b, a,-c], //o     v25
	[ b,-a,-c], //o     v26
	[ b, a,-c], //e
	[-b,-a, c], //o     v28
	[-b, a, c], //e
	[ b,-a, c], //e
	[ b, a, c], //o     v31
	[-a,-c,-b], //e ACB
	[ a,-c,-b], //o     v33
	[-a,-c, b], //o     v34
	[ a,-c, b], //e
	[-a, c,-b], //o     v36
	[ a, c,-b], //e
	[-a, c, b], //e
	[ a, c, b], //o     v39
	[-c,-b,-a], //e CBA
	[-c,-b, a], //o     v41
	[-c, b,-a], //o     v42
	[-c, b, a], //e
	[ c,-b,-a], //o     v44
	[ c,-b, a], //e
	[ c, b,-a], //e
	[ c, b, a], //o     v47
	];

    vec![
	vec![v[ 8], v[41], v[11], v[42]], // x = -c
	vec![v[13], v[47], v[14], v[44]], // x = +c
	vec![v[16], v[33], v[19], v[34]], // y = -c
	vec![v[21], v[39], v[22], v[36]], // y = +c
	vec![v[ 0], v[25], v[ 3], v[26]], // z = -c
	vec![v[ 5], v[31], v[ 6], v[28]], // z = +c
	// around square 0
	vec![v[ 8], v[34], v[41]],
	vec![v[41], v[ 6], v[11]],
	vec![v[11], v[36], v[42]],
	vec![v[42], v[ 0], v[ 8]],
	// around square 1
	vec![v[13], v[39], v[47]],
	vec![v[47], v[ 5], v[14]],
	vec![v[14], v[33], v[44]],
	vec![v[44], v[ 3], v[13]],
	// around square 2
	vec![v[16], v[26], v[33]],
	vec![v[33], v[14], v[19]],
	vec![v[19], v[28], v[34]],
	vec![v[34], v[ 8], v[16]],
	// around square 3
	vec![v[21], v[31], v[39]],
	vec![v[39], v[13], v[22]],
	vec![v[22], v[25], v[36]],
	vec![v[36], v[11], v[21]],
	// around square 4
	vec![v[ 0], v[42], v[25]],
	vec![v[25], v[22], v[ 3]],
	vec![v[ 3], v[44], v[26]],
	vec![v[26], v[16], v[ 0]],
	// around square 5
	vec![v[ 5], v[47], v[31]],
	vec![v[31], v[21], v[ 6]],
	vec![v[ 6], v[41], v[28]],
	vec![v[28], v[19], v[ 5]],

	// corners
	vec![v[ 0], v[16], v[ 8]],
	vec![v[11], v[ 6], v[21]],
	vec![v[13], v[ 3], v[22]],
	vec![v[14], v[ 5], v[19]],
	vec![v[41], v[34], v[28]],
	vec![v[42], v[36], v[25]],
	vec![v[44], v[33], v[26]],
	vec![v[47], v[39], v[31]],
    ]
}

// The "small" or "first" stellation of the dodecahedron.
pub fn stellated_dodecahedron() -> Vec<Vec<[f64; 3]>> {
    let edgelen_scale = 1.;

    let (v, p) = generate_dodecahedron_mesh(edgelen_scale, SizeMode::EDGELEN);

    let alt = edgelen_scale * 1.584599 / 1.236068;

    let mut ret = Vec::new();
    for l in p {
	let pent = vec![v[l[0]], v[l[1]], v[l[2]], v[l[3]], v[l[4]]];
	let c = centroid(&pent);
	let n = normalized(normal(&pent));
	let a = [c[0] + alt * n[0],
		 c[1] + alt * n[1],
		 c[2] + alt * n[2]];
	for i in 0..5 {
	    let mut hex = Vec::new();
	    hex.push(pent[(i+1)%5]);
	    hex.push(a);
	    hex.push(pent[i]);
	    ret.push(hex);
	}
    }
    ret
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

fn edge_lengths(src: &Vec<Vec<[f64; 3]>>) -> [f64; 2] {
    let mut min = 1000. as f64;
    let mut max = -1000. as f64;

    for p in src {
        let n = p.len();
        for i in 0..n {
            let j = (i+1) % n;
            let dx = p[i][0] - p[j][0];
            let dy = p[i][1] - p[j][1];
            let dz = p[i][2] - p[j][2];
            let d = (dx*dx + dy*dy + dz*dz).sqrt();
            min = min.min(d);
            max = max.max(d);
        }
    }
    [min, max]
}

fn radii(src: &Vec<Vec<[f64; 3]>>) -> [f64; 2] {
    let mut min = 1000. as f64;
    let mut max = -1000. as f64;

    for p in src {
	for v in p {
            let d = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
            min = min.min(d);
            max = max.max(d);
        }
    }
    [min, max]
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

fn normalized(v: [f64; 3]) -> [f64;3] {
    let l2 = dot(v, v);
    let s = 1. / l2.sqrt();
    [v[0] * s, v[1] * s, v[2] * s]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

fn lerp(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [(1.-t)*a[0] + t*b[0],
     (1.-t)*a[1] + t*b[1],
     (1.-t)*a[2] + t*b[2]]
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
    fn test_tetrahedron_edgelength() {
        let verts = tetrahedron();
        let r = edge_lengths(&verts);
	let expected = 1.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_tetrahedron_radii() {
        let verts = tetrahedron();
        let r = radii(&verts);
	let expected = 0.6123724356957945;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
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
    fn test_cube_edgelength() {
        let verts = cube();
        let r = edge_lengths(&verts);
        assert_eq!(r[0], 1.);
        assert_eq!(r[1], 1.);
    }

    #[test]
    fn test_cube_radii() {
        let verts = cube();
        let r = radii(&verts);
	let expected = (3. as f64).sqrt() / 2.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
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
    fn test_octahedron_edgelength() {
        let verts = octahedron();
        let r = edge_lengths(&verts);
//	let expected = (2. as f64).sqrt() / 2.;
	let expected = 1.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_octahedron_radii() {
        let verts = octahedron();
        let r = radii(&verts);
	let expected = (2. as f64).sqrt() / 2.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
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
    fn test_icosahedron_edgelength() {
        let verts = icosahedron();
        let r = edge_lengths(&verts);
        assert_eq!(r[0], 1.);
        assert_eq!(r[1], 1.);
    }

    #[test]
    fn test_icosahedron_radii() {
        let verts = icosahedron();
        let r = radii(&verts);
	let expected = 0.9510565162951535;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
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
    fn test_dodecahedron_edgelength() {
        let verts = dodecahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_dodecahedron_radii() {
        let verts = dodecahedron();
        let r = radii(&verts);
	let expected = 1.4012585384440734;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    // 8 polys. 4 with 3 sides, and 4 with 6 sides.
    fn test_truncated_tetrahedron_size() {
        let verts = truncated_tetrahedron();
        assert_eq!(verts.len(), 8);
        let mut num_hexes = 0;
        let mut num_tris = 0;
        for p in verts {
            match p.len() {
                6 => { num_hexes += 1 }
                3 => { num_tris += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_hexes == 4);
        assert!(num_tris == 4);
    }

    #[test]
    fn test_truncated_tetrahedron_orientation() {
	// 3rd triangle is backwards
        let verts = truncated_tetrahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
	    assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_tetrahedron_edgelength() {
        let verts = truncated_tetrahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_truncated_tetrahedron_radii() {
        let verts = truncated_tetrahedron();
        let r = radii(&verts);
	let expected = 1.1726039399558574;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_truncated_cube_size() {
        let verts = truncated_cube();
        assert_eq!(verts.len(), 14);
        let mut num_octs = 0;
        let mut num_tris = 0;
        for p in verts {
            match p.len() {
                8 => { num_octs += 1 }
                3 => { num_tris += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_octs == 6);
        assert!(num_tris == 8);
    }

    #[test]
    fn test_truncated_cube_orientation() {
        let verts = truncated_cube();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
	    assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_cube_edgelength() {
        let verts = truncated_cube();
        let r = edge_lengths(&verts);
	let expected = 0.8786796564403572;
	let tol = 1.0e-15;
        assert_abs_diff_eq!(r[0], expected, epsilon = tol);
        assert_abs_diff_eq!(r[1], expected, epsilon = tol);
    }

    #[test]
    fn test_truncated_truncated_cube_radii() {
        let verts = truncated_cube();
        let r = radii(&verts);
	let expected = 1.5630161498399613;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_truncated_octahedron_size() {
        let verts = truncated_octahedron();
        assert_eq!(verts.len(), 14);
        let mut num_hexes = 0;
        let mut num_squares = 0;
        for p in verts {
            match p.len() {
                6 => { num_hexes += 1 }
                4 => { num_squares += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_hexes == 8);
        assert!(num_squares == 6);
    }

    #[test]
    fn test_truncated_octahedron_orientation() {
        let verts = truncated_octahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
	    assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_octahedron_edgelength() {
        let verts = truncated_octahedron();
        let r = edge_lengths(&verts);
	let expected = 0.5;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_truncated_truncated_octahedron_radii() {
        let verts = truncated_octahedron();
        let r = radii(&verts);
	let expected = 0.790569415042095;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
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
    fn test_rhombic_dodecahedron_edgelength() {
        let verts = rhombic_dodecahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_rhombic_dodecahedron_radii() {
        let verts = rhombic_dodecahedron();
        let r = radii(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.1547005383792517);
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

    #[test]
    fn test_rhombic_triacontahedron_edgelength() {
        let verts = rhombic_triacontahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_rhombic_triacontahedron_radii() {
        let verts = rhombic_triacontahedron();
        let r = radii(&verts);
        assert_abs_diff_eq!(r[0], 1.4733704195652688);
        assert_abs_diff_eq!(r[1], ((5. as f64).sqrt() + 1.) / 2.);
    }

    #[test]
    // 14 polys. 8 with 3 sides, and 6 with 4 sides.
    fn test_cuboctahedron_size() {
        let verts = cuboctahedron();
        assert_eq!(verts.len(), 14);

        let mut num_squares = 0;
        let mut num_tris = 0;
        for p in verts {
            match p.len() {
                4 => { num_squares += 1 }
                3 => { num_tris += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_squares == 6);
        assert!(num_tris == 8);
    }

    #[test]
    fn test_cuboctahedron_orientation() {
        let verts = cuboctahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_cuboctahedron_edgelength() {
        let verts = cuboctahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_cuboctahedron_radii() {
        let verts = cuboctahedron();
        let r = radii(&verts);
	let expected = 1.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    // 26 polys. 8 with 3 sides, and 18 with 4 sides.
    fn test_rhombicuboctahedron_size() {
        let verts = rhombicuboctahedron();
        assert_eq!(verts.len(), 26);

        let mut num_squares = 0;
        let mut num_tris = 0;
        for p in verts {
            match p.len() {
                4 => { num_squares += 1 }
                3 => { num_tris += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_squares == 18);
        assert!(num_tris == 8);
    }

    #[test]
    fn test_rhombicuboctahedron_orientation() {
        let verts = rhombicuboctahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_rhombicuboctahedron_edgelength() {
        let verts = rhombicuboctahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_rhombicuboctahedron_radii() {
        let verts = rhombicuboctahedron();
        let r = radii(&verts);
	let expected = 1.3989663259659066;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    // 26 polys. 8 with 3 sides, and 18 with 4 sides.
    fn test_truncated_cuboctahedron_size() {
        let verts = truncated_cuboctahedron();
        assert_eq!(verts.len(), 26);

        let mut num_squares = 0;
        let mut num_hexes = 0;
        let mut num_octs = 0;
        for p in verts {
            match p.len() {
                4 => { num_squares += 1 }
                6 => { num_hexes += 1 }
                8 => { num_octs += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_squares == 12);
        assert!(num_hexes == 8);
        assert!(num_octs == 6);
    }

    #[test]
    fn test_truncated_cuboctahedron_orientation() {
        let verts = truncated_cuboctahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_cuboctahedron_edgelength() {
        let verts = truncated_cuboctahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_truncated_cuboctahedron_radii() {
        let verts = truncated_cuboctahedron();
        let r = radii(&verts);
	let expected = 2.3176109128927664;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_icosidodecahedron_size() {
        let verts = icosidodecahedron();
        assert_eq!(verts.len(), 32);

        let mut num_tris = 0;
        let mut num_pents = 0;
        for p in verts {
            match p.len() {
                3 => { num_tris += 1 }
                5 => { num_pents += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_pents == 12);
        assert!(num_tris == 20);
    }

    #[test]
    fn test_icosidodecahedron_orientation() {
        let verts = icosidodecahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_icosidodecahedron_edgelength() {
        let verts = icosidodecahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_icosidodecahedron_radii() {
        let verts = icosidodecahedron();
        let r = radii(&verts);
    	let expected =  (1. + (5. as f64).sqrt()) / 2.;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_truncated_icosahedron_size() {
	let verts = truncated_icosahedron();
        assert_eq!(verts.len(), 32);

        let mut num_hexes = 0;
        let mut num_pents = 0;
        for p in verts {
            match p.len() {
                6 => { num_hexes += 1 }
                5 => { num_pents += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_pents == 12);
        assert!(num_hexes == 20);
    }

    #[test]
    fn test_truncated_icosahedron_orientation() {
        let verts = truncated_icosahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_icosahedron_edgelength() {
        let verts = truncated_icosahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_truncated_icosahedron_radii() {
        let verts = truncated_icosahedron();
        let r = radii(&verts);
    	let expected = 2.4780186590676156;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_truncated_dodecahedron_size() {
	let verts = truncated_dodecahedron();
        assert_eq!(verts.len(), 32);

        let mut num_decs = 0;
        let mut num_tris = 0;
        for p in verts {
            match p.len() {
                10 => { num_decs += 1 }
                3 => { num_tris += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_decs == 12);
	assert!(num_tris == 20);
    }

    #[test]
    fn test_truncated_dodecahedron_orientation() {
        let verts = truncated_dodecahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_truncated_dodecahedron_edgelength() {
        let verts = truncated_dodecahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_truncated_dodecahedron_radii() {
        let verts = truncated_dodecahedron();
        let r = radii(&verts);
    	let expected = 2.9694490158633986;
	let tol = 1.0e-15;
        assert_abs_diff_eq!(r[0], expected, epsilon = tol);
        assert_abs_diff_eq!(r[1], expected, epsilon = tol);
    }

    #[test]
    fn test_rhombicosidodecahedron_size() {
	let verts = rhombicosidodecahedron();
        assert_eq!(verts.len(), 62);

        let mut num_tris = 0;
        let mut num_squares = 0;
        let mut num_pents = 0;
        for p in verts {
            match p.len() {
                3 => { num_tris += 1 }
                4 => { num_squares += 1 }
                5 => { num_pents += 1 }
                _ => { assert!(false) }
            }
        }
        assert!(num_pents == 12);
	assert!(num_squares == 30);
	assert!(num_tris == 20);
    }

    #[test]
    fn test_rhombicosidodecahedron_orientation() {
        let verts = rhombicosidodecahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_rhombicosidodecahedron_edgelength() {
        let verts = rhombicosidodecahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
    }

    #[test]
    fn test_rhombicosidodecahedron_radii() {
        let verts = rhombicosidodecahedron();
        let r = radii(&verts);
    	let expected = 2.2329505094156903;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    // #[test]
    fn test_snub_cube_size() {
    	let verts = snub_cube();
        assert_eq!(verts.len(), 38);

        let mut num_tris = 0;
        let mut num_squares = 0;
        for p in verts {
            match p.len() {
                3 => { num_tris += 1 }
                4 => { num_squares += 1 }
                _ => { assert!(false) }
            }
        }
    	assert!(num_squares == 6);
    	assert!(num_tris == 32);
    }

    #[test]
    fn test_snub_cube_orientation() {
        let verts = snub_cube();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_snub_cube_edgelength() {
        let verts = snub_cube();
        let r = edge_lengths(&verts);
	let expected = 1.6097190702244193;
        assert_abs_diff_eq!(r[0], expected);
        assert_abs_diff_eq!(r[1], expected);
    }

    #[test]
    fn test_snub_cube_radii() {
        let verts = snub_cube();
        let r = radii(&verts);
    	let expected = 2.1630010426322777;
	let tol = 1.0e-15;
        assert_abs_diff_eq!(r[0], expected, epsilon = tol);
        assert_abs_diff_eq!(r[1], expected, epsilon = tol);
    }

    // #[test]
    fn test_stellated_dodecahedron_size() {
    	let verts = stellated_dodecahedron();
        assert_eq!(verts.len(), 60);

        for p in verts {
	    assert_eq!(p.len(), 3);
        }
    }

    #[test]
    fn test_stellated_dodecahedron_orientation() {
        let verts = stellated_dodecahedron();
        for p in verts {
            let c = centroid(&p);
            let n = normal(&p);
            assert!(dot(c,n) > 0.);
        }
    }

    #[test]
    fn test_stellated_dodecahedron_edgelength() {
        let verts = stellated_dodecahedron();
        let r = edge_lengths(&verts);
	let expected_min = 1.;
	let expected_max = 1.5385211928812546;
	let tol = 1.0e-15;
        assert_abs_diff_eq!(r[0], expected_min, epsilon = tol);
        assert_abs_diff_eq!(r[1], expected_max, epsilon = tol);
    }

    #[test]
    fn test_stellated_dodecahedron_radii() {
        let verts = stellated_dodecahedron();
        let r = radii(&verts);
    	let expected_min = 1.4012585384440734;
    	let expected_max = 2.395483861345433;
	let tol = 1.0e-15;
        assert_abs_diff_eq!(r[0], expected_min, epsilon = tol);
        assert_abs_diff_eq!(r[1], expected_max, epsilon = tol);
    }

}

fn generate_tetrahedron_mesh(l: f64, sm: SizeMode) -> (Vec<[f64;3]>, Vec<Vec<usize>>) {
    let s = 0.5 * l;
    let v = vec![
        [ s, s,-s],
        [-s, s, s],
        [-s,-s,-s],
        [ s,-s, s] ];

    let p = vec![
	vec![2, 1, 0],
	vec![2, 0, 3],
	vec![3, 0, 1],
	vec![3, 1, 2] ];

    (v, p)
}

fn generate_cube_mesh(l: f64, sm: SizeMode) -> (Vec<[f64;3]>, Vec<Vec<usize>>) {
    let s = 0.5 * l;
    let v = vec![
        [-s,-s,-s],
        [ s,-s,-s],
        [-s, s,-s],
        [ s, s,-s],
        [-s,-s, s],
        [ s,-s, s],
        [-s, s, s],
        [ s, s, s] ];

    let p = vec![
	vec![4, 5, 7, 6],
	vec![5, 1, 3, 7],
	vec![1, 0, 2, 3],
	vec![0, 4, 6, 2],
	vec![6, 7, 3, 2],
	vec![0, 1, 5, 4] ];

    (v, p)
}

fn generate_octahedron_mesh(l: f64, sm: SizeMode) -> (Vec<[f64;3]>, Vec<Vec<usize>>) {
    let s = 0.5 * l;
    let v = vec![
        [-s,0.,0.],
        [ s,0.,0.],
        [0.,-s,0.],
        [0., s,0.],
        [0.,0.,-s],
        [0.,0., s] ];

    let p = vec![
	vec![2, 1, 5],
	vec![2, 4, 1],
	vec![2, 0, 4],
	vec![2, 5, 0],
	vec![1, 3, 5],
	vec![5, 3, 0],
	vec![0, 3, 4],
	vec![4, 3, 1] ];

    (v, p)
}

fn generate_icosahedron_mesh(l: f64, sm: SizeMode) -> (Vec<[f64;3]>, Vec<Vec<usize>>) {
    let r = l * 0.5;
    let p = l * (1. + (5 as f64).sqrt()) / 4.;
    let v = vec![
        [ 0.,-r, -p],
        [ 0., r, -p],
        [ 0.,-r,  p],
        [ 0., r,  p],
        [-r, -p, 0.],
        [ r, -p, 0.],
        [-r,  p, 0.],
        [ r,  p, 0.],
        [ -p, 0.,-r],
        [ -p, 0., r],
        [  p, 0.,-r],
        [  p, 0., r] ];

    let p = vec![
	vec![ 3, 2,11 ],
	vec![ 3,11, 7 ],
	vec![ 3, 7, 6 ],
	vec![ 3, 6, 9 ],
	vec![ 3, 9, 2 ],
	vec![ 2, 5,11 ],
	vec![ 5,10,11 ],
	vec![11,10, 7 ],
	vec![10, 1, 7 ],
	vec![ 7, 1, 6 ],
	vec![ 1, 8, 6 ],
	vec![ 6, 8, 9 ],
	vec![ 8, 4, 9 ],
	vec![ 9, 4, 2 ],
	vec![ 4, 5, 2 ],
	vec![ 0, 1,10 ],
	vec![ 0,10, 5 ],
	vec![ 0, 5, 4 ],
	vec![ 0, 4, 8 ],
	vec![ 0, 8, 1 ],
    ];
    (v, p)
}

fn generate_dodecahedron_mesh(l: f64, sm: SizeMode) -> (Vec<[f64;3]>, Vec<Vec<usize>>) {
    // @todo clean this up
    // edge length = (5. as f64).sqrt() - 1.
    let _radius_edgelen_ratio = 1.4012585384440734;
    let p = (1. + (5. as f64).sqrt()) / 2.;
    let i = 1.0 / p;

    let d = (5. as f64).sqrt() - 1.;
    let r = l / d;
    let q = l * p / d;
    let s = l * i / d;

    let v = vec![
        [-r,-r,-r],
        [ r,-r,-r],
        [-r, r,-r],
        [ r, r,-r],
        [-r,-r, r],
        [ r,-r, r],
        [-r, r, r],
        [ r, r, r],
        [0.,-q,-s],
        [0., q,-s],
        [0.,-q, s],
        [0., q, s],
        [-s,0.,-q],
        [-s,0., q],
        [ s,0.,-q],
        [ s,0., q],
        [-q,-s,0.],
        [ q,-s,0.],
        [-q, s,0.],
        [ q, s,0.] ];

    let p = vec![
	vec![10, 5,15,13, 4],
	vec![15, 7,11, 6,13],
	vec![ 5,17,19, 7,15],
	vec![16, 4,13, 6,18],
	vec![14, 1, 8, 0,12],
	vec![ 8, 1,17, 5,10],
	vec![ 1,14, 3,19,17],
	vec![ 8,10, 4,16, 0],
	vec![ 9, 3,14,12, 2],
	vec![11, 7,19, 3, 9],
	vec![ 6,11, 9, 2,18],
	vec![ 2,12, 0,16,18],
    ];
    (v, p)
}

