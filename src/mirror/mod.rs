use glam::Vec3;
use rend3::types::{Handedness, Mesh, MeshBuilder};
use crate::{Plane,E1,E2,E3,point};

fn vertex(pos: [f32; 3]) -> glam::Vec3 {
  glam::Vec3::from(pos)
}

pub fn create_plane_mesh(p:Plane)-> Mesh {
  let p = p.normalized();
  // for i in [[-1,-1],[-1,1],[1,1],[-1,-1],[1,1],[1,-1]] {}
  let m = (p*E1).sqrt();
  // x*o.Length,e0,z*o.Length,1
  // let points = [[-1f32,-1.0],[-1.0,1.0],[1.0,1.0],[-1.0,-1.0],[1.0,1.0],[1.0,-1.0]].map(|i|{point(i.0,0.0,i.1)});
  let a = m(point(-1.0,0.0,-1.0));
  let b = m(point(-1.0,0.0,1.0));
  let c = m(point(1.0,0.0,1.0));
  let d = m(point(1.0,0.0,-1.0));
  let vertices:Vec<Vec3> = vec!(a.into(), b.into(), c.into(), d.into());
  let indices = vec!(0u32, 2, 1, 0, 3, 2, 2, 3, 0, 1, 2, 0);

  // normals??

  MeshBuilder::new(vertices, Handedness::Left)
    .with_indices(indices)
    .build()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::E1;
  #[test] fn plane_mesh() {
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