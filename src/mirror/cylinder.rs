// use std::f32::consts::PI;
// use crate::{Point, Plane, point};
// use baryon::{Position,geometry::{Geometry}};

// pub fn rotate(a:f32, p:Plane)->Point {
//   a.cos()+p.normalized()*a.sin()
// }
//
// pub fn lathe(x:Point, n:i32, p:Plane, m:f32)->Vec<Point> {
//   (0..n+1).map( |i| {rotate(i as f32/n*PI*m,p) (x)} ).collect()
// }
//
// pub fn wrap(ps:Vec<Vec<Point>>)->Vec<[Point;3]> {
//   let u = ps.len()-1;
//   let v = ps[0].len();
//   let x2 = ps.into_iter().flat_map(|v|v);
//   let p = &mut Vec::new();
//   let vp = v+1;
//   for i in 0..u*vp {
//     for j in 0..v {
//       p.extend([[i+j, i+j+1, vp+i+j], [i+j+1, vp+i+j, vp+i+j+1]]);
//     }
//   }
//   p.map(|k| point(x2[k[0]], x2[k[1]], x2[k[2]])).collect()
// }
//
// pub fn disk(x:Point, r:f32, n:f32)->Vec<Point> {
//   lathe()
// }
//
// pub fn cylinder2(r:f32, h:f32, x:f32) {
//   wrap()
// }


// https://github.com/mrdoob/three.js/blob/dev/src/geometries/CylinderGeometry.js
// pub fn cylinder(_streams:super::Streams, radius:f32, height:f32)->Geometry {
//   const RADIAL_SEGMENTS:u32 = 8;
//
//   let half_height = height / 2 as f32;
//
//   let mut positions = Vec::new();
//   // let mut normals = Vec::new();
//
//   // let slope = 0; // rise over run is needed when radius bottom and top are different
//
//   for x in 0..RADIAL_SEGMENTS {
//     let theta = x as f32 / RADIAL_SEGMENTS as f32;
//     let sin_theta = theta.sin();
//     let cos_theta = theta.cos();
//
//     // vertex
//     positions.push(Position([radius*sin_theta, height + half_height, radius * cos_theta]));
//
//     // normals.push(Normal([sin_theta, slope, cos_theta])); // normalize
//   }
//
//   print_obj(&positions);
//
//   let object_radius = 20.0;
//   Geometry{
//     positions,
//     normals: None, indices: None, radius: object_radius
//   }
//
// }
//
// fn print_obj(ps:&Vec<Position>) {
//   for p in ps {
//     println!("v: {:?} {:?} {:?}", p.0[0], p.0[1], p.0[2])
//   }
// }

// #[cfg(test)]
// mod tests {
//   #[test] fn test_cylinder() {
//     super::Geometry::cylinder(super::Streas::NORMAL, 1, 1);
//   }
// }
