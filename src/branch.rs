use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not,Neg,BitXor,BitAnd};
use core_simd::{f32x4,mask32x4};
use crate::{Dual, Plane, Point, Rotor, Line, Horizon};
use crate::maths::{gp11, flip_signs, hi_dp, hi_dp_bc, hi_dp_ss, rsqrt_nr1, sqrt_nr1};

pub fn branch(a:f32,b:f32,c:f32)->Branch { Branch::new(a,b,c) }

// The `Branch` both a line through the origin and also the principal branch of
// the logarithm of a rotor.
//
// The rotor branch will be most commonly constructed by taking the
// logarithm of a normalized rotor. The branch may then be linearily scaled
// to adjust the "strength" of the rotor, and subsequently re-exponentiated
// to create the adjusted rotor.
//
// !!! example
//
//     Suppose we have a rotor $r$ and we wish to produce a rotor
//     $\sqrt[4]{r}$ which performs a quarter of the rotation produced by
//     $r$. We can construct it like so:
//
//     ```rust
//         let b = r.log();
//         let r_4 = (0.25f * b).exp();
//     ```
//
// !!! note
//
//     The branch of a rotor is technically a `Line`, but because there are
//     no translational components, the branch is given its own type for
//     efficiency.
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Branch {
  pub p1:f32x4
}

impl Branch {
  #[inline] pub fn e12(&self)->f32 { self.p1[3] }
  #[inline] pub fn e21(&self)->f32 { -self.e12() }
  #[inline] pub fn z(&self)->f32 { self.e12() }
  #[inline] pub fn e31(&self)->f32 { self.p1[2] }
  #[inline] pub fn e13(&self)->f32 { -self.e31() }
  #[inline] pub fn y(&self)->f32 { self.e31() }
  #[inline] pub fn e23(&self)->f32 { self.p1[1] }
  #[inline] pub fn e32(&self)->f32 { -self.e23() }
  #[inline] pub fn x(&self)->f32 { self.e23() }

  // Construct the branch as the following multivector:
  //
  // $$a \mathbf{e}_{23} + b\mathbf{e}_{31} + c\mathbf{e}_{12}$$
  //
  // To convince yourself this is a line through the origin, remember that
  // such a line can be generated using the geometric product of two planes
  // through the origin.
  pub fn new(a:f32, b:f32, c:f32)->Branch { Branch{p1: f32x4::from_array([0.0, a, b, c])} }

  // If a line is constructed as the regressive product (join) of
  // two points, the squared norm provided here is the squared
  // distance between the two points (provided the points are
  // normalized). Returns $d^2 + e^2 + f^2$.
  pub fn squared_norm(self)->f32 { hi_dp(self.p1, self.p1)[0] }

  // Returns the square root of the quantity produced by `squared_norm`.
  pub fn norm(self)->f32 { self.squared_norm().sqrt() }

  // TODO normalize

  pub fn normalized(self)->Branch {
    Branch{p1: self.p1 * rsqrt_nr1(hi_dp_bc(self.p1, self.p1))}
  }

  // TODO invert

  pub fn inverse(self)->Branch {
    let inv_norm = rsqrt_nr1(hi_dp_bc(self.p1, self.p1));
    let mut p1 = self.p1 * inv_norm * inv_norm;
    p1 = flip_signs(p1, mask32x4::from_array([false, true, true, true]));
    Branch{p1}
  }

  // Exponentiate a branch to produce a rotor.
  pub fn exp(self)->Rotor {
    let ang = sqrt_nr1(hi_dp(self.p1, self.p1))[0];
    let cos_ang = ang.cos();
    let sin_ang = ang.sin() / ang;
    let mut p1 = f32x4::splat(sin_ang) * self.p1;
    p1 = p1 + f32x4::from_array([cos_ang, 0.0, 0.0, 0.0]);
    Rotor{p1}
  }

  pub fn sqrt(self)->Rotor {
    let p1 = self.p1 + f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    Rotor{p1}.normalized()
  }

  // Reversion
  pub fn reverse(self)->Branch {
    Branch{p1: flip_signs(self.p1, mask32x4::from_array([false,true,true,true]))}
  }
}

impl Add<Branch> for Branch {
  type Output = Branch;
  fn add(self, other: Branch) -> Branch {
    Branch { p1: self.p1+other.p1 }
  }
}

impl AddAssign for Branch {
  fn add_assign(&mut self, other: Self) {
    self.p1 += other.p1;
  }
}

impl Sub<Branch> for Branch {
  type Output = Branch;
  fn sub(self, other: Branch) -> Branch {
    Branch { p1: self.p1-other.p1 }
  }
}

impl SubAssign for Branch {
  fn sub_assign(&mut self, other: Self) {
    self.p1 -= other.p1;
  }
}

impl Mul<f32> for Branch {
  type Output = Branch;
  fn mul(self, s: f32) -> Branch {
    Branch { p1:self.p1*f32x4::splat(s) }
  }
}

impl Mul<Branch> for Branch {
  type Output = Rotor;
  fn mul(self, other: Branch) -> Rotor {
    let other = other.inverse();
    let p1 = gp11(self.p1, other.p1);
    return Rotor{p1}
  }
}

impl Div<Branch> for Branch {
  type Output = Rotor;
  fn div(self, other: Branch) -> Rotor {
    let other = other.inverse();
    self * other
  }
}

impl MulAssign<f32> for Branch {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= f32x4::splat(s)
  }
}

impl Div<f32> for Branch {
  type Output = Branch;
  fn div(self, s: f32) -> Branch {
    Branch { p1:self.p1/f32x4::splat(s) }
  }
}

impl DivAssign<f32> for Branch {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= f32x4::splat(s)
  }
}

// Unary minus
impl Neg for Branch {
  type Output = Branch;
  fn neg(self)->Branch { Branch{p1: -self.p1} }
}

impl Not for Branch {
  type Output = Horizon;
  fn not(self)-> Horizon { Horizon {p2: self.p1} }
}

// Meet Operation, Exterior Product, ^
impl BitXor<Plane> for Branch {
  type Output = Point;
  fn bitxor(self, p:Plane)->Point { p ^ self }
}
impl BitXor<Line> for Branch {
  type Output = Dual;
  fn bitxor(self, l:Line)->Dual { l ^ self }
}
impl BitXor<Horizon> for Branch {
  type Output = Dual;
  fn bitxor(self, l: Horizon) ->Dual {
    Dual::new(0.0, hi_dp_ss(self.p1, l.p2)[0])
  }
}

// Join Operation, Regressive Product, &
impl BitAnd<Point> for Branch {
  type Output = Plane;
  fn bitand(self, a:Point)->Plane{ a & self }
}
