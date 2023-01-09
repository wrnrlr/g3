use std::{simd::{f32x2},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not}};

pub const e0123:Dual = dual(0.0,1.0);

/// scalar + pseudoscalar
pub const fn dual(p:f32,q:f32)->Dual { Dual::new(p,q) }
/// Dual Number
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Dual { pub(crate) p:f32x2 }

impl Dual {
  pub const fn new(p:f32,q:f32)->Dual { Dual{p:f32x2::from_array([p,q])} }
  #[inline] pub fn scalar(&self)->f32 { self.p[0] }
  #[inline] pub fn e0123(&self)->f32 { self.p[1] }
  #[inline] pub fn p(&self)->f32 { self.p[0] }
  #[inline] pub fn q(&self)->f32 { self.p[1] }
}

impl Add<Dual> for Dual { type Output = Dual;fn add(self, other: Dual) -> Dual { Dual { p:self.p+other.p } } }
impl AddAssign for Dual { fn add_assign(&mut self, other: Self) { self.p += other.p } }
impl Sub<Dual> for Dual { type Output = Dual;fn sub(self, other: Dual) -> Dual { Dual { p:self.p-other.p } } }
impl SubAssign for Dual { fn sub_assign(&mut self, other: Self) { self.p -= other.p } }
impl Mul<f32> for Dual { type Output = Dual;fn mul(self, s: f32) -> Dual { Dual { p:self.p*f32x2::splat(s) } } }
impl MulAssign<f32> for Dual { fn mul_assign(&mut self, s: f32) { self.p *= f32x2::splat(s) } }
impl Div<f32> for Dual { type Output = Dual;fn div(self, s: f32) -> Dual { Dual { p:self.p/f32x2::splat(s) } } }
impl DivAssign<f32> for Dual { fn div_assign(&mut self, s: f32) { self.p /= f32x2::splat(s) } }
impl Not for Dual { type Output = Dual;fn not(self)->Dual { Dual::new(self.e0123(), self.scalar()) } }

#[cfg(test)]
mod tests {
  use super::{Dual,dual};

  #[test] fn dual_getters() {
    assert_eq!(dual(1.0, 2.0).scalar(), 1.0);
    assert_eq!(dual(1.0, 2.0).e0123(), 2.0)
  }
  #[test] fn dual_constructor() {
    assert_eq!(Dual::new(1.0, 2.0), dual(1.0, 2.0))
  }
  #[test] fn dual_eq() {
    assert_eq!(dual(1.0, 2.0), dual(1.0, 2.0));
    assert_ne!(dual(1.0, 2.0), dual(2.0, 4.0))
  }
  #[test] fn dual_add() {
    assert_eq!(dual(1.0, 2.0) + dual(1.0,2.0), dual(2.0, 4.0))
  }
  #[test] fn dual_add_assign() {
    let mut d = dual(1.0, 2.0);
    d += dual(1.0, 2.0);
    assert_eq!(d, dual(2.0, 4.0))
  }
  #[test] fn dual_sub() {
    assert_eq!(dual(2.0, 4.0) - dual(1.0,2.0), dual(1.0,2.0))
  }
  #[test] fn dual_sub_assign() {
    let mut d = dual(2.0, 4.0);
    d -= dual(1.0, 2.0);
    assert_eq!(d, dual(1.0, 2.0))
  }
  #[test] fn dual_mul() {
    assert_eq!(dual(1.0, 2.0) * 2.0, dual(2.0, 4.0))
  }
  #[test] fn dual_mul_assign() {
    let mut d1 = dual(1.0, 2.0);
    d1 *= 2.0;
    assert_eq!(d1, dual(2.0, 4.0));
  }
  #[test] fn dual_div() {
    assert_eq!(dual(2.0, 4.0) / 2.0, dual(1.0, 2.0))
  }
  #[test] fn dual_div_assign() {
    let mut d = dual(2.0, 4.0);
    d /= 2.0;
    assert_eq!(d, dual(1.0, 2.0))
  }

  #[test] fn dual_dual() {
    assert_eq!(!dual(1.0, 2.0), dual(2.0, 1.0))
  }
}
