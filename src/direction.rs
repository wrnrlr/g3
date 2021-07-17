use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not,Neg};
use core_simd::{f32x4,Mask32};
use crate::Plane;
use crate::util::{f32x4_flip_signs,refined_reciprocal,hi_dp_bc};
use crate::sqrt::{rsqrt_nr1};

// Directions in are represented using points at infinity (homogeneous coordinate 0).
// Having a homogeneous coordinate of zero ensures that directions are translation-invariant.
#[derive(Default,Debug,Clone,PartialEq)]
pub struct Direction {
  pub p3:f32x4
}

impl Direction {
  pub fn x(&self)->f32 { self.p3[1] }
  pub fn y(&self)->f32 { self.p3[2] }
  pub fn z(&self)->f32 { self.p3[3] }
  
  // Create a normalized direction
  pub fn new(x:f32,y:f32,z:f32)->Direction {
    Direction{p3:f32x4::from_array([0.0,x,y,z])}.normalized()
  }

  /// Normalize this direction by dividing all components by the
  /// magnitude (by default, `rsqrtps` is used with a single Newton-Raphson
  /// refinement iteration)
  pub fn normalized(&self)->Direction {
    let tmp = rsqrt_nr1(hi_dp_bc(self.p3, self.p3));
    Direction{p3: self.p3 * tmp}
  }
}

impl Add<Direction> for Direction {
  type Output = Direction;
  fn add(self, other: Direction) -> Direction { Direction { p3:self.p3+other.p3 } }
}

impl AddAssign for Direction {
  fn add_assign(&mut self, other: Self) { self.p3 = self.p3+other.p3 }
}

impl Sub<Direction> for Direction {
  type Output = Direction;
  fn sub(self, other: Direction) -> Direction { Direction { p3:self.p3-other.p3 } }
}

impl SubAssign for Direction {
  fn sub_assign(&mut self, other: Self) { self.p3 = self.p3-other.p3 }
}

impl Mul<f32> for Direction {
  type Output = Direction;
  fn mul(self, s: f32) -> Direction { Direction { p3:self.p3*s } }
}

impl MulAssign<f32> for Direction {
  fn mul_assign(&mut self, s: f32) { self.p3 = self.p3*s }
}

impl Div<f32> for Direction {
  type Output = Direction;
  fn div(self, s: f32) -> Direction { Direction { p3:self.p3*refined_reciprocal(s) } }
}

impl DivAssign<f32> for Direction {
  fn div_assign(&mut self, s: f32) { self.p3 = self.p3*refined_reciprocal(s) }
}

// Reversion
impl Neg for Direction {
  type Output = Self;
  fn neg(self)->Self::Output {
      Direction { p3:f32x4_flip_signs(self.p3, Mask32::from_array([false,true,true,true])) }
  }
}

// TODO ~ flip all sign, the ~ is not available in rust ...


impl Not for Direction {
  type Output = Plane;
  fn not(self)->Plane { Plane { p0: self.p3 }}
}
