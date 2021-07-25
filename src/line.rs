use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not,Neg};
use core_simd::{f32x4,Mask32};
use crate::{Motor};
use crate::util::{f32x4_flip_signs,exp};

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
    self.p1 *= s
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
    self.p1 /= s
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

// TODO Branch
// inline rotor KLN_VEC_CALL exp(branch b) noexcept
// inline rotor KLN_VEC_CALL sqrt(branch b) noexcept

// TODO IdealLine
// inline translator KLN_VEC_CALL exp(ideal_line il) noexcept
