use std::simd::f32x4;
use glam;
use crate::{Motor, Point, Rotor};
use crate::{maths::mat4x4_12};
use crate::maths::{mat4x4_12_m, shuffle_wwww, shuffle_xxxx, shuffle_yyyy, shuffle_zzzz};

impl Into<glam::Vec3> for &Point {
  fn into(self)->glam::Vec3 {
    [self.x(), self.y(), self.z()].into()
  }
}

impl Into<glam::Vec3> for Point {
  fn into(self)->glam::Vec3 {
    [self.x(), self.y(), self.z()].into()
  }
}

pub struct Mat4(pub glam::Mat4);

impl Into<Mat4> for &Point {
  fn into(self)->Mat4 {
    Mat4(glam::Mat4::from_translation(self.into()))
  }
}

impl Into<Mat4> for Rotor {
  fn into(self)->Mat4 {
    let m = mat4x4_12(&self.0);
    Mat4(glam::Mat4::from_cols_array_2d(&[m.0.into(),m.1.into(),m.2.into(),m.3.into()]))
  }
}

// http://projectivegeometricalgebra.org/wiki/index.php?title=Motor#Conversion_from_Motor_to_Matrix


impl Into<Mat4> for Motor {
  fn into(self)->Mat4 {
    let cols = mat4x4_12_m(&self.p1, &self.p2);
    Mat4(glam::Mat4::from_cols_array_2d(&[*cols.0.as_array(), *cols.1.as_array(), *cols.2.as_array(), *cols.3.as_array()]))
  }
}

impl Mat4 {
  pub fn call(self, xyzw:f32x4)->f32x4 {
    let mut out = f32x4::from(self.0.x_axis.to_array()) * shuffle_xxxx(&xyzw);
    out = out + f32x4::from(self.0.y_axis.to_array()) * shuffle_yyyy(&xyzw);
    out = out + f32x4::from(self.0.z_axis.to_array()) * shuffle_zzzz(&xyzw);
    out = out + f32x4::from(self.0.w_axis.to_array()) * shuffle_wwww(&xyzw);
    out
  }
}

#[cfg(all(test, feature = "glam"))]
mod tests {
  use std::simd::f32x4;
  use crate::{Mat4, motor};

  #[test] fn motor_to_matrix() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0);
    let m4:Mat4 = m.into();
    let a = m4.call([-1.0,1.0,2.0,1.0].into());
    assert_eq!(a, f32x4::from([-12.0,-86.0,-86.0,30.0]));
  }

  #[test] fn motor_to_matrix_3x4() {todo!()}
}
