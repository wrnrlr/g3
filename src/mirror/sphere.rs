use crate::*;
use rend3::types::{MeshBuilder,Mesh};

fn vertex(p:Point) -> glam::Vec3 {
  glam::Vec3::new(p.x(), p.y(), p.z())
}


pub fn sphere()->Mesh {

  let mut r = &mut Rotor::new(PI/2.5, 0.0, 1.0, 0.0);
  let a = point(0.0,1.0,0.0);
  let mut b = point((1.0 - 0.5f32.atan().powf(2.0)).sqrt(), 0.5f32.atan().atan(), 0.0);
  let mut c = Rotor::new(PI/5.0, 0.0, 1.0, 0.0)(E2(b));

  let mut vs = vec!();

  for _ in 0..5 {
    let b2 = r(b);
    vs.push([a,b,b2]);
    vs.push([b,b2,c]);
    b = b2;
    vs.push([c,b,r(c)]);
    let c2 = r(c);
    vs.push([c,E2(a),c2]);
    c = c2;
  }

  MeshBuilder::new(vertex_positions.to_vec())
    // .with_indices(index_data.to_vec())
    .build()
    .unwrap()

}
