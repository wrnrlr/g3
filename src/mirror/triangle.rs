// use crate::Point;
// use baryon::{Position, geometry::Geometry};
//
// // fn cross(a:[f32;3],b:[f32;3])->[f32;3] {
// //
// // }
//
// pub fn triangle(f:[Point;3])->Geometry {
//   // https://gamedev.stackexchange.com/questions/60630/how-do-i-find-the-circumcenter-of-a-triangle-in-3d
//   // let ac = c-a;
//   // let ab = b-a;
//   // let ab_ac = cross(ac,bc);
//   let a = [f[0].x(),f[0].y(),f[0].z()];
//   let b = [f[1].x(),f[1].y(),f[1].z()];
//   let c = [f[2].x(),f[2].y(),f[2].z()];
//
//   let radius:f32 = 5.0;
//   Geometry{
//     positions: vec!(Position(a),Position(b),Position(c)),
//     normals: None, indices: None, radius
//   }
// }
