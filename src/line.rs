use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not,Neg};
use core_simd::{f32x4,Mask32};
use crate::{Motor, Translator, Rotor};
use crate::util::{f32x4_flip_signs,exp,hi_dp,hi_dp_bc};
use crate::sqrt::{rsqrt_nr1, sqrt_nr1};

pub fn line(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32)->Line { Line::new(a,b,c,d,e,f) }

#[derive(Default,Debug,Clone,PartialEq)]
pub struct Line {
  pub p1:f32x4,
  pub p2:f32x4
}

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

  pub fn new(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32)->Line {
    Line{p1:f32x4::from_array([0.0,d,e,f]), p2:f32x4::from_array([0.0,a,b,c])}
  }

  // pub fn norm(&self)->f32 { self.squared_norm().sqrt() } TODO

  // pub fn squared_norm(&self)->f32 { todo!() } TODO

  pub fn normalized()->Line { todo!() }

  pub fn inverse()->Line { todo!() }

  // Exponentiate a line to produce a motor that posesses this line
  // as its axis. This routine will be used most often when this line is
  // produced as the logarithm of an existing rotor, then scaled to subdivide
  // or accelerate the motor's action. The line need not be a _simple bivector_
  // for the operation to be well-defined.
  pub fn exp(&self)->Motor {
    let (p1,p2) = exp(self.p1, self.p1);
    Motor{p1,p2}
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
    Line { p1:self.p1*s, p2:self.p2*s }
  }
}

impl MulAssign<f32> for Line {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= s;
    self.p2 *= s;
  }
}

impl Div<f32> for Line {
  type Output = Line;
  fn div(self, s: f32) -> Line {
    Line { p1:self.p1/s, p2:self.p2/s }
  }
}

impl DivAssign<f32> for Line {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= s;
    self.p2 /= s;
  }
}

// Unary minus
impl Neg for Line {
  type Output = Self;
  fn neg(self)->Self::Output {
    Line {
      p1: -self.p1,
      p2: -self.p2
    }
  }
}

//  Reversion operator
impl Not for Line {
  type Output = Self;
  fn not(self)->Self::Output {
    Line {
      p1: f32x4_flip_signs(self.p1, Mask32::from_array([false,true,true,true])),
      p2: f32x4_flip_signs(self.p2, Mask32::from_array([false,true,true,true]))
    }
  }
}

// An ideal line represents a line at infinity and corresponds to the multivector:
//
// $$a\mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03}$$
#[derive(Default,Debug,Clone,PartialEq)]
pub struct IdealLine {
  pub p2:f32x4
}

impl IdealLine {
  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[2] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[3] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }

  pub fn squared_ideal_norm(self)->f32 {
    let dp = hi_dp(self.p2, self.p2);
    dp[0]
  }

  pub fn ideal_norm(self)->f32 {
    self.squared_ideal_norm().sqrt()
  }

  // Exponentiate an ideal line to produce a translation.
  //
  // The exponential of an ideal line
  // $a \mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03}$ is given as:
  //
  // $$\exp{\left[a\ee_{01} + b\ee_{02} + c\ee_{03}\right]} = 1 +\
  // a\ee_{01} + b\ee_{02} + c\ee_{03}$$
  pub fn exp(self)->Translator { Translator{p2: self.p2} }
}

impl Add<IdealLine> for IdealLine {
  type Output = IdealLine;
  fn add(self, other: IdealLine) -> IdealLine {
    IdealLine { p2: self.p2+other.p2 }
  }
}

impl AddAssign for IdealLine {
  fn add_assign(&mut self, other: Self) {
    self.p2 += other.p2;
  }
}

impl Sub<IdealLine> for IdealLine {
  type Output = IdealLine;
  fn sub(self, other: IdealLine) -> IdealLine {
    IdealLine { p2: self.p2-other.p2 }
  }
}

impl SubAssign for IdealLine {
  fn sub_assign(&mut self, other: Self) {
    self.p2 -= other.p2;
  }
}

impl Mul<f32> for IdealLine {
  type Output = IdealLine;
  fn mul(self, s: f32) -> IdealLine {
    IdealLine { p2:self.p2*s }
  }
}

impl MulAssign<f32> for IdealLine {
  fn mul_assign(&mut self, s: f32) {
    self.p2 *= s
  }
}

impl Div<f32> for IdealLine {
  type Output = IdealLine;
  fn div(self, s: f32) -> IdealLine {
    IdealLine { p2:self.p2/s }
  }
}

impl DivAssign<f32> for IdealLine {
  fn div_assign(&mut self, s: f32) {
    self.p2 /= s
  }
}

// Unary minus
impl Neg for IdealLine {
  type Output = Self;
  fn neg(self)->Self::Output {
    IdealLine { p2: -self.p2 }
  }
}

// TODO Reversion operator


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
#[derive(Default,Debug,Clone,PartialEq)]
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
    p1 = f32x4_flip_signs(p1, Mask32::from_array([false, true, true, true]));
    Branch{p1}
  }

  // Exponentiate a branch to produce a rotor.
  pub fn exp(self)->Rotor {
    let ang = sqrt_nr1(hi_dp(self.p1, self.p1))[0];
    let cos_ang = ang.cos();
    let sin_ang = ang.sin();
    let mut p1 = f32x4::splat(sin_ang) * self.p1;
    p1 = p1 + f32x4::from_array([cos_ang, 0.0, 0.0, 0.0]);
    Rotor{p1}
  }

  pub fn sqrt(self)->Rotor {
    let p1 = self.p1 + f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    Rotor{p1}.normalized()
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
    Branch { p1:self.p1*s }
  }
}

impl MulAssign<f32> for Branch {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= s
  }
}

impl Div<f32> for Branch {
  type Output = Branch;
  fn div(self, s: f32) -> Branch {
    Branch { p1:self.p1/s }
  }
}

impl DivAssign<f32> for Branch {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= s
  }
}

// Unary minus
impl Neg for Branch {
  type Output = Self;
  fn neg(self)->Self::Output {
    Branch { p1: -self.p1 }
  }
}

// TODO Reversion operator
