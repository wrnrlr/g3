use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg,Fn};
use core_simd::{f32x4,Mask32,i32x4};
use crate::sqrt::rsqrt_nr1;
use crate::{Rotor,Translator,Point,Line,Plane};
use crate::util::{flip_signs, log, rcp_nr1, dp_bc, shuffle_wwww, f32x4_and};
use crate::sandwich::{sw012,sw312,swmm};
use crate::geometric::{gp11,gp12,gprt,gpmm};
// use crate::define_call_fn;

#[derive(Default,Debug,Clone,PartialEq)]
pub struct Motor {
  pub p1:f32x4,
  pub p2:f32x4
}

impl Motor {
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

  pub fn new(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32)->Motor {
    Motor{p1:f32x4::from_array([a,b,c,d]), p2:f32x4::from_array([h,e,f,g])}}

  pub fn inverse(&self)->Motor {
    // s, t computed as in the normalization
    let b2 = dp_bc(self.p1, self.p1);
    let s = rsqrt_nr1(b2);
    let bc = dp_bc(flip_signs(self.p1, Mask32::from_array([true,false,false,false])), self.p2);
    let b2_inv = rcp_nr1(b2);
    let t = bc * b2_inv * s;
    let neg = Mask32::from_array([true,false,false,false]);

    // p1 * (s + t e0123)^2 = (s * p1 - t p1_perp) * (s + t e0123)
    // = s^2 p1 - s t p1_perp - s t p1_perp
    // = s^2 p1 - 2 s t p1_perp
    // (the scalar component above needs to be negated)
    // p2 * (s + t e0123)^2 = s^2 p2 NOTE: s^2 = b2_inv
    let st = s * t * self.p1;
    let mut p2 = self.p2 * b2_inv - (flip_signs(st*st, Mask32::from_array([true,false,false,false])));
    p2 = flip_signs(p2, neg);
    let p1 = flip_signs(self.p1 * b2_inv, neg);
    Motor{p1,p2}
  }

  pub fn normalize(&self)->Motor {
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
    let b2 = dp_bc(self.p1, self.p1);
    let s = rsqrt_nr1(b2);
    let neg = Mask32::from_array([true,false,false,false]);
    let bc = dp_bc(flip_signs(self.p1, neg), self.p2);
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
    let tmp = self.p2 * s;
    let p2 = tmp - (flip_signs(self.p1, neg)); // Why not just +????
    let p1 = self.p1 * t;
    Motor{p1,p2}
  }

  // Constrains the motor to traverse the shortest arc
  pub fn constrained(&self)->Motor {
    let mask = self.p1.to_bits() & f32x4::from_array([-0.0, 0.0, 0.0, 0.0]).to_bits();
    let p1 = f32x4::from_bits(mask ^ self.p1.to_bits());
    let p2 = f32x4::from_bits(mask ^ self.p2.to_bits());
    Motor{p1,p2}
  }

  // Takes the principal branch of the logarithm of the motor, returning a
  // bivector. Exponentiation of that bivector without any changes produces
  // this motor again. Scaling that bivector by $\frac{1}{n}$,
  // re-exponentiating, and taking the result to the $n$th power will also
  // produce this motor again. The logarithm presumes that the motor is
  // normalized.
  pub fn log(&self)->Line {
    let (p1,p2) = log(self.p1,self.p2);
    Line{p1,p2}
  }

  pub fn sqrt(self)->Motor {
    let p1 = self.p1 * f32x4::splat(1.0);
    Motor{p1:p1, p2:f32x4::splat(0.0)}.normalize() // TODO avoid extra copy of Motor 
  }

  pub fn reverse(self)->Motor {
    Motor {
      p1: flip_signs(self.p1, Mask32::from_array([false,true,true,true])),
      p2: flip_signs(self.p2, Mask32::from_array([false,true,true,true]))
    }
  }

  pub fn as_mat3x4(&self) { todo!(); }
  pub fn as_mat4x4(&self) { todo!(); }
}

impl FnMut<(Plane,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane { self.call(args) }}
impl FnOnce<(Plane,)> for Motor { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane { self.call(args) }}
impl Fn<(Plane,)> for Motor {
  extern "rust-call" fn call(&self, args: (Plane,))->Plane {
    Plane{p0:sw012::<false,true>(args.0.p0, self.p1)}
  }
}

// TODO operator()(plane* in, plane* out, size_t count)

impl FnMut<(Line,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line { self.call(args) }}
impl FnOnce<(Line,)> for Motor { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line { self.call(args) }}
impl Fn<(Line,)> for Motor {
  extern "rust-call" fn call(&self, args: (Line,))->Self::Output {
    let (p1,p2) = swmm::<false,true,true>(args.0.p1, self.p1, Some(self.p2));
    Line{p1:p1,p2:p2}
  }
}

// TODO operator()(line* in, line* out, size_t count)

impl FnMut<(Point,)> for Motor { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point { self.call(args) }}
impl FnOnce<(Point,)> for Motor { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
// Conjugates a point p with this motor and returns the result.
impl Fn<(Point,)> for Motor {
  extern "rust-call" fn call(&self, args: (Point,))->Self::Output {
    let p3 = sw312::<false, true>(args.0.p3, self.p1, self.p2);
    Point{p3: p3}
  }
}

// TODO operator()(point* in, point* out, size_t count)

// TODO operator()(origin)

// The cost of this operation is the same as the application of a rotor due
// to the translational invariance of directions (points at infinity).
// TODO operator()(direction const& d)


// TODO operator()(direction* in,direction* out,size_t count)

impl Add for Motor {
  type Output = Motor;
  fn add(self, other: Self)->Motor {
    Motor{p1: self.p1+other.p1, p2: self.p2+other.p2}
  }
}

impl AddAssign for Motor {
  fn add_assign(&mut self, other: Self) {
    self.p1 += other.p1;
    self.p2 += other.p2;
  }
}

impl Sub<Motor> for Motor {
  type Output = Motor;
  fn sub(self, other: Motor) -> Motor {
    Motor { p1: self.p1-other.p1, p2: self.p2-other.p2 }
  }
}

impl SubAssign for Motor {
  fn sub_assign(&mut self, other: Self) {
    self.p1 -= other.p1;
    self.p2 -= other.p2;
  }
}

impl Mul<f32> for Motor {
  type Output = Motor;
  fn mul(self, s: f32) -> Motor {
    Motor { p1:self.p1*s, p2:self.p2*s }
  }
}

impl MulAssign<f32> for Motor {
  fn mul_assign(&mut self, s: f32) {
    self.p1 *= s
  }
}

impl Div<f32> for Motor {
  type Output = Motor;
  fn div(self, s: f32) -> Motor {
    Motor { p1:self.p1/s, p2:self.p2/s }
  }
}

impl DivAssign<f32> for Motor {
  fn div_assign(&mut self, s: f32) {
    self.p1 /= s
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
    let p1 = gp11(self.p1,r.p1);
    let p2 = gp12::<true>(r.p1,self.p1);
    Motor{p1,p2}
  }
}

impl MulAssign<Rotor> for Motor {
  fn mul_assign(&mut self, r: Rotor) {
    self.p1 = gp11(self.p1,r.p1);
    self.p2 = gp12::<true>(r.p1,self.p1);
  }
}

impl Mul<Translator> for Motor {
  type Output = Motor;
  fn mul(self, t:Translator)->Motor {
    let p2 = gprt::<false>(self.p1,t.p2) + self.p2;
    Motor{p1:self.p1, p2:p2}
  }
}

impl MulAssign<Translator> for Motor {
  fn mul_assign(&mut self, t: Translator) {
    self.p1 = gprt::<false>(self.p1,t.p2) + self.p2
  }
}

impl Mul<Motor> for Motor {
  type Output = Motor;
  fn mul(self, other:Motor)->Motor {
    let (p1,p2) = gpmm(self.p1,other.p1);
    Motor{p1:p1, p2:p2}
  }
}

impl MulAssign<Motor> for Motor {
  fn mul_assign(&mut self, other:Motor) {
    let (p1,p2) = gpmm(self.p1,other.p1);
    self.p1 = p1; self.p2 = p2
  }
}

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
  fn div(self,other:Motor)->Motor {
    self * other.inverse()
  }
}

// TODO DivAssign ???
