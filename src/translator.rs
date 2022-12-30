use std::{fmt::{Display, Formatter, Result},simd::{f32x4,mask32x4},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not, Neg, Fn}};
use crate::{Plane, Line, Point, Motor, Rotor, Horizon,maths::*};

pub fn translator(delta:f32,x:f32,y:f32,z:f32)->Translator {
  Translator::new(delta,x,y,z)
}

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Translator { pub p2:f32x4 }

impl Translator {
  #[inline] pub fn scalar(&self)->f32 { 1.0 }
  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[2] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[3] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }

  pub fn new(delta:f32,x:f32,y:f32,z:f32)->Translator {
    let norm:f32 = (x * x + y * y + z * z).sqrt();
    let inv_norm:f32 = 1.0 / norm;
    let half_d = -0.5 * delta;
    let mut p2 = f32x4::splat(half_d) * f32x4::from_array([0.0,x,y,z]) * f32x4::from_array([0.0,inv_norm,inv_norm,inv_norm]);
    Translator{p2}
  }

  /// Fast load operation for packed data that is already normalized. The
  /// argument `data` should point to a set of 4 float values with layout
  /// `(0.f, a, b, c)` corresponding to the multivector
  /// $`a\mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03}`$.
  ///
  /// The translator data loaded this way *must* be normalized. That is,
  /// the quantity $`-\sqrt{a^2 + b^2 + c^2}`$ must be half the desired
  /// displacement.
  pub fn load_normalized(data:[f32;4])->Translator {
    Translator{ p2: data.into()}
  }

  // TODO broken???
  pub fn normalized(&self)->Translator {
    let inv_norm = rsqrt_nr1(&dp_bc(&self.p2,&self.p2));
    Translator{p2: &self.p2 * inv_norm}
  }

  pub fn inverse(&self)->Translator {
    Translator{p2: flip_signs(&self.p2, mask32x4::from_array([false,true,true,true]))}
  }

  // Compute the logarithm of the translator, producing a horizon axis.
  // In practice, the logarithm of a translator is simply the horizon partition
  // (without the scalar $1$).f
  // pub fn log(&self)->IdealLine { IdealLine{p2: self.p2} } TODO

  // Compute the square root of the provided translator $t$.
  #[inline] pub fn sqrt(&self)->Translator { *self * 0.5 }

  // Compute the logarithm of the translator, producing an horizon axis.
  // In practice, the logarithm of a translator is simply the horizon partition
  // (without the scalar $1$).
  pub fn log(&self)-> Horizon { Horizon {p2: self.p2} }
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
    let tmp:f32x4 = &self.p2 + f32x4::from_array([1.0,1.0,1.0,1.0]);
    Plane(sw02(&args.0.0, &tmp))
  }
}

impl FnMut<(Line,)> for Translator { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line { self.call(args) }}
impl FnOnce<(Line,)> for Translator { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line {self.call(args)} }
impl Fn<(Line,)> for Translator {
  extern "rust-call" fn call(&self, args: (Line,))->Line {
    let (p1,p2) = swl2(&args.0.p1, &args.0.p2, &self.p2); // TODO p1 is just a, isn't this unnecessary
    Line{p1,p2 }
  }
}

impl FnMut<(Point,)> for Translator { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
impl FnOnce<(Point,)> for Translator { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point {self.call(args)} }
impl Fn<(Point,)> for Translator {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    Point(sw32(&args.0.0, &self.p2))
  }
}

impl Add<f32> for Translator {
  type Output = Translator;
  fn add(self, s:f32) -> Translator {
    Translator{ p2: self.p2+f32x4::splat(s) }
  }
}

impl Add<Translator> for f32 {
  type Output = Translator;
  fn add(self, t:Translator) -> Translator {
    Translator{ p2: t.p2+f32x4::splat(self) }
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
    t*self
  }
}

impl Mul<f32> for Translator {
  type Output = Translator;
  fn mul(self, s: f32) -> Translator {
    Translator{ p2: self.p2*f32x4::splat(s) }
  }
}

impl MulAssign<f32> for Translator {
  fn mul_assign(&mut self, s: f32) {
    self.p2 *= f32x4::splat(s)
  }
}

impl Div<f32> for Translator {
  type Output = Translator;
  fn div(self, s: f32) -> Translator {
    Translator{ p2:self.p2/f32x4::splat(s) }
  }
}

impl DivAssign<f32> for Translator {
  fn div_assign(&mut self, s: f32) {
    self.p2 /= f32x4::splat(s)
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
    Translator { p2: flip_signs(&self.p2, mask32x4::from_array([false,true,true,true])) }
  }
}

// Geometric Product
impl Mul<Rotor> for Translator {
  type Output = Motor;
  fn mul(self, r: Rotor) -> Self::Output {
    let p2 = gptr(&r.0, &self.p2);
    Motor{p1: r.0, p2}
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
    let p2 = gptr(&m.p1, &self.p2);
    Motor{p1: m.p1.into(), p2: p2 + m.p2}
  }
}
impl Div<Translator> for Translator {
  type Output = Translator;
  fn div(self, other: Translator) -> Self::Output {
    self * other.inverse()
  }
}

// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use crate::{Translator, point, Point, ORIGIN};

  fn approx_eq(result: [f32; 4], expected: [f32; 4]) {
    const EPSILON: f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn translator_from_points() {
    let a = point(1.0, 1.0, 1.0);
    let b = point(-1.0, -1.0, -1.0);
    let a_to_b = (b.normalized() / a.normalized()).sqrt();

    // assert_eq!(a_to_b);

    // translate a to b
    let c:Point = a_to_b(a);
    approx_eq([c.e013(), c.e021(), c.e032(), c.e123()], [b.e013(), b.e021(), b.e032(), b.e123()]);
    //translate halfway between a and b (origin)
    let d:Point = (a_to_b*0.5)(a);
    approx_eq([d.e013(), d.e021(), d.e032(), d.e123()], [ORIGIN.e013(), ORIGIN.e021(), ORIGIN.e032(), ORIGIN.e123()]);

    let t = Translator::new(4.0,1.0,0.0,1.0);
    let e:Point = t(ORIGIN);
    let f = point(8f32.sqrt(), 0.0, 8f32.sqrt());
    approx_eq([e.e013(), e.e021(), e.e032(), e.e123()], [f.e013(), f.e021(), f.e032(), f.e123()]);
  }
}
