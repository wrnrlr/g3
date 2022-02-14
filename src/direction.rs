use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};
use core_simd::{f32x4,u32x4};
use crate::maths::{refined_reciprocal, hi_dp_bc, rsqrt_nr1};

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

  // Data should point to four floats with memory layout `(0.f, x, y, z)`
  // where the zero occupies the lowest address in memory.
  pub fn from_bits(bits:u32x4)->Direction {
    Direction{p3: f32x4::from_bits(bits)}
  }

  /// Normalize this direction by dividing all components by the
  /// magnitude (by default, `rsqrtps` is used with a single Newton-Raphson
  /// refinement iteration)
  pub fn normalized(&self)->Direction {
    let tmp = rsqrt_nr1(hi_dp_bc(self.p3, self.p3));
    Direction{p3: self.p3 * tmp}
  }
}

impl Into<[f32;3]> for Direction {
  fn into(self) -> [f32; 3] {
    [self.x(), self.y(), self.z()]
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
  fn mul(self, s: f32) -> Direction { Direction { p3:self.p3*f32x4::splat(s) } }
}

impl MulAssign<f32> for Direction {
  fn mul_assign(&mut self, s: f32) { self.p3 = self.p3*f32x4::splat(s) }
}

impl Div<f32> for Direction {
  type Output = Direction;
  fn div(self, s: f32) -> Direction { Direction { p3:self.p3*refined_reciprocal(s) } }
}

impl DivAssign<f32> for Direction {
  fn div_assign(&mut self, s: f32) { self.p3 = self.p3*refined_reciprocal(s) }
}

// Unary minus
impl Neg for Direction {
  type Output = Direction;
  fn neg(self)->Direction { Direction{ p3: -self.p3 } }
}
