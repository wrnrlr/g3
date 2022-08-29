use std::{simd::{f32x4,mask32x4},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not, Neg, BitXor, BitAnd, BitOr}};
use crate::{Dual, Plane, Point, Motor, Branch, Horizon,maths::{gpll, exp, f32x4_abs, flip_signs, hi_dp, hi_dp_bc, hi_dp_ss, rcp_nr1, rsqrt_nr1, dot11, dotlp}};
#[cfg(feature = "bevy")] use bevy::prelude::Component;

pub const fn line(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32)->Line { Line::new(a,b,c,d,e,f) }

#[cfg_attr(feature="bevy",derive(Component))]
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Line {pub p1:f32x4, pub p2:f32x4}

impl Line {
  #[inline] pub fn e12(&self)->f32 { self.p1[3] }
  #[inline] pub fn e21(&self)->f32 { -self.e12() }
  #[inline] pub fn e31(&self)->f32 { self.p1[2] }
  #[inline] pub fn e13(&self)->f32 { -self.e31() }
  #[inline] pub fn e23(&self)->f32 { self.p1[1] }
  #[inline] pub fn e32(&self)->f32 { -self.e23() }
  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[2] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[3] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }

  pub const fn new(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32)->Line {
    Line{p1:f32x4::from_array([0.0,d,e,f]), p2:f32x4::from_array([0.0,a,b,c])}
  }

  // Returns the square root of the quantity produced by `squared_norm`.
  pub fn norm(&self)->f32 { self.squared_norm().sqrt() }

  // If a line is constructed as the regressive product (join) of
  // two points, the squared norm provided here is the squared
  // distance between the two points (provided the points are
  // normalized). Returns $d^2 + e^2 + f^2$.
  pub fn squared_norm(&self)->f32 { hi_dp(&self.p1, &self.p1)[0] }

  // Normalize a line such that $\ell^2 = -1$.
  pub fn normalized(&self)->Line {
    // l = b + c where b is p1 and c is p2
    // l * ~l = |b|^2 - 2(b1 c1 + b2 c2 + b3 c3)e0123
    //
    // sqrt(l*~l) = |b| - (b1 c1 + b2 c2 + b3 c3)/|b| e0123
    //
    // 1/sqrt(l*~l) = 1/|b| + (b1 c1 + b2 c2 + b3 c3)/|b|^3 e0123
    //              = s + t e0123
    let b2 = &hi_dp_bc(&self.p1, &self.p1);
    let s = &rsqrt_nr1(b2);
    let bc = hi_dp_bc(&self.p1, &self.p2);
    let t = bc * rcp_nr1(b2) * s;

    // p1 * (s + t e0123) = s * p1 - t p1_perp
    let tmp = &self.p2 * s;
    Line{p1: &self.p1 * s, p2: tmp - &self.p1 * t}
  }

  pub fn inverse(&self)->Line {
    // s, t computed as in the normalization
    let b2 = &hi_dp_bc(&self.p1, &self.p1);
    let s = &rsqrt_nr1(b2);
    let bc = hi_dp_bc(&self.p1, &self.p2);
    let b2_inv = &rcp_nr1(&b2);
    let t = bc * b2_inv * s;
    let neg  = mask32x4::from_array([false, true, true, true]);

    // p1 * (s + t e0123)^2 = (s * p1 - t p1_perp) * (s + t e0123)
    // = s^2 p1 - s t p1_perp - s t p1_perp
    // = s^2 p1 - 2 s t p1_perp
    // p2 * (s + t e0123)^2 = s^2 p2
    // NOTE: s^2 = b2_inv
    let st = s * t * &self.p1;
    let p2 = flip_signs(&(&self.p2 * b2_inv - (st + st)), neg);
    let p1 = flip_signs(&(&self.p1 * b2_inv), neg);
    Line{p1,p2}
  }

  pub fn approx_eq(&self, other:Line, epsilon:f32)->bool {
    let esp = f32x4::splat(epsilon);
    let cmp1 = f32x4_abs(&self.p1 - other.p1) < esp;
    let cmp2 = f32x4_abs(&self.p2 - other.p2) < esp;
    cmp1 && cmp2
  }

  // Exponentiate a line to produce a motor that has this line
  // as its axis. This routine will be used most often when this line is
  // produced as the logarithm of an existing rotor, then scaled to subdivide
  // or accelerate the motor's action. The line need not be a _simple bivector_
  // for the operation to be well-defined.
  pub fn exp(&self)->Motor {
    let (p1,p2) = exp(&self.p1, &self.p2);
    Motor{p1,p2}
  }

  pub fn reverse(self)->Line {
    Line {
      p1: flip_signs(&self.p1, mask32x4::from_array([false,true,true,true])),
      p2: flip_signs(&self.p2, mask32x4::from_array([false,true,true,true]))
    }
  }

  // Project a line onto a point. Given a line $\ell$ and point $P$, produces the
  // line through $P$ that is parallel to $\ell$.
  pub fn project_point(self, a:Point)->Line { (self | a) | a }

  // Project a line onto a plane
  pub fn project_plane(self, p:Plane)->Line { (self | p) ^ p }
}

impl From<Branch> for Line {
  fn from(b: Branch) -> Self {
    Line{p1: b.0, p2: f32x4::splat(0.0)}
  }
}

impl From<Horizon> for Line {
  fn from(h: Horizon) -> Self {
    Line{p1: f32x4::splat(0.0), p2: h.p2}
  }
}

impl Add<Line> for Line {
  type Output = Line;
  fn add(self, other: Line) -> Line {
    Line { p1: self.p1+other.p1, p2: self.p2+other.p2 }
  }
}

impl AddAssign for Line {
  fn add_assign(&mut self, other: Self) {
    self.p1 += other.p1;
    self.p2 += other.p2;
  }
}

impl Sub<Line> for Line {
  type Output = Line;
  fn sub(self, other: Line) -> Line {
    Line { p1: self.p1-other.p1, p2: self.p2-other.p2 }
  }
}

impl SubAssign for Line {
  fn sub_assign(&mut self, other: Self) {
    self.p1 -= other.p1;
    self.p2 -= other.p2;
  }
}

impl Mul<f32> for Line {
  type Output = Line;
  fn mul(self, s: f32) -> Line {
    Line { p1:self.p1*f32x4::splat(s), p2:self.p2*f32x4::splat(s) }
  }
}

impl MulAssign<f32> for Line {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= f32x4::splat(s);
    self.p2 *= f32x4::splat(s);
  }
}

impl Div<f32> for Line {
  type Output = Line;
  fn div(self, s: f32) -> Line {
    Line { p1:self.p1/f32x4::splat(s), p2:self.p2/f32x4::splat(s) }
  }
}

impl DivAssign<f32> for Line {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= f32x4::splat(s);
    self.p2 /= f32x4::splat(s);
  }
}

// Unary minus
impl Neg for Line {
  type Output = Self;
  fn neg(self)->Self::Output { Line{p1: -self.p1, p2: -self.p2} }
}

// Dual operator
impl Not for Line {
  type Output = Self;
  fn not(self)->Self::Output { Line {p1: self.p2, p2: self.p1} }
}

// Exterior Product
impl BitXor<Plane> for Line {
  type Output = Point;
  fn bitxor(self, p:Plane)->Point{ p ^ self }
}
impl BitXor<Line> for Line {
  type Output = Dual;
  fn bitxor(self, l:Line)->Dual {
    let dp1 = hi_dp_ss(&self.p1, &l.p2);
    let dp2 = hi_dp_ss(&l.p1, &self.p2);
    Dual::new(0.0, dp1[0] + dp2[0])
  }
}
impl BitXor<Horizon> for Line {
  type Output = Dual;
  fn bitxor(self, b: Horizon) ->Dual { Branch(self.p1) ^ b }
}
impl BitXor<Branch> for Line {
  type Output = Dual;
  fn bitxor(self, b:Branch)->Dual { Horizon {p2: self.p2} ^ b }
}

// Join Operation, Regressive Product, &
impl BitAnd<Point> for Line {
  type Output = Plane;
  fn bitand(self, a:Point)->Plane{ a & self }
}

// Inner Product, |
impl BitOr<Point> for Line {
  type Output = Plane;
  fn bitor(self, a:Point)->Plane { a | self }
}

impl BitOr<Line> for Line {
  type Output = f32;
  fn bitor(self, l:Line)->f32 { dot11(&self.p1, &l.p1)[0] }
}

impl BitOr<Plane> for Line {
  type Output = Plane;
  fn bitor(self, p:Plane)->Plane { Plane(dotlp(&p.0, &self.p1, &self.p2))}
}

// Geometric Product

impl Mul<Line> for Line {
  type Output = Motor;
  fn mul(self, l: Line) -> Motor {
    let (p1,p2) = gpll(&self.p1, &self.p2, &l.p1, &l.p2);
    Motor{ p1, p2 }
  }
}

impl Div<Line> for Line {
  type Output = Motor;
  fn div(self, other: Line) -> Motor {
    let other = other.inverse();
    self * other
  }
}

#[cfg(test)]
mod tests {
  use super::{line};

  #[test] fn line_constructor() {
    let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    assert_eq!(l,line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0));
  }
  #[test] fn line_eq() {
    let l1 = line(1.,2.,3.,4.,5.,6.);
    let l2 = line(6.,5.,4.,3.,2.,1.);
    assert_eq!(l1,l1);
    assert_ne!(l1,l2)
  }
  #[test] fn line_getters() {
    let _l1 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
  }
  #[test] #[ignore] fn line_add() {}
  #[test] #[ignore] fn line_add_assign() {}
  #[test] #[ignore] fn line_sub() {}
  #[test] #[ignore] fn line_sub_assign() {}
  #[test] #[ignore] fn line_mul_scalar() {}
  #[test] #[ignore] fn line_mul_assign_scalar() {}
  #[test] #[ignore] fn line_div_scalar() {}
  #[test] #[ignore] fn line_div_assign_scalar() {}
  #[test] fn line_dual() {
    assert_eq!(!line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0), line(4.0, 5.0, 6.0, 1.0, 2.0, 3.0));
  }
  #[test] fn line_reverse() {
    assert_eq!(line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).reverse(), line(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0))
  }
}
