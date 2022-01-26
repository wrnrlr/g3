use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg,Fn};
use core_simd::{f32x4,mask32x4,simd_swizzle};
use crate::{Motor,Translator,Point,Line,Plane,Branch,Direction};
use crate::sandwich::{sw01, swrl};
use crate::util::{add_ss, dp_bc, flip_signs, f32x4_xor, f32x4_abs, hi_dp_bc, rcp_nr1, rsqrt_nr1};
use crate::geometric::{gp11,gp12,gprt};


pub fn rotor(ang_rad:f32,x:f32,y:f32,z:f32)->Rotor {
  Rotor::new(ang_rad, x, y, z)
}

#[derive(Default,Debug,Clone,PartialEq)]
pub struct EulerAngels {
  pub roll:f32,
  pub pitch:f32,
  pub yaw:f32
}

// The rotor is an entity that represents a rigid rotation about an axis.
// To apply the rotor to a supported entity, the call operator is available.
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Rotor {
  // p1: scalar, e12, e31, e23
  pub p1:f32x4
}

impl Rotor {
  #[inline] pub fn scalar(&self)->f32 { self.p1[0] }
  #[inline] pub fn e12(&self)->f32 { self.p1[3] }
  #[inline] pub fn e21(&self)->f32 { -self.e12() }
  #[inline] pub fn e31(&self)->f32 { self.p1[2] }
  #[inline] pub fn e13(&self)->f32 { -self.e31() }
  #[inline] pub fn e23(&self)->f32 { self.p1[1] }
  #[inline] pub fn e32(&self)->f32 { -self.e23() }

  pub fn new(ang_rad:f32,x:f32,y:f32,z:f32)->Rotor {
    let norm  = (x*x + y*y + z*z).sqrt();
    let inv_norm = 1.0 / norm;
    let half = 0.5 * ang_rad;
    let sin_ang = half.sin();
    let scale = sin_ang * inv_norm;
    let p1 = f32x4::from_array([half.cos(),x,y,z]) * f32x4::from_array([1.0,scale,scale,scale]);
    Rotor{p1}
  }

  pub fn from_euler_angles(roll:f32,pitch:f32,yaw:f32)->Rotor {
    // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#cite_note-3
    let half_yaw = yaw * 0.5;
    let half_pitch = pitch * 0.5;
    let half_roll = roll * 0.5;
    let cos_y = half_yaw.cos();
    let sin_y = half_yaw.sin();
    let cos_p = half_pitch.cos();
    let sin_p = half_pitch.sin();
    let cos_r = half_roll.cos();
    let sin_r = half_roll.sin();

    Rotor{p1:f32x4::from_array([
      cos_r * cos_p * sin_y - sin_r * sin_p * cos_y,
      cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
      sin_r * cos_p * cos_y - cos_r * sin_p * sin_y,
      cos_r * cos_p * cos_y + sin_r * sin_p * sin_y])}
  }

  pub fn load_normalized(data:[f32;4])->Rotor {
    Rotor{ p1: f32x4::from(data) }
  }

  pub fn normalized(&self)->Rotor {
    let inv_norm = rsqrt_nr1(dp_bc(self.p1,self.p1));
    Rotor{p1: self.p1 * inv_norm}
  }

  pub fn inverse(&self)->Rotor {
    let inv_norm = rsqrt_nr1(hi_dp_bc(self.p1, self.p1));
    let mut p1 = self.p1 * inv_norm;
    p1 = p1 * inv_norm;
    p1 = flip_signs(p1, mask32x4::from_array([false,true,true,true]));
    Rotor{p1}
  }

  // Constrains the rotor to traverse the shortest arc
  pub fn constrained(&self)->Rotor {
    let mask = simd_swizzle!(-self.p1, [0,0,0,0]); // TODO
    let p1 =  f32x4_xor(mask,self.p1);
    Rotor{p1}
  }

  pub fn approx_eq(&self, other:Rotor, epsilon:f32)->bool {
    let eps = f32x4::splat(epsilon);
    f32x4_abs(self.p1 - other.p1) < eps
  }

  // Returns the principal branch of this rotor's logarithm. Invoking
  // `exp` on the returned `Branch` maps back to this rotor.
  //
  // Given a rotor $\cos\alpha + \sin\alpha\left[a\ee_{23} + b\ee_{31} +\
  // c\ee_{23}\right]$, the log is computed as simply
  // $\alpha\left[a\ee_{23} + b\ee_{31} + c\ee_{23}\right]$.
  // This map is only well-defined if the
  // rotor is normalized such that $a^2 + b^2 + c^2 = 1$.
  pub fn log(&self)->Branch {
    let cos_ang = self.p1[0];
    let ang = cos_ang.acos();
    let sin_ang = ang.sin();

    let mut p1  = self.p1 * rcp_nr1(f32x4::splat(sin_ang));
    p1 = p1 * f32x4::splat(ang);
    p1 = mask32x4::from_array([false, true, true, true]).select(p1, f32x4::splat(0.0));
    Branch{p1}
  }

  // Compute the square root of the provided rotor $r$.
  pub fn sqrt(&self)->Rotor {
    let p1 = add_ss(self.p1, f32x4::from([1.0, 0.0, 0.0, 0.0]));
    Rotor{p1}.normalized() // TODO avoid extra by normalize...
  }

  pub fn as_mat3x4() { todo!() }
  pub fn as_mat4x4() { todo!() }

  // TODO as_euler_angle
}

impl From<EulerAngels> for Rotor {
  fn from(ea:EulerAngels)->Self {
    Rotor::from_euler_angles(ea.roll,ea.pitch,ea.pitch)
  }
}

impl FnMut<(Plane,)> for Rotor { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane {self.call(args)} }
impl FnOnce<(Plane,)> for Rotor { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane { self.call(args) }}
impl Fn<(Plane,)> for Rotor {
  extern "rust-call" fn call(&self, args: (Plane,))->Plane {
    Plane{p0: sw01(args.0.p0, self.p1)}
  }
}

// TODO operator()(plane* in, plane* out, size_t count) const noexcept
// TODO operator()(branch const& b) const noexcept

impl FnMut<(Line,)> for Rotor { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line {self.call(args)} }
impl FnOnce<(Line,)> for Rotor { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line { self.call(args) }}
impl Fn<(Line,)> for Rotor {
  extern "rust-call" fn call(&self, args: (Line,))->Line {
    let (p1, p2) = swrl(args.0.p1, args.0.p2, self.p1);
    Line{p1, p2}
  }
}

// TODO operator()(line* in, line* out, size_t count) const noexcept

impl FnMut<(Point,)> for Rotor { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point {self.call(args)} }
impl FnOnce<(Point,)> for Rotor { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
impl Fn<(Point,)> for Rotor {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    Point{p3:sw01(args.0.p3, self.p1)}
  }
}

// TODO operator()(point* in, point* out, size_t count) const noexcept

impl FnMut<(Direction,)> for Rotor { extern "rust-call" fn call_mut(&mut self, args: (Direction,))->Direction {self.call(args)} }
impl FnOnce<(Direction,)> for Rotor { type Output = Direction; extern "rust-call" fn call_once(self, args: (Direction,))->Direction { self.call(args) }}
impl Fn<(Direction,)> for Rotor {
  extern "rust-call" fn call(&self, args: (Direction,))->Direction {
    Direction{p3: sw01(args.0.p3, self.p1)}
  }
}

// TODO operator()(direction* in, direction* out, size_t count) const noexcept

impl Add<f32> for Rotor {
  type Output = Rotor;
  fn add(self, s:f32) -> Rotor {
    Rotor{ p1: self.p1+f32x4::splat(s) }
  }
}

impl Add<Rotor> for f32 {
  type Output = Rotor;
  fn add(self, t:Rotor) -> Rotor {
    Rotor{ p1: t.p1+f32x4::splat(self) }
  }
}

impl Add<Rotor> for Rotor {
  type Output = Rotor;
  fn add(self, other: Rotor) -> Rotor { Rotor { p1:self.p1+other.p1 } }
}

impl AddAssign for Rotor {
  fn add_assign(&mut self, other: Self) { self.p1 = self.p1+other.p1 }
}

impl Sub<Rotor> for Rotor {
  type Output = Rotor;
  fn sub(self, other:Rotor) -> Rotor { Rotor { p1:self.p1-other.p1 } }
}

impl SubAssign for Rotor {
  fn sub_assign(&mut self, other: Self) { self.p1 = self.p1-other.p1 }
}

impl Mul<f32> for Rotor {
  type Output = Rotor;
  fn mul(self, s: f32) -> Rotor { Rotor { p1:self.p1*f32x4::splat(s) } }
}

impl MulAssign<f32> for Rotor {
  fn mul_assign(&mut self, s: f32) { self.p1 = self.p1*f32x4::splat(s) }
}

impl Div<f32> for Rotor {
  type Output = Rotor;
  fn div(self, s: f32) -> Rotor { Rotor { p1:self.p1/f32x4::splat(s) } }
}

impl DivAssign<f32> for Rotor {
  fn div_assign(&mut self, s: f32) { self.p1 = self.p1/f32x4::splat(s) }
}

// Reversion
impl Neg for Rotor {
  type Output = Self;
  fn neg(self)->Self::Output {
    Rotor { p1:flip_signs(self.p1, mask32x4::from_array([false,true,true,true])) }
  }
}

// TODO ~ flip all sign, the ~ is not available in rust ...

// impl Not for Rotor {
//   type Output = Self;
//   fn not(self)->Plane { Plane { p0: self.p3 }}
// }

// Geometric Product

impl Mul<Rotor> for Rotor {
  type Output = Rotor;
  fn mul(self,other:Rotor)->Self::Output {
    Rotor{p1: gp11(self.p1, other.p1)}
  }
}

impl Mul<Translator> for Rotor {
  type Output = Motor;
  fn mul(self,t:Translator)->Self::Output {
    Motor{p1: self.p1, p2: gprt(self.p1, t.p2)}
  }
}

impl Mul<Motor> for Rotor {
  type Output = Motor;
  fn mul(self,m:Motor)->Self::Output {
    Motor{p1: gp11(m.p1,self.p1), p2: gp12(self.p1,m.p2)}
  }
}

impl Div<Rotor> for Rotor {
  type Output = Rotor;
  fn div(self,other:Rotor)->Self::Output {
    self * other.inverse()
  }
}

impl Div<Translator> for Rotor {
  type Output = Motor;
  fn div(self,t:Translator)->Self::Output {
    self * t.inverse()
  }
}

impl Div<Motor> for Rotor {
  type Output = Motor;
  fn div(self,m:Motor)->Self::Output {
    self * m.inverse()
  }
}
