use std::simd::{f32x4::from,Simd};

pub struct Mesh;

pub struct Sphere(Simd([f32;4]),Simd([f32;4]));

impl Sphere {
  fn new(x0:f32,y0:f32,z0:f32,d:f32,x1:f32,y1:f32,z1:f32,r:f32)->Self{Sphere(from([r,x0,y0,z0]),from([r,x1,y1,z1]))}
  fn mesh()->Mesh{Mesh}
}
