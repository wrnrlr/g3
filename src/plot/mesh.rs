use crate::*;
use glam::Vec3;
use rend3::types::{Handedness, Mesh, MeshBuilder};


pub fn plane_mesh(p:&Plane)->Mesh {
  let p = p.normalized(); let m = (p*E2).sqrt();
  let a:Vec3 = m(point(-1.0,0.0,-1.0)).into(); let b:Vec3 = m(point(-1.0,0.0,1.0)).into();
  let c:Vec3 = m(point(1.0,0.0,1.0)).into(); let d:Vec3 = m(point(1.0,0.0,-1.0)).into();
  let vertices:Vec<Vec3> = vec!(a, b, c, d, a, b, c, d);
  let indices = vec!(0u32, 1, 2, 2, 3, 0, 4, 7, 6, 6, 5, 4);
  MeshBuilder::new(vertices, Handedness::Left).with_indices(indices.to_vec()).build().unwrap()
}

pub fn point_mesh(_center:&Point,radius:f32)->Mesh {
  // let r = rotor(PI/2.5, 0.0, 1.0, 0.0); // rotate around x axis
  // let a = point(0.0,1.0,0.0);
  // let b = point((1.0-0.5f32.atan().powi(2)).sqrt(), 0.5f32.atan(), 0.0);
  // let c = rotor(PI/5.0, 0.0, 1.0, 0.0)(E2(b)).normalized();
  //
  // let a1 = E2(a).normalized();
  // let b1 = r(b).normalized(); let c1 = r(c).normalized();
  // let b2 = r(b1).normalized(); let c2 = r(c1).normalized();
  // let b3 = r(b2).normalized(); let c3 = r(c2).normalized();
  // let b4 = r(b3).normalized(); let c4 = r(c3).normalized();
  // let b5 = r(b4).normalized(); let c5 = r(c4).normalized();
  //
  // let vertices:Vec<Vec3> = vec!(
  //   a.into(), b.into(), b1.into(),
  //   c.into(), c1.into(), a1.into(),
  //   b2.into(), c2.into(), b3.into(), c3.into(),
  //   b4.into(), c4.into(), b5.into(), c5.into(),
  // );
  // let indices = vec!(
  //   0u32,1,2, // a, b, b1
  //   1, 2, 3,  // b, b1, c
  //   3, 2, 4,  // c, b1, c1
  //   3, 5, 4,  // c, a1, c1
  //
  //   0u32,2,6, // a, b1, b2
  //   2, 6, 4,  // b1, b2, c1
  //   4, 6, 7,  // c1, b2, c2
  //   4, 5, 7,  // c1, a1, c2
  //
  //   0u32,6,8, // a, b2, b3
  //   6, 8, 7,  // b2, b3, c2
  //   3, 8, 9,  // c2, b3, c3
  //   3, 5, 9,  // c2, a1, c3
  //
  //   0u32,8,10, // a, b3, b4
  //   8, 10, 9,  // b3, b4, c3
  //   9, 10, 11,  // c3, b4, c4
  //   9, 5, 11,  // c3, a1, c4
  //
  //   0u32,10,12, // a, b4, b5
  //   10, 12, 11,  // b4, b5, c4
  //   11, 12, 13,  // c4, b5, c5
  //   11, 5, 13,  // c4, a1, c5
  // );
  const F: f32 = 1.618034; // 0.5 * (1.0 + 5f32.sqrt());

  // Base icosahedron positions
  const BASE_POSITIONS: [[f32; 3]; 12] = [
    [-1.0, F, 0.0],
    [1.0, F, 0.0],
    [-1.0, -F, 0.0],
    [1.0, -F, 0.0],
    [0.0, -1.0, F],
    [0.0, 1.0, F],
    [0.0, -1.0, -F],
    [0.0, 1.0, -F],
    [F, 0.0, -1.0],
    [F, 0.0, 1.0],
    [-F, 0.0, -1.0],
    [-F, 0.0, 1.0],
  ];

  // Base icosahedron faces
  const BASE_FACES: [[u32; 3]; 20] = [
    [0, 11, 5],
    [0, 5, 1],
    [0, 1, 7],
    [0, 7, 10],
    [0, 10, 11],
    [11, 10, 2],
    [5, 11, 4],
    [1, 5, 9],
    [7, 1, 8],
    [10, 7, 6],
    [3, 9, 4],
    [3, 4, 2],
    [3, 2, 6],
    [3, 6, 8],
    [3, 8, 9],
    [9, 8, 1],
    [4, 9, 5],
    [2, 4, 11],
    [6, 2, 10],
    [8, 6, 7],
  ];
  let detail = 3;
  assert!(detail < 30); // just a sanity check
  let mut lookup = fxhash::FxHashMap::default();
  let mut prev_faces = Vec::new();
  let mut vertices = BASE_POSITIONS.iter().map(|p| Vec3::from_slice(p)).collect::<Vec<_>>();
  let mut faces = BASE_FACES.to_vec();

  for _ in 1..detail {
    lookup.clear();
    prev_faces.clear();
    prev_faces.append(&mut faces);

    for face in prev_faces.iter() {
      let mut mid = [0u32; 3];
      for (pair, index) in face
        .iter()
        .cloned()
        .zip(face[1..].iter().chain(face.first()).cloned())
        .zip(mid.iter_mut())
      {
        *index = match lookup.get(&pair) {
          Some(i) => *i,
          None => {
            let i = vertices.len() as u32;
            lookup.insert(pair, i);
            lookup.insert((pair.1, pair.0), i);
            let v = 0.5 * (vertices[pair.0 as usize] + vertices[pair.1 as usize]);
            vertices.push(v);
            i
          }
        };
      }

      faces.push([face[0], mid[0], mid[2]]);
      faces.push([face[1], mid[1], mid[0]]);
      faces.push([face[2], mid[2], mid[1]]);
      faces.push([mid[0], mid[1], mid[2]]);
    }
  }

  let indices = faces.into_iter().flat_map(|face| face).collect::<Vec<_>>();
  let mut positions:Vec<Vec3> = Vec::with_capacity(vertices.len());
  let mut normals = Vec::with_capacity(vertices.len());

  for v in vertices {
    let n = v.normalize();
    positions.push(n * radius);
    normals.push(n);
  }

  println!("normals point: {:?}", normals);

  MeshBuilder::new(positions, Handedness::Left)
    .with_indices(indices)
    .with_vertex_normals(normals).build().unwrap()
}
