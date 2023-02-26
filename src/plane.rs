use std::{fmt::{Display, Formatter, Result},simd::{f32x4,mask32x4, SimdFloat},ops::*};
use crate::{*,maths::*};

/// e₀
pub const e0:Plane = plane(0.0, 0.0, 0.0, 1.0);
/// e₁
pub const e1:Plane = plane(1.0, 0.0, 0.0, 0.0);
/// e₂
pub const e2:Plane = plane(0.0, 1.0, 0.0, 0.0);
/// e₃
pub const e3:Plane = plane(0.0, 0.0, 1.0, 0.0);

/// ae₁ + be₂ + ce₃ + de₀
#[inline] pub const fn plane(a:f32,b:f32,c:f32,d:f32)->Plane { Plane::new(a,b,c,d) }

/// ae₁ + be₂ + ce₃ + de₀
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Plane (pub(crate) f32x4);

impl Plane {
  /// The constructor performs the rearrangement so the plane can be specified
  /// in the familiar form: ax + by + cz + d
  #[inline] pub const fn new(a:f32,b:f32,c:f32,d:f32)->Plane { Plane(f32x4::from_array([d,a,b,c]))}

  /// Normalize this plane $p$ such that $p \cdot p = 1$.
  /// In order to compute the cosine of the angle between planes via the
  /// inner product operator `|`, the planes must be normalized. Producing a
  /// normalized rotor between two planes with the geometric product `*` also
  /// requires that the planes are normalized.
  pub fn normalized(&self)->Plane {
    let mut inv_norm  = rsqrt_nr1(&hi_dp_bc(&self.0, &self.0));
    inv_norm[0] = 1.0;
    Plane(inv_norm * &self.0)
  }

  /// Compute the plane norm, which is often used to compute distances
  /// between points and lines.
  /// Given a normalized point $P$ and normalized line $\ell$, the plane
  /// $P\vee\ell$ containing both $\ell$ and $P$ will have a norm equivalent
  /// to the distance between $P$ and $\ell$.
  #[inline] pub fn norm(&self)->f32 {
    sqrt_nr1(&hi_dp(&self.0, &self.0))[0]
  }

  pub fn inverse(&self)->Plane {
    let inv_norm = &rsqrt_nr1(&hi_dp_bc(&self.0, &self.0));
    Plane(inv_norm * inv_norm * &self.0)
  }

  pub fn approx_eq(&self, other:Plane, epsilon:f32)->bool {(&self.0 - other.0).abs() < f32x4::splat(epsilon)}

  /// Project a plane onto a point. Given a plane $p$ and point $P$, produces the
  /// plane through $P$ that is parallel to $p$.
  /// Intuitively, the point is represented dually in terms of a _pencil of
  /// planes_ that converge on the point itself. When we compute $p | P$, this
  /// selects the line perpendicular to $p$ through $P$. Subsequently, taking the
  /// inner product with $P$ again selects the plane from the plane pencil of $P$
  /// _least like_ that line.
  pub fn project_point(self, a:Point)->Plane { (self | a) | a }

  /// Project a plane onto a line. Given a plane $p$ and line $\ell$, produces the
  /// plane through $\ell$ that is parallel to $p$ if $p \parallel \ell$.
  /// If $p \nparallel \ell$, the result will be the plane $p'$ containing $\ell$
  /// that maximizes $p \cdot p'$ (that is, $p'$ is as parallel to $p$ as
  /// possible).
  pub fn project_line(self, l:Line)->Plane {  (self | l) | l }

  #[inline] pub fn a(&self) ->f32 { self.0[1] }
  #[inline] pub fn b(&self) ->f32 { self.0[2] }
  #[inline] pub fn c(&self) ->f32 { self.0[3] }
  #[inline] pub fn d(&self)->f32 { self.0[0] }
  #[inline] pub fn e0(&self)->f32 { self.d() }
  #[inline] pub fn e1(&self)->f32 { self.a() }
  #[inline] pub fn e2(&self)->f32 { self.b() }
  #[inline] pub fn e3(&self)->f32 { self.c() }
}

impl Display for Plane {fn fmt(&self, f: &mut Formatter<'_>) -> Result {write!(f, "{}e1 + {}e2 + {}e2 + {}e0)", self.a(), self.b(), self.c(), self.d())}}

/// Reflect another plane $p_2$ through this plane $p_1$. The operation
/// performed via this call operator is an optimized routine equivalent to
/// the expression $p_1 p_2 p_1$.
impl FnMut<(Plane,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane { self.call(args) }}
impl FnOnce<(Plane,)> for Plane { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane {self.call(args)} }
impl Fn<(Plane,)> for Plane {extern "rust-call" fn call(&self, args: (Plane,))->Plane { Plane(sw00(&self.0, &args.0.0)) } }

/// Reflect the point $P$ through this plane $p$. The operation
/// performed via this call operator is an optimized routine equivalent to
/// the expression $p P p$.
impl FnMut<(Point,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point {self.call(args)} }
impl FnOnce<(Point,)> for Plane { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
impl Fn<(Point,)> for Plane {extern "rust-call" fn call(&self, args: (Point,))->Point { Point(sw30(&self.0, &args.0.0)) } }

/// Reflect line $\ell$ through this plane $p$. The operation
/// performed via this call operator is an optimized routine equivalent to
/// the expression $p \ell p$.
impl FnMut<(Line,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line {self.call(args)} }
impl FnOnce<(Line,)> for Plane { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line { self.call(args) }}
impl Fn<(Line,)> for Plane { extern "rust-call" fn call(&self, args: (Line,)) -> Line { let (p1, p2) = sw10(&self.0, &args.0.p1);Line{p1,p2:p2 + sw20(&self.0, &args.0.p2)} } }

impl Add<f32> for Plane {type Output = Plane;fn add(self, s: f32) -> Plane { self+plane(s,s,s,s)}}
impl Add<i32> for Plane {type Output = Plane;fn add(self, s: i32) -> Plane { self+plane(s as f32,s as f32,s as f32,s as f32)}}

impl Add<Plane> for Plane {type Output = Plane;fn add(self, p: Plane) -> Plane { Plane(self.0+p.0)}}
impl AddAssign for Plane {fn add_assign(&mut self, p: Self) { self.0 += p.0}}
impl Sub<Plane> for Plane {type Output = Plane;fn sub(self, p: Plane) -> Plane { Plane(self.0-p.0)}}
impl SubAssign for Plane {fn sub_assign(&mut self, p: Self) { self.0 -= p.0}}
impl Mul<f32> for Plane {type Output = Plane;fn mul(self, s: f32) -> Plane { Plane(self.0*f32x4::splat(s))}}
impl Mul<i32> for Plane {type Output = Plane;fn mul(self, s: i32) -> Plane { Plane(self.0*f32x4::splat(s as f32))}}
impl Mul<Plane> for f32 {type Output = Plane;fn mul(self, p: Plane) -> Plane { p*self }}
impl Mul<Plane> for i32 {type Output = Plane;fn mul(self, p: Plane) -> Plane { p*self }}
impl MulAssign<f32> for Plane {fn mul_assign(&mut self, s: f32) { self.0 *= f32x4::splat(s)}}
impl Div<f32> for Plane {type Output = Plane;fn div(self, s: f32) -> Plane { Plane(self.0/f32x4::splat(s))}}
impl DivAssign<f32> for Plane {fn div_assign(&mut self, s: f32) { self.0 /= f32x4::splat(s)}}
/// Unary minus (leaves displacement from origin untouched, changing orientation only)
impl Neg for Plane {type Output = Self;fn neg(self)->Self::Output { Plane(flip_signs(&self.0, [false,true,true,true].into()) ) } }
/// Geometric Product *
impl Mul<Point> for Plane { type Output = Motor;fn mul(self, a: Point) -> Motor { let (p1,p2) = gp03(&self.0,&a.0);Motor{p1,p2} } }
impl Mul<Plane> for Plane { type Output = Motor;fn mul(self, p: Plane) -> Motor { let (p1,p2) = gp00(&self.0,&p.0);Motor{p1,p2} } }
impl Div<Plane> for Plane {type Output = Motor;fn div(self, p: Plane) -> Motor {self * p.inverse()}}
/// Inner Product |
impl BitOr<Plane> for Plane {type Output = f32;fn bitor(self, p:Plane) -> f32 { dot00(&self.0,&p.0)[0]}}
impl BitOr<Line> for Plane {type Output = Plane;fn bitor(self, l:Line) -> Plane {let p0 = dotpl(&self.0,&l.p1,&l.p2);Plane(p0)}}
impl BitOr<Horizon> for Plane {type Output = Plane;fn bitor(self, l: Horizon) -> Plane {let p0 = dotpil(&self.0,&l.p2);Plane(p0)}}
impl BitOr<Point> for Plane {type Output = Line;fn bitor(self, a:Point) -> Line { let (p1,p2) = dot03(&self.0,&a.0);Line{p1,p2}} }
/// Meet Operator, Exterior Product, ^
impl BitXor<Plane> for Plane {type Output = Line;fn bitxor(self, p:Plane) -> Line {let (p1,p2) = ext00(&self.0,&p.0);Line{p1,p2}} }
impl BitXor<Line> for Plane {type Output = Point;fn bitxor(self, l:Line) -> Point {Point(extpb(&self.0,&l.p1)+ext02(&self.0,&l.p2))}}
impl BitXor<Horizon> for Plane {type Output = Point;fn bitxor(self, l: Horizon) -> Point { Point(ext02(&self.0, &l.p2)) }}
impl BitXor<Branch> for Plane {type Output = Point;fn bitxor(self, b:Branch) -> Point {Point(extpb(&self.0, &b.0)) }}
/// (a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123
impl BitXor<Point> for Plane {type Output = Dual;fn bitxor(self, p:Point) -> Dual{Dual::new(0.0, dp(&self.0,&p.0)[0])}}
impl BitAnd<Point> for Plane {type Output = Dual;fn bitand(self, p: Point) -> Dual {!(!self ^ !p)}}
impl Not for Plane {type Output = Point;fn not(self)->Point{Point(self.0)}}

// TODO this is not in klein
// impl Div<Point> for Plane {
//   type Output = Motor;
//   fn div(self, a: Point) -> Motor {
//     self * a.inverse()
//   }
// }

// plane * plane
fn gp00(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (a1b1 + a2b2 + a3b3) + (a2b3 - a3b2)e23 + (a3b1 - a1b3)e31 + (a1b2 - a2b1)e12 +
  // (a0b1 - a1b0)e01 + (a0b2 - a2b0)e02 + (a0b3 - a3b0)e03
  let mut p1_out = a.yzwy() * b.ywyz();
  p1_out = p1_out - (f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(a.zwyz() * b.zzwy())));
  // Add a3 b3 to the lowest component
  p1_out = add_ss(&p1_out, &(a.wxxx() * b.wxxx()));
  // (a0 b0, a0 b1, a0 b2, a0 b3)
  let mut p2_out = a.xxxx() * b;
  // Sub (a0 b0, a1 b0, a2 b0, a3 b0)
  // Note that the lowest component cancels
  p2_out = p2_out - a * b.xxxx();
  return (p1_out, p2_out);
}

// a1 b1 + a2 b2 + a3 b3
fn dot00(a:&f32x4, b:&f32x4)->f32x4 {
  hi_dp(a,b)
}

// (a2b1 - a1b2)e03 + (a3b2 - a2b3)e01 + (a1b3 - a3b1)e02 + a1b0e23 + a2b0e31 + a3b0e12
fn dot03(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let mut p1_out = a * b.xxxx();
  p1_out = zero_first(p1_out);
  (p1_out, (a.xzwy()*b - a*b.xzwy()).xzwy())
}

fn dotpl(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  let mut p0 = a.xzwy() * b;
  p0 -= a * b.xzwy();
  sub_ss(&(p0.xzwy()), hi_dp_ss(a, c))
}

fn dotpil(a:&f32x4, c:&f32x4)->f32x4 {
  f32x4_xor(&hi_dp(a, c), &[-0.0, 0.0, 0.0, 0.0].into())
}

// (a1 b2 - a2 b1) e12 + (a2 b3 - a3 b2) e23 + (a3 b1 - a1 b3) e31 +
// (a0 b1 - a1 b0) e01 + (a0 b2 - a2 b0) e02 + (a0 b3 - a3 b0) e03
fn ext00(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // For both outputs above, we don't zero the lowest component because
  // we've arranged a cancelation TODO wdym???
  ((a * b.xzwy() - a.xzwy() * b).xzwy(),
   a.xxxx() * b - a * b.xxxx())
}

// p0 ^ p2 = p2 ^ p0
fn ext02(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b2 - a2 b1) e021 + (a2 b3 - a3 b2) e032 + (a3 b1 - a1 b3) e013 +
  (a * b.xzwy() - a.xzwy() * b).xzwy()
}

fn extpb(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e123 + (-a0 b1) e032 + (-a0 b2) e013 + (-a0 b3) e021
  let p3_out = &flip_signs(&(a.yxxx() * b), mask32x4::from_array([false,true,true,true]));
  return add_ss(p3_out, &hi_dp(a,b));
}

// reflect point through plane
fn sw30(a:&f32x4, b:&f32x4) ->f32x4 {
  //                                b0(a1^2 + a2^2 + a3^2)  e123 +
  // (-2a1(a0 b0 + a3 b3 + a2 b2) + b1(a2^2 + a3^2 - a1^2)) e032 +
  // (-2a2(a0 b0 + a1 b1 + a3 b3) + b2(a3^2 + a1^2 - a2^2)) e013 +
  // (-2a3(a0 b0 + a2 b2 + a1 b1) + b3(a1^2 + a2^2 - a3^2)) e021

  let a_zwyz = a.zwyz(); // a2, a3, a1, a2
  let a_yzwy = a.yzwy(); // a1, a2, a3, a1
  let a_wyzw = a.wyzw(); // a3, a1, a2, a3

  //     a0 b0              |      a0 b0              |      a0 b0              |      a0 b0
  let mut p3_out = a.xxxx() * b.xxxx();
  //     a0 b0+a2 b0        |      a0 b0+a3 b3        |      a0 b0+a1 b1        |      a0 b0+a3 b2
  p3_out += a_zwyz * b.xwyz();
  //     a0 b0+a2 b0+a1 b0  |      a0 b0+a3 b3+a2 b2  |      a0 b0+a1 b0+a3 b3  |      a0 b0+a3 b2+a1 b1
  p3_out += a_yzwy * b.xzwy();
  // 0b0(a0 b0+a2 b0+a1 b0) | -2a1(a0 b0+a3 b3+a2 b2) | -2a2(a0 b0+a1 b0+a3 b3) | -2a3(a0 b0+a3 b2+a1 b1)
  p3_out *= a * f32x4::from_array([0.0,-2.0,-2.0,-2.0]);
  //                        | -2a1(a0 b0+a3 b3+a2 b2) | -2a2(a0 b0+a1 b0+a3 b3) | -2a3(a0 b0+a3 b2+a1 b1)

  // a1^2           | a2^2           | a3^2           | a1^2
  let mut tmp = a_yzwy * a_yzwy;
  // a1^2+a2^2      | a2^2+a3^2      | a3^2+a1^2      | a1^2+a2^2
  tmp += a_zwyz * a_zwyz;
  // a1^2+a2^2+a3^2 | a2^2+a3^2-a1^2 | a3^2+a1^2-a2^2 | a1^2+a2^2-a3^2
  tmp -= f32x4_xor(&(a_wyzw * a_wyzw), &f32x4::from_array([-0.0,0.0,0.0,0.0]));

  p3_out = p3_out + b * tmp;

  p3_out
}

// Reflect a plane through another plane
// b * a * b
fn sw00(a:&f32x4,b:&f32x4)->f32x4 {
  // (2a0(a2 b2 + a3 b3 + a1 b1) - b0(a1^2 + a2^2 + a3^2)) e0 +
  // (2a1(a2 b2 + a3 b3)         + b1(a1^2 - a2^2 - a3^2)) e1 +
  // (2a2(a3 b3 + a1 b1)         + b2(a2^2 - a3^2 - a1^2)) e2 +
  // (2a3(a1 b1 + a2 b2)         + b3(a3^2 - a1^2 - a2^2)) e3
  let a_zzwy = a.zzwy();
  let a_wwyz = a.wwyz();

  // Left block
  let mut tmp = a_zzwy * b.zzwy();
  tmp += a_wwyz * b.wwyz();

  let a1 = &a.yyww();
  let b1 = &b.yyww();
  tmp = add_ss(&tmp, &mul_ss(a1, b1));
  tmp *= a + a;

  // Right block
  let a_yyzw = &a.yyzw();
  let mut tmp2 = f32x4_xor(&(a_yyzw * a_yyzw), &[-0.0, 0.0, 0.0, 0.0].into());
  tmp2 -= a_zzwy * a_zzwy;
  tmp2 -= a_wwyz * a_wwyz;
  tmp2 *= b;

  tmp + tmp2
}

fn sw10(a:&f32x4,b:&f32x4)->(f32x4,f32x4) {
  //                       b0(a1^2 + a2^2 + a3^2) +
  // (2a3(a1 b1 + a2 b2) + b3(a3^2 - a1^2 - a2^2)) e12 +
  // (2a1(a2 b2 + a3 b3) + b1(a1^2 - a2^2 - a3^2)) e23 +
  // (2a2(a3 b3 + a1 b1) + b2(a2^2 - a3^2 - a1^2)) e31 +
  //
  // 2a0(a1 b2 - a2 b1) e03
  // 2a0(a2 b3 - a3 b2) e01 +
  // 2a0(a3 b1 - a1 b3) e02

  let a_zyzw = &a.zyzw();
  let a_ywyz = &a.ywyz();
  let a_wzwy = &a.wzwy();
  let b_xzwy = &b.xzwy();

  let two_zero:f32x4 = [0.0, 2.0, 2.0, 2.0].into(); // TODO is this right?
  let mut p1 = a * b;
  p1 += a_wzwy * b_xzwy;
  p1 *= a_ywyz * two_zero;

  let mut tmp = a_zyzw * a_zyzw;
  tmp += a_wzwy * a_wzwy;
  tmp = f32x4_xor(&tmp, &[-0.0, 0.0, 0.0, 0.0].into());
  tmp = (a_ywyz * a_ywyz) - tmp;
  tmp = b.xwyz() * tmp;

  let p1 = (p1 + tmp).xzwy();

  let mut p2 = a_zyzw * b_xzwy;
  p2 = p2 - a_wzwy * b;
  p2 = p2 * a.xxxx() * two_zero;
  p2 = p2.xzwy();

  (p1,p2)
}

fn sw20(a:&f32x4,b:&f32x4)->f32x4 {
  //                       -b0(a1^2 + a2^2 + a3^2) e0123 +
  // (-2a3(a1 b1 + a2 b2) + b3(a1^2 + a2^2 - a3^2)) e03
  // (-2a1(a2 b2 + a3 b3) + b1(a2^2 + a3^2 - a1^2)) e01 +
  // (-2a2(a3 b3 + a1 b1) + b2(a3^2 + a1^2 - a2^2)) e02 +
  let a_zzwy = a.zzwy();
  let a_wwyz = a.wwyz();

  let mut p2 = a * b;
  p2 += a_zzwy * b.xzwy();
  p2 *= a_wwyz * &[0.0, -2.0, -2.0, -2.0].into();

  let a_yyzw = a.yyzw();
  let mut tmp = a_yyzw * a_yyzw;
  tmp = f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(tmp + a_zzwy * a_zzwy));
  tmp -= a_wwyz * a_wwyz;
  p2 += tmp * b.xwyz();
  p2.xzwy()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn plane_new() { assert_eq!(e1, plane(1.0,0.0,0.0,0.0)) }
  #[test] fn plane_eq() { assert_eq!(e1, e1); assert_eq!(e1, 1*e1); assert_eq!(e1, e1*1.0) }
  #[test] fn plane_approx_eq() {
    let a  = plane(1.0, 1.0, 1.0, 1.0);
    let b = plane(0.9, 0.9, 0.9, 0.9);
    let c = plane(0.8, 0.8, 0.8, 0.8);
    assert_eq!(a.approx_eq(b, 0.1001), true, "{:?} eq {:?} approx 0.1", a.0, b.0);
    assert_eq!(a.approx_eq(b, 0.099), false, "{:?} eq {:?} approx 0.11", a.0, b.0);
    assert_eq!(a.approx_eq(c, 0.1), false, "{:?} eq {:?} approx 0.09", a.0, c.0);
    assert_eq!(a.approx_eq(c, 0.2), true, "{:?} eq {:?} approx 0.1", a.0, c.0);
    let a1  = plane(1.0, 2.0, 3.0, 4.0);
    let b1 = plane(0.9, 2.0, 3.0, 4.0);
    assert_eq!(a1.approx_eq(b1, 0.1001), true, "{:?} eq {:?} approx 0.1", a1.0, b1.0);
  }
  #[test] fn plane_getters() { assert_eq!([e1.e1(),e2.e2(),e3.e3(),e0.e0()], [1.0,1.0,1.0,1.0]) }
  #[test] fn plane_abcd() { assert_eq!([e1.a(),e2.b(),e3.c(),e0.d()], [1.0,1.0,1.0,1.0]) }
  #[test] fn plane_add() { assert_plane(e1+e2, 1.0,1.0,0.0,0.0) }
  #[test] fn plane_add_assign() { let mut p = e1; p += e2; assert_plane(p, 1.0,1.0,0.0,0.0) }
  #[test] fn plane_sub() { assert_plane(plane(2.0,4.0,6.0,8.0)-plane(1.0,2.,3.,4.0), 1.0,2.0,3.0,4.0) }
  #[test] fn plane_sub_assign() {
    let mut p = plane(2.0,4.0,6.0,8.0);p -= plane(1.0,2.0,3.0,4.0);
    assert_plane(p, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_mul_scalar() {
    assert_plane(plane(1.0,2.0,3.0,4.0)*2.0, 2.0,4.0,6.0,8.0);
  }
  #[test] fn plane_mul_assign_scalar() {
    let mut p = plane(1.0,2.0,3.0,4.0); p *= 2.0;
    assert_plane(p, 2.0,4.0,6.0,8.0);
  }
  #[test] fn plane_div_scalar() {
    assert_plane(plane(2.0,4.0,6.0,8.0)/2.0, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_div_assign_scalar() {
    let mut p = plane(2.0,4.0,6.0,8.0);
    p /= 2.0;
    assert_plane(p, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_negative() {
    let p = plane(1.0,2.0,3.0,4.0);
    assert_plane(-p, -1.0,-2.0,-3.0,4.0);
  }
  #[test] #[ignore] fn plane_normalized() {}
  #[test] #[ignore] fn plane_invserse() {}
  #[test] #[ignore] fn plane_reverse() {}
  #[test] fn plane_not() {
    let a = !plane(4.0, 3.0, 2.0, 1.0);
    assert_eq!(a.0, [1.0,4.0,3.0,2.0].into());
  }

  #[test] fn planes() {
    let p1 = plane(1.0, 3.0, 4.0, -5.0);
    assert_ne!(p1 | p1, 1.0);
    let p2 = p1.normalized();
    approx_eq1(p2 | p2, 1.0);
  }

  fn assert_plane(p:Plane,x:f32,y:f32,z:f32,d:f32) {
    assert_eq!(p.a(), x);
    assert_eq!(p.b(), y);
    assert_eq!(p.c(), z);
    assert_eq!(p.d(), d);
  }

  #[test] fn meet_plane_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let p2 = plane(2.0, 3.0, -1.0, -2.0);
    let l = p1 ^ p2;
    assert_eq!([l.e01(), l.e02(), l.e03()], [10.0, 16.0, 2.0]);
    assert_eq!([l.e12(), l.e31(), l.e23()], [-1.0, 7.0, -11.0]);
  }

  #[test] fn meet_plane_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(0.0, 0.0, 1.0, 4.0, 1.0, -2.0);
    let a = p ^ l;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [8.0, -5.0, -14.0, 0.0]);
  }

  #[test] fn meet_plane_horizon() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = horizon(-2.0, 1.0, 4.0);
    let a = p ^ l;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [5.0, -10.0, 5.0, 0.0]);
  }

  #[test] fn meet_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(-2.0, 1.0, 4.0);
    let d =  p ^ a;
    assert_eq!([d.scalar(), d.e0123()], [0.0, 16.0]);
  }

  const EPSILON: f32 = 0.02;
  fn approx_eq1(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}", a, b);
  }

  #[test] fn measure_point_to_plane() {
    //    Plane p2
    //    /
    //   / \ line perpendicular to
    //  /   \ p2 through p1
    // 0------x--------->
    //        p1
    let a = point(2.0, 0.0, 0.0);
    let p = plane(1.0, -1.0, 0.0, 0.0).normalized();
    // Distance from point p1 to plane p2
    let root_two = 2f32.sqrt();
    approx_eq1((a & p).scalar().abs(), root_two);
    approx_eq1((a ^ p).e0123().abs(), root_two);
  }

  #[test] fn reflect_pane() {
    let p1 = plane(3.0, 2.0, 1.0, -1.0);
    let p2 = plane(1.0, 2.0, -1.0, -3.0);
    let p3 = p1(p2);
    assert_eq!([p3.e0(), p3.e1(), p3.e2(), p3.e3()], [30.0, 22.0, -4.0, 26.0]);
  }

  #[test] fn reflect_line() {
    let p1 = plane(3.0, 2.0, 1.0, -1.0);
    let l1 = line(1.0, -2.0, 3.0, 6.0, 5.0, -4.0);
    let l2 = p1(l1);
    assert_eq!([l2.e01(), l2.e02(), l2.e03(), l2.e12(), l2.e31(), l2.e23()],
               [28.0, -72.0, 32.0, 104.0, 26.0, 60.0]);
  }

  #[test] fn reflect_point() {
    let p = plane(3.0, 2.0, 1.0, -1.0);
    let a = point(4.0, -2.0, -1.0);
    let b = p(a);
    assert_eq!([b.e021(), b.e013(), b.e032(), b.e123()], [-26.0, -52.0, 20.0, 14.0]);
  }
}
