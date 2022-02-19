use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not};
use core_simd::{f32x2};

pub fn dual(p:f32,q:f32)->Dual { Dual::new(p,q) }

// A dual number is a multivector of the form p + e_0123.
#[cfg_attr(feature="bevy",derive(Component))]
#[derive(Default,Debug,Clone,PartialEq)]
pub struct Dual {
  p:f32x2
}

impl Dual {
  #[inline] pub fn scalar(&self)->f32 { self.p[0] }
  #[inline] pub fn e0123(&self)->f32 { self.p[1] }

  pub fn new(p:f32,q:f32)->Dual { Dual{p:f32x2::from_array([p,q])} }
}

impl Add<Dual> for Dual {
  type Output = Dual;
  fn add(self, other: Dual) -> Dual { Dual { p:self.p+other.p } }
}

impl AddAssign for Dual {
  fn add_assign(&mut self, other: Self) { self.p = self.p+other.p }
}

impl Sub<Dual> for Dual {
  type Output = Dual;
  fn sub(self, other: Dual) -> Dual { Dual { p:self.p-other.p } }
}

impl SubAssign for Dual {
  fn sub_assign(&mut self, other: Self) { self.p = self.p-other.p }
}

impl Mul<f32> for Dual {
  type Output = Dual;
  fn mul(self, s: f32) -> Dual { Dual { p:self.p*f32x2::splat(s) } }
}

impl MulAssign<f32> for Dual {
  fn mul_assign(&mut self, s: f32) { self.p = self.p*f32x2::splat(s) }
}

impl Div<f32> for Dual {
  type Output = Dual;
  fn div(self, s: f32) -> Dual { Dual { p:self.p/f32x2::splat(s) } }
}

impl DivAssign<f32> for Dual {
  fn div_assign(&mut self, s: f32) { self.p = self.p/f32x2::splat(s) }
}

impl Not for Dual {
  type Output = Dual;
  fn not(self)->Dual { Dual::new(self.e0123(), self.scalar()) }
}
