fn vertex(pos: [f32; 3]) -> glam::Vec3 {
  glam::Vec3::from(pos)
}

pub fn create_mesh() -> rend3::types::Mesh {
  let vertex_positions = [
    // far side (0.0, 0.0, 1.0)
    vertex([-1.0, -1.0, 1.0]),
    vertex([1.0, -1.0, 1.0]),
    vertex([1.0, 1.0, 1.0]),
    vertex([-1.0, 1.0, 1.0]),
    // near side (0.0, 0.0, -1.0)
    vertex([-1.0, 1.0, -1.0]),
    vertex([1.0, 1.0, -1.0]),
    vertex([1.0, -1.0, -1.0]),
    vertex([-1.0, -1.0, -1.0]),
    // right side (1.0, 0.0, 0.0)
    vertex([1.0, -1.0, -1.0]),
    vertex([1.0, 1.0, -1.0]),
    vertex([1.0, 1.0, 1.0]),
    vertex([1.0, -1.0, 1.0]),
    // left side (-1.0, 0.0, 0.0)
    vertex([-1.0, -1.0, 1.0]),
    vertex([-1.0, 1.0, 1.0]),
    vertex([-1.0, 1.0, -1.0]),
    vertex([-1.0, -1.0, -1.0]),
    // top (0.0, 1.0, 0.0)
    vertex([1.0, 1.0, -1.0]),
    vertex([-1.0, 1.0, -1.0]),
    vertex([-1.0, 1.0, 1.0]),
    vertex([1.0, 1.0, 1.0]),
    // bottom (0.0, -1.0, 0.0)
    vertex([1.0, -1.0, 1.0]),
    vertex([-1.0, -1.0, 1.0]),
    vertex([-1.0, -1.0, -1.0]),
    vertex([1.0, -1.0, -1.0]),
  ];

  let index_data: &[u32] = &[
    0, 1, 2, 2, 3, 0, // far
    4, 5, 6, 6, 7, 4, // near
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // top
    20, 21, 22, 22, 23, 20, // bottom
  ];

  rend3::types::MeshBuilder::new(vertex_positions.to_vec())
    .with_indices(index_data.to_vec())
    .build()
    .unwrap()
}