use std::{convert::From,fmt::{Display,Formatter,Result},simd::{f32x4,mask32x4,SimdFloat},ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Fn}};
use crate::{Rotor,Translator,Point,Line,Plane,Origin,maths::*};

/// A Motor is a combination of a translation along a line combined
/// with a rotation about an axis parallel to that line.
/// In other words, it is the geometric product of a Translator and a Rotor.
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct Motor { pub(crate) p1:f32x4, pub(crate) p2:f32x4 }

/// a + b*e23 + c*e31 + d*e12 + e*e01 + f*e02 + g*e03 + h*e0123
pub const fn motor(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32)->Motor { Motor::new(a, b, c, d, e, f, g, h) }

impl Motor {
  /// a + b*e23 + c*e31 + d*e12 + e*e01 + f*e02 + g*e03 + h*e0123
  pub const fn new(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32)->Motor {
    Motor{p1:f32x4::from_array([a,b,c,d]), p2:f32x4::from_array([h,e,f,g])}}

  /// Produce a screw motion rotating and translating by given amounts along a
  /// provided Euclidean axis.
  pub fn from_screw_axis(angle:f32, d:f32, l:Line)->Motor {
    let (p1,p2) = gpdl(-angle * 0.5, d * 0.5, &l.p1, &l.p2);
    let (p1,p2) = exp(&p1, &p2);
    Motor{p1,p2}
  }

  /// Motor with only scalar component set to one
  pub fn one()->Motor {
    Motor::new(1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0)
  }

  pub fn from_scalar(s:f32)->Motor {
    Motor::new(s,0.0,0.0,0.0,0.0,0.0,0.0,0.0)
  }

  pub fn inverse(&self)->Motor {
    // s, t computed as in the normalization
    let b2 = dp_bc(&self.p1.into(), &self.p1.into());
    let s = &rsqrt_nr1(&b2.into());
    let bc = dp_bc(&flip_signs(&self.p1, mask32x4::from_array([true,false,false,false])), &self.p2);
    let b2_inv = &rcp_nr1(&b2);
    let t = bc * b2_inv * s;
    let neg = mask32x4::from_array([false,true,true,true]);

    // p1 * (s + t e0123)^2 = (s * p1 - t p1_perp) * (s + t e0123)
    // = s^2 p1 - s t p1_perp - s t p1_perp
    // = s^2 p1 - 2 s t p1_perp
    // (the scalar component above needs to be negated)
    // p2 * (s + t e0123)^2 = s^2 p2 NOTE: s^2 = b2_inv
    let st = s * t * &self.p1;
    let mut p2 = &self.p2 * b2_inv - (flip_signs(&(st+st), mask32x4::from_array([true,false,false,false])));
    p2 = flip_signs(&p2, neg);
    let p1 = flip_signs(&(&self.p1 * b2_inv), neg);
    Motor{p1,p2}
  }

  pub fn normalized(&self)->Motor {
    // m = b + c where b is p1 and c is p2
    //
    // m * ~m = |b|^2 + 2(b0 c0 - b1 c1 - b2 c2 - b3 c3)e0123
    //
    // The square root is given as:
    // |b| + (b0 c0 - b1 c1 - b2 c2 - b3 c3)/|b| e0123
    //
    // The inverse of this is given by:
    // 1/|b| + (-b0 c0 + b1 c1 + b2 c2 + b3 c3)/|b|^3 e0123 = s + t e0123
    //
    // Multiplying our original motor by this inverse will give us a
    // normalized motor.
    let b2 = &dp_bc(&self.p1, &self.p1);
    let s = &rsqrt_nr1(b2);
    let bc = dp_bc(&f32x4_xor(&self.p1, &[-0.0,0.0,0.0,0.0].into()), &self.p2);
    let t = bc * rcp_nr1(b2) * s;

    // (s + t e0123) * motor =
    //
    // s b0 +
    // s b1 e23 +
    // s b2 e31 +
    // s b3 e12 +
    // (s c0 + t b0) e0123 +
    // (s c1 - t b1) e01 +
    // (s c2 - t b2) e02 +
    // (s c3 - t b3) e03
    let tmp = &self.p2 * s;
    let p2 = tmp - (f32x4_xor(&(&self.p1 * t), &[-0.0,0.0,0.0,0.0].into()));
    let p1 = &self.p1 * s;
    Motor{p1,p2}
  }

  // Constrains the motor to traverse the shortest arc
  pub fn constrained(&self)->Motor {
    let mask = bits_wwww(to_bits(&self.p1) & to_bits(&f32x4::from_array([-0.0, 0.0, 0.0, 0.0])));
    let p1 = f32x4::from_bits(&mask ^ self.p1.to_bits());
    let p2 = f32x4::from_bits(&mask ^ self.p2.to_bits());
    Motor{p1,p2}
  }

  // Takes the principal branch of the logarithm of the motor, returning a
  // bivector. Exponentiation of that bivector without any changes produces
  // this motor again. Scaling that bivector by $\frac{1}{n}$,
  // re-exponentiating, and taking the result to the $n$th power will also
  // produce this motor again. The logarithm presumes that the motor is
  // normalized.
  pub fn log(&self)->Line {
    let (p1,p2) = logarithm(&self.p1, &self.p2);
    Line{p1,p2}
  }

  pub fn sqrt(self)->Motor {
    let p1 = &self.p1 + f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    Motor{p1, p2:self.p2}.normalized() // TODO use normalize to prevent extra copy
  }

  pub fn reverse(self)->Motor {
    Motor {
      p1: flip_signs(&self.p1, mask32x4::from_array([false,true,true,true])),
      p2: flip_signs(&self.p2, mask32x4::from_array([false,true,true,true]))
    }
  }

  pub fn approx_eq(&self, m:Motor, epsilon:f32)->bool {
    let eps = f32x4::splat(epsilon);
    let cmp1 = f32x4_abs(&self.p1 - m.p1) < eps;
    let cmp2 = f32x4_abs(&self.p2 - m.p2) < eps;
    cmp1 && cmp2
  }

  pub fn e12(&self)->f32 { self.p1[3] }
  pub fn e21(&self)->f32 { -self.e12() }
  pub fn e31(&self)->f32 { self.p1[2] }
  pub fn e13(&self)->f32 { -self.e31() }
  pub fn e23(&self)->f32 { self.p1[1] }
  pub fn e32(&self)->f32 { -self.e23() }
  pub fn scalar(&self)->f32 { self.p1[0] }

  pub fn e01(&self)->f32 { self.p2[1] }
  pub fn e10(&self)->f32 { -self.e01() }
  pub fn e02(&self)->f32 { self.p2[2] }
  pub fn e20(&self)->f32 { -self.e02() }
  pub fn e03(&self)->f32 { self.p2[3] }
  pub fn e30(&self)->f32 { -self.e03() }
  pub fn e0123(&self)->f32 { self.p2[0] }
}

impl Display for Motor {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{} + {}e23 + {}e31 + {}e12 + {}e01 + {}e02 + {}e03 + {}e0123",
           self.scalar(), self.e23(), self.e31(), self.e12(),
           self.e01(), self.e02(), self.e03(), self.e0123())
  }
}

impl From<Rotor> for Motor { fn from(r:Rotor)->Motor { Motor{p1: r.0, p2: f32x4::splat(0.0)} } }

impl From<Translator> for Motor { fn from(t:Translator)->Motor { Motor{p1: f32x4::from_array([1.0,0.0,0.0,0.0]), p2: t.p2} } }

impl FnMut<(Plane,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane { self.call(args) }}
impl FnOnce<(Plane,)> for Motor { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane { self.call(args) }}
impl Fn<(Plane,)> for Motor {
  extern "rust-call" fn call(&self, args: (Plane,))->Plane {
    Plane(sw012(&args.0.0, &self.p1, &self.p2))
  }
}

// TODO operator()(plane* in, plane* out, size_t count)

impl FnMut<(Line,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line { self.call(args) }}
impl FnOnce<(Line,)> for Motor { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line { self.call(args) }}
impl Fn<(Line,)> for Motor {
  extern "rust-call" fn call(&self, args: (Line,))->Self::Output {
    let (p1,p2) = swml(&args.0.p1, &args.0.p2, &self.p1, &self.p2);
    Line{p1,p2 }
  }
}

// TODO operator()(line* in, line* out, size_t count)

impl FnMut<(Point,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
impl FnOnce<(Point,)> for Motor { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
// Conjugates a point p with this motor and returns the result.
impl Fn<(Point,)> for Motor {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    let p3 = sw312(&args.0.0, &self.p1, &self.p2);
    Point(p3)
  }
}

// TODO operator()(point* in, point* out, size_t count)

impl FnMut<(Origin,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Origin,))->Point { self.call(args) }}
impl FnOnce<(Origin,)> for Motor { type Output = Point; extern "rust-call" fn call_once(self, args: (Origin,))->Point { self.call(args) }}
impl Fn<(Origin,)> for Motor {
  extern "rust-call" fn call(&self, _args: (Origin,))->Point { Point(swo12(&self.p1, &self.p2)) }
}

// The cost of this operation is the same as the application of a rotor due
// to the translational invariance of directions (points at infinity).
// TODO operator()(direction const& d)


// TODO operator()(direction* in,direction* out,size_t count)

impl Add<f32> for Motor {
  type Output = Motor;
  fn add(self, s:f32) -> Motor {
    Motor{ p1: self.p1+f32x4::splat(s), p2: self.p2+f32x4::splat(s) }
  }
}

impl Add<Motor> for f32 {
  type Output = Motor;
  fn add(self, m:Motor) -> Motor {
    Motor{ p1: m.p1+f32x4::splat(self), p2: m.p2+f32x4::splat(self) }
  }
}

impl Add for Motor {
  type Output = Motor;
  fn add(self, m: Self) ->Motor {
    Motor{p1: self.p1+ m.p1, p2: self.p2+ m.p2}
  }
}

impl AddAssign for Motor {
  fn add_assign(&mut self, m: Self) {
    self.p1 += m.p1;
    self.p2 += m.p2;
  }
}

impl Add<Translator> for Motor {
  type Output = Motor;
  fn add(self, t: Translator) ->Motor {
    Motor{p1: self.p1, p2: self.p2+ t.p2}
  }
}

impl Sub<Motor> for Motor {
  type Output = Motor;
  fn sub(self, m: Motor) -> Motor {
    Motor { p1: self.p1- m.p1, p2: self.p2- m.p2 }
  }
}

impl SubAssign for Motor {
  fn sub_assign(&mut self, m: Self) {
    self.p1 -= m.p1;
    self.p2 -= m.p2;
  }
}

impl Mul<f32> for Motor {
  type Output = Motor;
  fn mul(self, s: f32) -> Motor {
    Motor { p1:self.p1*f32x4::splat(s), p2:self.p2*f32x4::splat(s) }
  }
}

impl Mul<Motor> for f32 {
  type Output = Motor;
  fn mul(self, m: Motor) -> Motor {
    Motor { p1:f32x4::splat(self)*m.p1, p2:f32x4::splat(self)*m.p2 }
  }
}

impl MulAssign<f32> for Motor {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= f32x4::splat(s)
  }
}

impl Div<f32> for Motor {
  type Output = Motor;
  fn div(self, s: f32) -> Motor {
    Motor { p1:self.p1/f32x4::splat(s), p2:self.p2/f32x4::splat(s) }
  }
}

impl DivAssign<f32> for Motor {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= f32x4::splat(s)
  }
}

// Unary minus
impl Neg for Motor {
  type Output = Self;
  fn neg(self)->Self::Output {
    Motor { p1: -self.p1, p2: -self.p2 }
  }
}

// geometric product

// Compose the action of a rotor and motor (`a` will be applied, then `b`)
impl Mul<Rotor> for Motor {
  type Output = Self;
  fn mul(self, r:Rotor)->Motor {
    let p1 = gp11(&self.p1, &r.0);
    let p2 = gp21(&r.0, &self.p2);
    Motor{p1,p2}
  }
}

impl MulAssign<Rotor> for Motor {
  fn mul_assign(&mut self, r: Rotor) {
    self.p1 = gp11(&self.p1, &r.0);
    self.p2 = gp21(&r.0, &self.p2);
  }
}

impl Mul<Translator> for Motor {
  type Output = Motor;
  fn mul(self, t:Translator)->Motor {
    let p2 = gprt(&self.p1, &t.p2) + self.p2;
    Motor{p1:self.p1.into(), p2}
  }
}

impl MulAssign<Translator> for Motor {
  fn mul_assign(&mut self, t: Translator) {
    self.p1 = gprt(&self.p1, &t.p2) + &self.p2
  }
}

impl Mul<Motor> for Motor {
  type Output = Motor;
  fn mul(self, m:Motor)->Motor {
    let (p1,p2) = gpmm(&self.p1, &self.p2, &m.p1, &m.p2);
    Motor{p1, p2}
  }
}

impl MulAssign<Motor> for Motor {
  fn mul_assign(&mut self, m:Motor) {
    let (p1,p2) = gpmm(&self.p1, &self.p2, &m.p1, &m.p2);
    self.p1 = p1; self.p2 = p2;
  }
}


// impl Mul<Plane> for Motor {
//   type Output = Motor;
//   fn mul(self, m:Motor)->Motor {
//     let (p1,p2) = gpmm(&self.p1, &self.p2, &m.p1, &m.p2);
//     Motor{p1, p2}
//   }
// }

impl Div<Rotor> for Motor {
  type Output = Self;
  fn div(self,r:Rotor)->Motor {
    self * r.inverse()
  }
}

impl Div<Translator> for Motor {
  type Output = Self;
  fn div(self,t:Translator)->Motor {
    self * t.inverse()
  }
}

impl Div<Motor> for Motor {
  type Output = Self;
  fn div(self, m:Motor)->Motor {
    self * m.inverse()
  }
}

// TODO DivAssign ???

#[cfg(test)]
mod tests {
  use crate::*;

  #[test] fn motor_normalized() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4).normalized();
    assert_eq!((m*m.reverse()).scalar(), 1.0, "for a normalized motor m*~m = 1")
  }

  #[test] fn motor_by_scalar() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4)*2.0;
    assert_eq!(m, Motor::new(0.2,0.4,0.6,0.8,0.2,0.4,0.6,0.8));
  }

  #[test] fn motor_from_translator() {
    let a = point(2.0,0.0,0.0);
    let m = Motor::from(Translator::new(2.0,1.0,0.0,0.0));
    assert_eq!(m(a), point(4.0, 0.0, 0.0));
  }

  #[test] fn motor_from_rotor() {
    // Rotate point 90 degrees
    let a = point(2.0,0.0,0.0);
    let m:Motor = Rotor::new(-pi/2.0,0.0,0.0,1.0).into();
    assert_eq!(m(a).normalized(), point(0.0,2.0,0.0));
  }

  #[test] fn motor_constrained() {
    let m1 = Motor::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    let m2 = m1.constrained();
    assert_eq!(m1, m2);
    let m3 = -m1;
    let m4 = m1.constrained();
    assert_eq!(m3, -m4);
  }

  fn approx_eq(result:[f32; 3], expected:[f32; 3]) {
    const EPSILON:f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a-b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  fn approx_eq4(result:[f32; 4], expected:[f32; 4]) {
    const EPSILON:f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a-b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn construct_motor() {
    let r = rotor(pi * 0.5, 0.0, 0.0, 1.0);
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let m = r * t;
    let a = point(1.0, 0.0, 0.0);
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, -1.0, 1.0]);

    let m = t * r;
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, -1.0, 1.0]);

    let l = m.log();
    approx_eq([l.e23(), l.e12(), l.e31()], [0f32, 0.7854, 0.0]);
    approx_eq([l.e01(), l.e02(), l.e03()], [0f32, 0.0, -0.5]);
  }

  #[test] fn construct_motor_via_screw_axis() {
    let m = Motor::from_screw_axis(pi*0.5, 1.0, line(0.0,0.0,0.0,0.0,0.0,1.0));
    let a = point(1.0, 0.0, 0.0);
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, 1.0, 1.0]);
  }

  #[test] fn motor_plane() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0);
    let a = plane(3.0, 2.0, 1.0, -1.0);
    let b:Plane = m(a);
    assert_eq!([b.a(), b.b(), b.c(), b.d()], [78.0, 60.0, 54.0, 358.0]);
  }

  // #[test] fn motor_plane_variadic() {todo!()}

  #[test] fn motor_point() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0);
    let a = point(-1.0, 1.0, 2.0);
    let b = m(a);
    assert_eq!([b.x(), b.y(), b.z(), b.w()], [-12.0, -86.0, -86.0, 30.0]);
  }

  // #[test] fn motor_point_variadic() {todo!()}

  #[test] fn motor_line() {
    let m = motor(2.0, 4.0, 3.0, -1.0, -5.0, -2.0, 2.0, -3.0);
    let l = line(-1.0, 2.0, -3.0, -6.0, 5.0, 4.0);
    let k = m(l);
    approx_eq([k.e01(), k.e02(), k.e03()], [6.0, 522.0, 96.0]);
    approx_eq([k.e12(), k.e31(), k.e23()], [-214.0, -148.0, -40.0]);
  }

  // #[test] fn motor_line_variadic() {todo!()}

  #[test] fn motor_origin() {
    let r = rotor(pi * 0.5, 0.0, 0.0, 1.0);
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let m = r * t;
    let p1:Point = m(point(0.0,0.0,0.0));
    let p2:Point = m(Origin{});
    approx_eq([p1.x(), p1.y(), p1.z()], [0.0, 0.0, 1.0]);
    approx_eq([p2.x(), p2.y(), p2.z()], [0.0, 0.0, 1.0]);
  }

  #[test] fn normalize_motor() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0).normalized();
    let norm = m * m.reverse();
    approx_eq([norm.scalar(), norm.e0123(), 0.0], [1.0, 0.0, 0.0]);
  }

  #[test] fn motor_sqrt() {
    let m = Motor::from_screw_axis(pi/2.0, 3.0, line(3.0, 1.0, 3.0, 4.0, -2.0, 1.0).normalized());
    let s = m.sqrt();
    let n = s * s;
    approx_eq([m.scalar(), m.e01(), m.e02()], [n.scalar(), n.e01(), n.e02()]);
    approx_eq([m.e03(), m.e23(), m.e31()], [n.e03(), n.e23(), n.e31()]);
    approx_eq([m.e12(), m.e0123(), 0.0], [n.e12(), n.e0123(), 0.0]);
  }
}
