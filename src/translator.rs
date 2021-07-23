use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Not,Neg,Fn};
use core_simd::{f32x4,Mask32};
use crate::{Plane,Line,Point,Motor,Rotor};
use crate::util::{f32x4_flip_signs};
use crate::sandwich::{sw02,sw32,swl2};
use crate::geometric::{gprt};

pub struct Translator {
  pub p2:f32x4
}

impl Translator {

  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[1] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[1] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }

  pub fn new(delta:f32,x:f32,y:f32,z:f32)->Translator {
    let norm:f32 = (x * x + y * y + z * z).sqrt();
    let inv_norm:f32 = 1.0 / norm;
    let half_d = -0.5 * delta;
    let mut p2:f32x4 = f32x4::splat(half_d) * f32x4::from_array([0.0,x,y,z]);
    p2 = p2 * f32x4::from_array([0.0,inv_norm,inv_norm,inv_norm]);
    Translator{p2:p2}
  }

  // TODO pub load_normalized() {}

  pub fn inverse(&self)->Translator {
    Translator{p2: f32x4_flip_signs(self.p2, Mask32::from_array([false,true,true,true]))}
  }
}

impl FnMut<(Plane,)> for Translator { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane { self.call(args) }}
impl FnOnce<(Plane,)> for Translator { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane {self.call(args)} }
// Conjugates a plane $p$ with this translator and returns the result t*p*!t TODO check manual if this is right
impl Fn<(Plane,)> for Translator {
  extern "rust-call" fn call(&self, args: (Plane,))->Plane {
    let tmp:f32x4 = self.p2 + f32x4::from_array([1.0,1.0,1.0,1.0]);
    Plane{p0:sw02(args.0.p0, tmp)}
  }
}

impl FnMut<(Line,)> for Translator { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line { self.call(args) }}
impl FnOnce<(Line,)> for Translator { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line {self.call(args)} }
impl Fn<(Line,)> for Translator {
  extern "rust-call" fn call(&self, args: (Line,))->Line {
    let (p1,p2) = swl2(args.0.p1, args.0.p2, self.p2); // TODO p1 is just a, isn't this unnecessary
    Line{p1:p1,p2:p2}
  }
}

impl FnMut<(Point,)> for Translator { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
impl FnOnce<(Point,)> for Translator { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point {self.call(args)} }
impl Fn<(Point,)> for Translator {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    Point{p3:sw32(args.0.p3, self.p2)}
  }
}

impl Add<Translator> for Translator {
  type Output = Translator;
  fn add(self, other: Translator) -> Translator {
    Translator{ p2: self.p2+other.p2 }
  }
}

impl AddAssign for Translator {
  fn add_assign(&mut self, other: Self) {
    self.p2 += other.p2;
  }
}

impl Sub<Translator> for Translator {
  type Output = Translator;
  fn sub(self, other: Translator) -> Translator {
    Translator{ p2: self.p2-other.p2 }
  }
}

impl SubAssign for Translator {
  fn sub_assign(&mut self, other: Self) {
    self.p2 -= other.p2;
  }
}

impl Mul<f32> for Translator {
  type Output = Translator;
  fn mul(self, s: f32) -> Translator {
    Translator{ p2: self.p2*s }
  }
}

impl MulAssign<f32> for Translator {
  fn mul_assign(&mut self, s: f32) {
    self.p2 *= s
  }
}

impl Div<f32> for Translator {
  type Output = Translator;
  fn div(self, s: f32) -> Translator {
    Translator{ p2:self.p2/s }
  }
}

impl DivAssign<f32> for Translator {
  fn div_assign(&mut self, s: f32) {
    self.p2 /= s
  }
}

// Unary minus
impl Neg for Translator {
  type Output = Self;
  fn neg(self)->Self::Output {
    Translator{ p2: -self.p2}
  }
}

//  Reversion operator
impl Not for Translator {
  type Output = Self;
  fn not(self)->Self::Output {
    Translator { p2: f32x4_flip_signs(self.p2, Mask32::from_array([false,true,true,true])) }
  }
}

// Geometric Product
impl Mul<Rotor> for Translator {
  type Output = Motor;
  fn mul(self, r: Rotor) -> Self::Output {
    let p2 = gprt::<true>(r.p1,self.p2);
    let p1 = f32x4::splat(0.0); // TODO default zero??? p1 is not set in klein...
    Motor{p1,p2}
  }
}
impl Mul<Translator> for Translator {
  type Output = Translator;
  fn mul(self, other: Translator) -> Translator {
    self + other
  }
}
impl Mul<Motor> for Translator {
  type Output = Motor;
  fn mul(self, m: Motor) -> Self::Output {
    let mut p2 = gprt::<true>(m.p1,self.p2);
    let homogenious = self.p2[0];
    p2 = p2 + f32x4::splat(homogenious);
    let p1 = f32x4::splat(0.0); // TODO default zero??? p1 is not set in klein...
    Motor{p2,p1}
  }
}
impl Div<Translator> for Translator {
  type Output = Translator;
  fn div(self, other: Translator) -> Self::Output {
    self * other.inverse()
  }
}
