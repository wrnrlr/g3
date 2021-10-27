use baryon::{Position,geometry::{Geometry}};

// https://github.com/mrdoob/three.js/blob/dev/src/geometries/CylinderGeometry.js
pub fn cylinder(streams:super::Streams, radius:f32, height:f32)->Geometry {
  const RADIAL_SEGMENTS:u32 = 8;

  let half_height = height / 2 as f32;

  let mut positions = Vec::new();


  for x in 1..RADIAL_SEGMENTS {
    let theta = x as f32 / RADIAL_SEGMENTS as f32;
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();

    // vertex
    positions.push(Position([radius*sin_theta, height + half_height, radius * cos_theta]));

  }

  println!("Positions: {:?}", positions);

  Geometry{
    positions: vec!(Position([0f32, 0.0, 0.0])),
    normals: None, indices: None, radius
  }

}

// #[cfg(test)]
// mod tests {
//   #[test] fn test_cylinder() {
//     super::Geometry::cylinder(super::Streas::NORMAL, 1, 1);
//   }
// }
