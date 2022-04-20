use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

/*

[a,b,c][c,b,d]
 */

#[derive(Debug, Copy, Clone)]
pub struct DoubleSidedPlane {
  /// The total side length of the square.
  pub size: f32,
}

impl Default for DoubleSidedPlane {
  fn default() -> Self {
    DoubleSidedPlane { size: 1.0 }
  }
}

impl From<DoubleSidedPlane> for Mesh {
  fn from(plane: DoubleSidedPlane) -> Self {
    let extent = plane.size / 2.0;

    let vertices = [
      ([extent, 0.0, -extent], [0.0, 1.0, 0.0], [1.0, 1.0]),
      ([extent, 0.0, extent], [0.0, 1.0, 0.0], [1.0, 0.0]),
      ([-extent, 0.0, extent], [0.0, 1.0, 0.0], [0.0, 0.0]),
      ([-extent, 0.0, -extent], [0.0, 1.0, 0.0], [0.0, 1.0]),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2, 2, 3, 0, 1, 2, 0]);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
      positions.push(*position);
      normals.push(*normal);
      uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
  }
}