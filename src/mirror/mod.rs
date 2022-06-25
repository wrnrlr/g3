use glam::Vec3;
use rend3::types::{Handedness, Mesh, MeshBuilder};
use crate::{Plane,Point,E2,point};

pub fn create_plane_mesh(p:Plane)-> Mesh {
  let p = p.normalized();
  let m = (p*E2).sqrt();
  let a = m(point(-1.0,0.0,-1.0));
  let b = m(point(-1.0,0.0,1.0));
  let c = m(point(1.0,0.0,1.0));
  let d = m(point(1.0,0.0,-1.0));
  let vertices:Vec<Vec3> = vec!(a.into(), b.into(), c.into(), d.into());
  // let vertices:Vec<Vec3> = vec!(a, b, c, d).into();
  let indices = vec!(0u32, 2, 1, 0, 3, 2, 2, 3, 0, 1, 2, 0);

  // normals??

  MeshBuilder::new(vertices, Handedness::Left)
    .with_indices(indices)
    .build()
    .unwrap()
}

pub fn create_point_mesh(p:Point)->Mesh {
  let generated = IcoSphere::new(sphere.subdivisions, |point| {
    let inclination = point.y.acos();
    let azimuth = point.z.atan2(point.x);

    let norm_inclination = inclination / std::f32::consts::PI;
    let norm_azimuth = 0.5 - (azimuth / std::f32::consts::TAU);

    [norm_azimuth, norm_inclination]
  });

  let raw_points = generated.raw_points();

  let points = raw_points
    .iter()
    .map(|&p| (p * sphere.radius).into())
    .collect::<Vec<[f32; 3]>>();

  let normals = raw_points
    .iter()
    .copied()
    .map(Into::into)
    .collect::<Vec<[f32; 3]>>();

  let uvs = generated.raw_data().to_owned();

  let mut indices = Vec::with_capacity(generated.indices_per_main_triangle() * 20);

  for i in 0..20 {
    generated.get_indices(i, &mut indices);
  }

  // let indices = Indices::U32(indices);
  //
  // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
  // mesh.set_indices(Some(indices));
  // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
  // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
  // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
  // mesh


  let vertices:Vec<Vec3> = vec!();
  MeshBuilder::new(vertices, Handedness::Left)
    // .with_indices(indices)
    .build()
    .unwrap()
}


#[cfg(test)]
mod tests {
  use crate::E1;
  #[ignore] #[test] fn plane_mesh() {
    let a1 = [-1f32, 1.0, 0.0];
    let b1 = [1f32, 1.0, 0.0];
    let c1 = [1f32, -1.0, 0.0];
    let d1 = [-1f32, -1.0, 0.0];
    let p = E1;
    let x = p.x(); let y = p.y(); let z = p.z(); let d = p.d();

    // let ax =
    let a2 = [-1f32, 1.0, 0.0];
    let b2 = [1f32, 1.0, 0.0];
    let c2 = [1f32, -1.0, 0.0];
    let d2 = [-1f32, -1.0, 0.0];
    assert_eq!([a1,b1,c1,d1], [a2,b2,c2,d3]);
  }
}