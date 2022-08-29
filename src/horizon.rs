use std::{simd::{f32x4,mask32x4},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not, Neg, BitXor, BitAnd, BitOr}};
use crate::{Dual, Plane, Point, Line, Branch, Translator,maths::{flip_signs, hi_dp, dotilp}};
#[cfg(feature = "bevy")] use bevy::prelude::Component;

pub const fn horizon(a:f32, b:f32, c:f32) -> Horizon { Horizon::new(a, b, c) }

// A horizon represents a line at infinity and corresponds to the multivector:
//
// $$a\mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03}$$
#[cfg_attr(feature="bevy",derive(Component))]
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Horizon { pub p2:f32x4 }

impl Horizon {
  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[2] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[3] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }

  pub const fn new(a:f32,b:f32,c:f32)->Horizon { Horizon {p2: f32x4::from_array([0.0, a, b, c])} }

  pub fn squared_ideal_norm(&self)->f32 {
    hi_dp(&self.p2, &self.p2)[0]
  }

  pub fn ideal_norm(self)->f32 {
    self.squared_ideal_norm().sqrt()
  }

  // Exponentiate a horizon to produce a translation.
  //
  // The exponential of an horizon
  // $a \mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03}$ is given as:
  //
  // $$\exp{\left[a\ee_{01} + b\ee_{02} + c\ee_{03}\right]} = 1 +\
  // a\ee_{01} + b\ee_{02} + c\ee_{03}$$
  #[inline] pub fn exp(self)->Translator { Translator{p2: self.p2} }

  pub fn reverse(self)-> Horizon {
    Horizon {p2: flip_signs(&self.p2, mask32x4::from_array([false,true,true,true]))}
  }
}

impl Add<Horizon> for Horizon {
  type Output = Horizon;
  fn add(self, other: Horizon) -> Horizon {
    Horizon { p2: self.p2+other.p2 }
  }
}

impl AddAssign for Horizon {
  fn add_assign(&mut self, other: Self) {
    self.p2 += other.p2;
  }
}

impl Sub<Horizon> for Horizon {
  type Output = Horizon;
  fn sub(self, other: Horizon) -> Horizon {
    Horizon { p2: self.p2-other.p2 }
  }
}

impl SubAssign for Horizon {
  fn sub_assign(&mut self, other: Self) {
    self.p2 -= other.p2;
  }
}

impl Mul<f32> for Horizon {
  type Output = Horizon;
  fn mul(self, s: f32) -> Horizon {
    Horizon { p2:self.p2*f32x4::splat(s) }
  }
}

impl MulAssign<f32> for Horizon {
  fn mul_assign(&mut self, s: f32) {
    self.p2 *= f32x4::splat(s)
  }
}

impl Div<f32> for Horizon {
  type Output = Horizon;
  fn div(self, s: f32) -> Horizon {
    Horizon { p2:self.p2/f32x4::splat(s) }
  }
}

impl DivAssign<f32> for Horizon {
  fn div_assign(&mut self, s: f32) {
    self.p2 /= f32x4::splat(s)
  }
}

// Unary minus
impl Neg for Horizon {
  type Output = Horizon;
  fn neg(self)-> Horizon { Horizon {p2: -self.p2} }
}

// Dual operator
impl Not for Horizon {
  type Output = Branch;
  fn not(self)->Branch { Branch(self.p2) }
}

// Meet Operation, Exterior Product, ^
impl BitXor<Plane> for Horizon {
  type Output = Point;
  fn bitxor(self, p:Plane)->Point { p ^ self }
}
impl BitXor<Line> for Horizon {
  type Output = Dual;
  fn bitxor(self, l:Line)->Dual { l ^ self }
}
impl BitXor<Branch> for Horizon {
  type Output = Dual;
  fn bitxor(self, b:Branch)->Dual { b ^ self }
}

// Join Operation, Regressive Product, &
impl BitAnd<Point> for Horizon {
  type Output = Plane;
  fn bitand(self, a:Point)->Plane{ a & self }
}

// Inner Product, |
impl BitOr<Plane> for Horizon {
  type Output = Plane;
  fn bitor(self, p:Plane)->Plane { Plane(dotilp(&p.0, &self.p2)) }
}
