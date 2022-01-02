use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not, Neg, Fn};
use core_simd::{f32x4,mask32x4};
use crate::{Plane,Line,Point,Motor,Rotor,IdealLine};
use crate::util::{dp_bc, flip_signs, rsqrt_nr1};
use crate::sandwich::{sw02,sw32,swl2};
use crate::geometric::{gprt};

pub fn translator(delta:f32,x:f32,y:f32,z:f32)->Translator {
  Translator::new(delta,x,y,z)
}

#[derive(Default,Debug,Clone,Copy,PartialEq)]
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

  /// Fast load operation for packed data that is already normalized. The
  /// argument `data` should point to a set of 4 float values with layout
  /// `(0.f, a, b, c)` corresponding to the multivector $a\mathbf{e}_{01} +
  /// b\mathbf{e}_{02} + c\mathbf{e}_{03}$.
  ///
  /// !!! danger
  ///
  ///     The translator data loaded this way *must* be normalized. That is,
  ///     the quantity $-\sqrt{a^2 + b^2 + c^2}$ must be half the desired
  ///     displacement.
  pub fn load_normalized(data:[f32;4])->Translator {
    Translator{ p2: f32x4::from(data)}
  }

  pub fn normalized(&self)->Translator {
    let inv_norm = rsqrt_nr1(dp_bc(self.p2,self.p2));
    Translator{p2: self.p2 * inv_norm}
  }

  pub fn inverse(&self)->Translator {
    Translator{p2: flip_signs(self.p2, mask32x4::from_array([false,true,true,true]))}
  }

  // Compute the logarithm of the translator, producing an ideal line axis.
  // In practice, the logarithm of a translator is simply the ideal partition
  // (without the scalar $1$).
  // pub fn log(&self)->IdealLine { IdealLine{p2: self.p2} } TODO

  // Compute the square root of the provided translator $t$.
  #[inline] pub fn sqrt(self)->Translator { self * 0.5 }

  // Compute the logarithm of the translator, producing an ideal line axis.
  // In practice, the logarithm of a translator is simply the ideal partition
  // (without the scalar $1$).
  pub fn log(self)->IdealLine { IdealLine{p2: self.p2} }
}

impl Display for Translator {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}e01 + {}e02 + {}e03 + 1e0123", self.e01(), self.e02(), self.e03())
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

impl Add<f32> for Translator {
  type Output = Translator;
  fn add(self, f:f32) -> Translator {
    Translator{ p2: self.p2+f }
  }
}

impl Add<Translator> for f32 {
  type Output = Translator;
  fn add(self, t:Translator) -> Translator {
    Translator{ p2: t.p2+self }
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

impl Mul<Translator> for f32 {
  type Output = Translator;
  fn mul(self, t: Translator) -> Translator {
    Translator{ p2: self*t.p2 }
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
    Translator { p2: flip_signs(self.p2, mask32x4::from_array([false,true,true,true])) }
  }
}

// Geometric Product
impl Mul<Rotor> for Translator {
  type Output = Motor;
  fn mul(self, r: Rotor) -> Self::Output {
    let p2 = gprt::<true>(r.p1, self.p2);
    Motor{p1: r.p1, p2}
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
    let p2 = gprt::<true>(m.p1, self.p2);
    Motor{p1: m.p1, p2: p2 + m.p2}
  }
}
impl Div<Translator> for Translator {
  type Output = Translator;
  fn div(self, other: Translator) -> Self::Output {
    self * other.inverse()
  }
}
