use std::ops::{Fn};
use core_simd::{f32x4,Mask32};
use crate::{Point};
use crate::util::{shuffle_dddd, shuffle_wwww, shuffle_wwyz, shuffle_wyzw, shuffle_wyzx, shuffle_wzxy, shuffle_yyzw, shuffle_yyzz, shuffle_yzwy, shuffle_zwyx, shuffle_zwyz, shuffle_zzwy};


#[derive(Default,Debug,Clone,PartialEq)]
pub struct Matrix4x4 {
  pub cols:[f32x4;4]
}

impl FnMut<(Point,)> for Matrix4x4 { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
impl FnOnce<(Point,)> for Matrix4x4 { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point {self.call(args)} }
impl Fn<(Point,)> for Matrix4x4 {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    let p3 = args.0.p3;
    let mut out = self.cols[0] * shuffle_wwww(p3);
    out = out + self.cols[1] * shuffle_xxxx(p3);
    out = out + self.cols[2] * shuffle_yyyy(p3);
    out = out + self.cols[3] * shuffle_zzzz(p3);
    Point{p3:out}
  }
}

struct Matrix3x4 {
  pub cols:[f32x4;4]
}


