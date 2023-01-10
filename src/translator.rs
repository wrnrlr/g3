use std::{fmt::{Display, Formatter, Result},simd::{f32x4,mask32x4,SimdFloat,simd_swizzle as swizzle,Which::{First,Second}},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Not, Neg, Fn}};
use crate::{Plane, Line, Point, Motor, Rotor, Horizon,maths::*};

/// ae₀₁ + be₀₂ + ce₀₃
pub fn translator(delta:f32,x:f32,y:f32,z:f32)->Translator {
  Translator::new(delta,x,y,z)
}

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Translator { pub p2:f32x4 }

impl Translator {
  pub fn new(delta:f32,x:f32,y:f32,z:f32)->Translator {
    let norm:f32 = (x * x + y * y + z * z).sqrt();
    let inv_norm:f32 = 1.0 / norm;
    let half_d = -0.5 * delta;
    let p2 = f32x4::splat(half_d) * f32x4::from_array([0.0,x,y,z]) * f32x4::from_array([0.0,inv_norm,inv_norm,inv_norm]);
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

  /// Compute the square root of the provided translator $t$.
  #[inline] pub fn sqrt(&self)->Translator { *self * 0.5 }

  /// Compute the logarithm of the translator, producing an horizon axis.
  /// In practice, the logarithm of a translator is simply the horizon partition
  /// (without the scalar $1$).
  pub fn log(&self)-> Horizon { Horizon {p2: self.p2} }

  pub fn approx_eq(&self, other:Translator, epsilon:f32)->bool {(&self.p2 - other.p2).abs() < f32x4::splat(epsilon)}

  #[inline] pub fn scalar(&self)->f32 { 1.0 }
  #[inline] pub fn e01(&self)->f32 { self.p2[1] }
  #[inline] pub fn e10(&self)->f32 { -self.e01() }
  #[inline] pub fn e02(&self)->f32 { self.p2[2] }
  #[inline] pub fn e20(&self)->f32 { -self.e02() }
  #[inline] pub fn e03(&self)->f32 { self.p2[3] }
  #[inline] pub fn e30(&self)->f32 { -self.e03() }
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

// Apply a translator to a line
// a := p1 input
// d := p2 input
// c := p2 translator
// out points to the start address of a line (p1, p2)
fn swl2(a:&f32x4, d:&f32x4, c:&f32x4)->(f32x4, f32x4) {
  // a0 + a1 e23 + a2 e31 + a3 e12 +
  //
  // (2a0 c0 + d0) e0123 +
  // (2(a2 c3 - a3 c2 - a1 c0) + d1) e01 +
  // (2(a3 c1 - a1 c3 - a2 c0) + d2) e02 +
  // (2(a1 c2 - a2 c1 - a3 c0) + d3) e03
  let mut p2_out = a.xzwy() * c.xwyz();
  // Add and subtract the same quantity in the low component to produce a cancellation
  p2_out -= a.xwyz() * c.xzwy();
  p2_out -= flip_signs(&(a * c.xxxx()), mask32x4::from_array([true, false, false, false]));
  (a.clone(), p2_out + p2_out + d)
}

// Apply a translator to a point.
// Assumes e0123 component of p2 is exactly 0
// p2: (e0123, e01, e02, e03)
// p3: (e123, e032, e013, e021)
// b * a * ~b
pub fn sw32(a:&f32x4, b:&f32x4)->f32x4 {
  // a0 e123 +
  // (a1 - 2 a0 b1) e032 +
  // (a2 - 2 a0 b2) e013 +
  // (a3 - 2 a0 b3) e021
  a + a.xxxx() * b * f32x4::from_array([0.0, -2.0, -2.0, -2.0])
}


// Apply a translator to a plane.
// Assumes e0123 component of p2 is exactly 0
// p0: (e0, e1, e2, e3)
// p2: (e0123, e01, e02, e03)
// b * a * ~b
// The low component of p2 is expected to be the scalar component instead
 fn sw02(a:&f32x4, b:&f32x4)->f32x4 {
  // (a0 b0^2 + 2a1 b0 b1 + 2a2 b0 b2 + 2a3 b0 b3) e0 +
  // (a1 b0^2) e1 +
  // (a2 b0^2) e2 +
  // (a3 b0^2) e3
  //
  // Because the plane is projectively equivalent on multiplication by a
  // scalar, we can divide the result through by b0^2
  //
  // (a0 + 2a1 b1 / b0 + 2a2 b2 / b0 + 2a3 b3 / b0) e0 +
  // a1 e1 +
  // a2 e2 +
  // a3 e3
  //
  // The additive term clearly contains a dot product between the plane's
  // normal and the translation axis, demonstrating that the plane
  // "doesn't care" about translations along its span. More precisely, the
  // plane translates by the projection of the translator on the plane's
  // normal.

  // a1*b1 + a2*b2 + a3*b3 stored in the low component of tmp
  let tmp = hi_dp(a, b);
  let mut inv_b = rcp_nr1(b);
  // 2 / b0
  inv_b = add_ss(&inv_b, &inv_b);
  inv_b = swizzle!(inv_b.clone(), f32x4::splat(0.0), [First(0),Second(1),Second(2),Second(3)]); // TODO faster?
  a + mul_ss(&tmp, &inv_b)
}

// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use std::simd::f32x4;
  use super::*;
  use crate::*;
  const ORIGIN:Point = point(0.0,0.0,0.0);
  const EPSILON: f32 = 0.02;

  #[test]
  fn simd_sandwich() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([-4.0, -3.0, -2.0, -1.0]);
    let c = sw02(&a, &b);
    assert_eq!([c[0], c[1], c[2], c[3]], [9.0, 2.0, 3.0, 4.0]);
  }

  #[test] fn translator_from_points() {
    let a = point(1.0, 1.0, 1.0);
    let b = point(-1.0, -1.0, -1.0);
    let a_to_b = (b.normalized() / a.normalized()).sqrt();

    // assert_eq!(a_to_b);

    // translate a to b
    let c:Point = a_to_b(a);
    assert!(c.approx_eq(b, EPSILON));
    //translate halfway between a and b (origin)
    let d:Point = (a_to_b*0.5)(a);
    assert!(d.approx_eq(ORIGIN, EPSILON));

    let t = Translator::new(4.0,1.0,0.0,1.0);
    let e:Point = t(ORIGIN);
    let f = point(8f32.sqrt(), 0.0, 8f32.sqrt());
    assert!(e.approx_eq(f, EPSILON));
  }

  #[test] fn translator_point() {
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let a = point(1.0, 0.0, 0.0);
    let b = t(a);
    assert_eq!([b.x(), b.y(), b.z()], [1.0, 0.0, 1.0]);
  }

  #[test] fn translator_line() {
    let data = [0.0, -5.0, -2.0, 2.0];
    let t = Translator::load_normalized(data);
    let l = line(-1.0, 2.0, -3.0, -6.0, 5.0, 4.0);
    let k = t(l);
    assert_eq!([k.e01(),k.e02(),k.e03(),k.e12(),k.e31(),k.e23()],
               [35.0, -14.0, 71.0, 4.0, 5.0, -6.0])
  }
}
