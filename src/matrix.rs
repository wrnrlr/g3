use crate::{Point};

// #[derive(Default,Debug,Clone,PartialEq)]
// pub struct Matrix4x4 (
//   pub [f32x4;4]
// );
//
// impl FnMut<(Point,)> for Matrix4x4 { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
// impl FnOnce<(Point,)> for Matrix4x4 { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point {self.call(args)} }
// impl Fn<(Point,)> for Matrix4x4 {
//   extern "rust-call" fn call(&self, args: (Point,))->Point {
//     let p3 = args.0.p3;
//     let mut out = self.cols[0] * shuffle_wwww(p3);
//     out = out + self.cols[1] * shuffle_xxxx(p3);
//     out = out + self.cols[2] * shuffle_yyyy(p3);
//     out = out + self.cols[3] * shuffle_zzzz(p3);
//     Point{p3:out}
//   }
// }
//
// struct Matrix3x4 {
//   pub cols:[f32x4;4]
// }

#[cfg(feature = "glam")]
impl Into<glam::Vec3> for &Point {
  fn into(self)->glam::Vec3 {
    [self.x(), self.y(), self.z()].into()
  }
}

// #[cfg(feature = "glam")]
// impl Into<glam::XYZ<f32>> for &Point {
//   fn into(self)->glam::XYZ<f32> {
//     [self.x(), self.y(), self.z()].into()
//   }
// }

#[cfg(feature = "glam")]
impl Into<glam::Vec3> for Point {
  fn into(self)->glam::Vec3 {
    [self.x(), self.y(), self.z()].into()
  }
}

#[cfg(feature = "glam")]
impl Into<glam::Mat4> for &Point {
  fn into(self)->glam::Mat4 {
    glam::Mat4::from_translation(self.into())
  }
}
