pub fn tetrahedron() -> Vec<Vec<[f64; 3]>> {
    // edge length = 2. * (2. as f64).sqrt();
    let s = 0.5 / (2. as f64).sqrt();
    let v = [
        [-s,-s,-s],
        [-s, s, s],
        [ s, s,-s],
        [ s,-s, s] ];
    [ vec![v[0], v[1], v[2] ],
      vec![v[0], v[2], v[3] ],
      vec![v[3], v[2], v[1] ],
      vec![v[3], v[1], v[0] ],
    ].to_vec()
}

pub fn cube() -> Vec<Vec<[f64; 3]>> {
    // edge length = 2.
    let s = 0.5;
    let v = [
        [-s,-s,-s],
        [ s,-s,-s],
        [-s, s,-s],
        [ s, s,-s],
        [-s,-s, s],
        [ s,-s, s],
        [-s, s, s],
        [ s, s, s] ];
    [ vec![v[4], v[5], v[7], v[6] ],
      vec![v[5], v[1], v[3], v[7] ],
      vec![v[1], v[0], v[2], v[3] ],
      vec![v[0], v[4], v[6], v[2] ],
      vec![v[6], v[7], v[3], v[2] ],
      vec![v[0], v[1], v[5], v[4] ],
    ].to_vec()
}

pub fn octahedron() -> Vec<Vec<[f64; 3]>> {
    // edge length = (2. as f64).sqrt()
    let s = 1. / (2. as f64).sqrt();
    let v = [
        [-s,0.,0.],
        [ s,0.,0.],
        [0.,-s,0.],
        [0., s,0.],
        [0.,0.,-s],
        [0.,0., s] ];

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
    // edge length = 2.
    let r = 0.5;
    let p = (1. + (5 as f64).sqrt()) / 4.;
    let v = [
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
    // @todo clean this up
    // edge length = (5. as f64).sqrt() - 1.
    let p = (1. + (5. as f64).sqrt()) / 2.;
    let i = 1.0 / p;

    let d = (5. as f64).sqrt() - 1.;
    let r = 1. / d;
    let q = p / d;
    let s = i / d;

    let v = [
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
    // edge length = (3. as f64).sqrt() / 2.
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

pub fn cuboctahedron() -> Vec<Vec<[f64; 3]>> {
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

pub fn rhombicuboctahedron() -> Vec<Vec<[f64; 3]>> {
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

// pub fn rhombicuboctahedron() -> Vec<Vec<[f64; 3]>> {
//     let q = 1. + (2. as f64).sqrt();

//     let v = [
//      [-1.,-q ,-q ],
//      [-q ,-q ,-1.],
//      [-q ,-q , 1.],
//      [-1.,-q , q ],
//      [ 1.,-q , q ],
//      [ q ,-q , 1.],
//      [ q ,-q ,-1.],
//      [ 1.,-q ,-q ],

//      [-q ,-1.,-q ],
//      [-q ,-1., q ],
//      [ q ,-1., q ],
//      [ q ,-1.,-q ],
//      [-q , 1.,-q ],
//      [-q , 1., q ],
//      [ q , 1., q ],
//      [ q , 1.,-q ],

//      [-1., q ,-q ],
//      [-q , q ,-1.],
//      [-q , q , 1.],
//      [-1., q , q ],
//      [ 1., q , q ],
//      [ q , q , 1.],
//      [ q , q ,-1.],
//      [ 1., q ,-q ] ];

//     [ vec![v[ 7], v[ 6], v[ 5], v[ 4], v[ 3], v[ 2], v[ 1], v[ 0]],
//       vec![v[ 1], v[ 2], v[ 9], v[13], v[18], v[17], v[12], v[ 8]],
//       vec![v[ 3], v[ 4], v[10], v[14], v[20], v[19], v[13], v[ 9]],
//       vec![v[ 5], v[ 6], v[11], v[15], v[22], v[21], v[14], v[10]],
//       vec![v[ 7], v[ 0], v[ 8], v[12], v[16], v[23], v[15], v[11]],
//       vec![v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]],
//     ].to_vec()
// }

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
    fn test_tetrahedron_edgelength() {
        let verts = tetrahedron();
        let r = edge_lengths(&verts);
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
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
        assert_abs_diff_eq!(r[0], 1.);
        assert_abs_diff_eq!(r[1], 1.);
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
}
