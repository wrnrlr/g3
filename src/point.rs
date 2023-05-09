use std::{fmt::{Display, Formatter, Result},simd::{f32x4,mask32x4,simd_swizzle,SimdFloat},mem::transmute,ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg}};
use crate::{*,maths::*};

/// e₀₃₂ + e₁₂₃
pub const X:Point = point(1.0, 0.0, 0.0);
/// e₀₁₃ + e₁₂₃
pub const Y:Point = point(0.0, 1.0, 0.0);
/// e₀₂₃ + e₁₂₃
pub const Z:Point = point(0.0, 0.0, 1.0);
/// Origin
pub const O:Point = point(0.0, 0.0, 0.0);
/// Origin
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Origin {} impl Into<Point> for Origin { fn into(self)->Point { Point::new(0.0,0.0,0.0) } }

/// xe₀₃₂ + ye₀₁₃ + ze₀₂₁ + e₁₂₃
pub const fn point(x:f32,y:f32,z:f32)->Point { Point::new(x,y,z) }

/// xe₀₃₂ + ye₀₁₃ + ze₀₂₁ + e₁₂₃
#[cfg_attr(feature = "bytemuck", repr(C), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Point (pub(crate) f32x4);

impl Point {
  /// Component-wise constructor where homogeneous coordinate is automatically initialized to 1.
  pub const fn new(x:f32,y:f32,z:f32)->Self{ Point(f32x4::from_array([1.0,x,y,z])) }
  /// x/w + y/w + z/w + w/w
  pub fn normalized(&self)->Self{Self(&self.0 * rcp_nr1(&self.0.xxxx()))}
  /// x(1/w) + y(1/w) + z(1/w) + w(1/w)
  pub fn inverse(&self)->Self { let inv_norm = &rcp_nr1(&(self.0.xxxx()));Self(inv_norm * inv_norm * &self.0) }
  pub fn reverse(&self)->Point { Point(flip_signs(&self.0, mask32x4::from_array([false,true,true,true]))) }
  /// Project a point onto a line
  pub fn project_line(self, l:Line)->Point { (self | l) ^ l }
  /// Project a point onto a plane
  pub fn project_plane(self, p:Plane)->Point { (self | p) ^ p }
  pub fn approx_eq(&self, other:Point, epsilon:f32)->bool {(&self.0 - other.0).abs() < f32x4::splat(epsilon)}
  #[inline] pub fn x(&self)->f32 { self.0[1] }
  #[inline] pub fn y(&self)->f32 { self.0[2] }
  #[inline] pub fn z(&self)->f32 { self.0[3] }
  #[inline] pub fn w(&self)->f32 { self.0[0] }
  #[inline] pub fn e032(&self)->f32 { self.x() }
  #[inline] pub fn e013(&self)->f32 { self.y() }
  #[inline] pub fn e021(&self)->f32 { self.z() }
  #[inline] pub fn e123(&self)->f32 { self.w() }
}
impl GeometricProduct for Point {} impl JoinProduct for Point {} impl MeetProduct for Point {}
/// Dual operator
impl Not for Point {type Output = Plane;fn not(self)->Plane {Plane(self.0)}}
/// Unary minus (leaves homogeneous coordinate untouched)
impl Neg for Point {type Output = Self;fn neg(self)->Point{Point(self.0 * <f32x4>::from([1.0, -1.0, -1.0, -1.0]))}}
impl Add for Point {type Output=Self;fn add(self,p:Self)->Self{Self(self.0+p.0)}}
impl Add<f32> for Point {type Output = Point;fn add(self, s: f32) -> Point { self+point(s,s,s)}}
impl Add<i32> for Point {type Output = Point;fn add(self, s: i32) -> Point { self+point(s as f32,s as f32,s as f32)}}
impl Sub for Point {type Output=Self;fn sub(self,p:Self)->Self{Self(self.0-p.0)}}
impl AddAssign for Point {fn add_assign(&mut self,p:Self){self.0+=p.0}}
impl SubAssign for Point {fn sub_assign(&mut self,p:Self){self.0-=p.0}}
/// Point uniform scale
impl Mul<f32> for Point {type Output=Self;fn mul(self, s:f32) -> Self { Point(self.0*f32x4::splat(s)) } }
impl Mul<i32> for Point {type Output=Self;fn mul(self, s:i32) -> Self { self * s as f32 } }
/// Point uniform inverse scale
impl Div<f32> for Point {type Output=Self;fn div(self, s:f32) -> Self { Point(self.0 / f32x4::splat(s)) } }
impl MulAssign<f32> for Point {fn mul_assign(&mut self, s: f32) { self.0 = self.0 * f32x4::splat(s) }}
impl DivAssign<f32> for Point {fn div_assign(&mut self, s: f32) { self.0 = self.0 / f32x4::splat(s) }}
impl Mul<Point> for i32 {type Output=Point;fn mul(self, p:Point) -> Point { p*self as f32 } }
impl Mul<Point> for f32 {type Output=Point;fn mul(self, p:Point) -> Point { p*self } }
impl Mul<Point> for Point {type Output=Translator;fn mul(self,p:Point)->Translator{Translator{p2:gp33(&self.0, &p.0)}}}
impl Mul<Plane> for Point {type Output=Motor;fn mul(self,p:Plane)->Motor{let(p1,p2)=gp30(&p.0,&self.0);Motor{p1,p2}}}
impl Div<Point> for Point {type Output=Translator;fn div(self,p:Point)->Translator{self*p.inverse()}}
impl BitOr<Plane> for Point {type Output=Line;fn bitor(self, p:Plane) -> Line { p | self }}
impl BitOr<Line> for Point {type Output=Plane;fn bitor(self, l:Line) -> Plane { Plane(dotptl(&self.0,&l.p1)) }}
/// a|b = -a0 b0
impl BitOr<Point> for Point {type Output=f32;fn bitor(self, a:Point) -> f32 {let out = dot33(&self.0,&a.0);out[0]}}
/// a^b = (a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123
impl BitXor<Plane> for Point {type Output=Dual;fn bitxor(self, p:Plane) -> Dual {let out = -dp(&p.0,&self.0);Dual::new(0.0,out[0])} }
impl BitAnd<Point> for Point {type Output=Line;fn bitand(self, p: Point) -> Line { !(!self ^ (!p))}}
impl BitAnd<Line> for Point {type Output=Plane;fn bitand(self, l: Line) -> Plane { !(!self ^ !l)}}
impl BitAnd<Horizon> for Point {type Output=Plane;fn bitand(self, l: Horizon) -> Plane { !(!self ^ !l)}}
impl BitAnd<Branch> for Point {type Output=Plane;fn bitand(self, b: Branch) -> Plane { !(!self ^ !b)}}
impl BitAnd<Plane> for Point {type Output=Dual;fn bitand(self, p: Plane)->Dual { !(!self ^ !p)}}
impl Display for Point { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "(x:{}, y:{}, z:{}, w:{})", self.x(), self.y(), self.z(), self.w()) } }
/// Convert point to an array
impl From<Point> for [f32;3] { fn from(p:Point) -> Self {[p.x(), p.y(), p.z()]} }
/// Convert array to a point
impl From<[f32;3]> for Point { fn from([x,y,z]:[f32;3]) -> Self {point(x, y, z)} }
/// Convert point to a tuple
impl From<Point> for (f32,f32,f32) { fn from(p:Point) -> Self {(p.x(), p.y(), p.z())} }
/// Convert tuple to a point
impl From<(f32,f32,f32)> for Point { fn from((x,y,z):(f32,f32,f32)) -> Self {point(x, y, z)} }
/// Returns `&[x,y,z,w]`
impl From<&Point> for [f32;4] { #[inline(always)] fn from(v: &Point) -> Self { unsafe { transmute::<f32x4,[f32;4]>(simd_swizzle!(v.0, [1,2,3,0])) } } }
/// Returns `[x,y,z,w]`
impl From<Point> for [f32;4] { #[inline(always)] fn from(v: Point) -> Self { unsafe { transmute::<f32x4,[f32;4]>(simd_swizzle!(v.0, [1,2,3,0])) } } }
#[cfg(feature = "mint")] impl From<mint::Point2<f32>> for Point { #[inline] fn from(v: mint::Point2<f32>)->Point { Self::new(v.x,v.y,0.0) } }
#[cfg(feature = "mint")] impl From<Point> for mint::Point2<f32> { #[inline] fn from(v: Point) -> Self { Self { x: v.x(), y: v.y() } } }
#[cfg(feature = "mint")] impl From<mint::Point3<f32>> for Point { #[inline] fn from(v: mint::Point3<f32>)->Point { Self::new(v.x,v.y,v.z) } }
#[cfg(feature = "mint")] impl From<Point> for mint::Point3<f32> { #[inline] fn from(v: Point) -> Self { Self { x: v.x(), y: v.y(), z: v.z() } } }

// TODO Does not exist in klein?
// impl Div<Plane> for Point { type Output = Motor;fn div(self, p: Plane) -> Motor { other.invert();self * other } }
//Do not exist because
// impl BitXor<Line> for Point { type Output = Point;fn bitxor(self, l:Line) -> Point {} }}
// impl BitXor<Point> for Point { type Output = Dual;fn bitxor(self, a:Point) -> Dual {} }

fn dot33(a:&f32x4, b:&f32x4)->f32x4 {
  // -a0 b0
  f32x4::from_array([-1.0, 0.0, 0.0, 0.0]) * mul_ss(a, b)
}

fn dotptl(a:&f32x4, b:&f32x4)->f32x4 {
  let dp = &hi_dp_ss(a, b);
  let p0 = &a.xxxx() * b;
  let p0 = &f32x4_xor(&p0, &[0.0, -0.0, -0.0, -0.0].into());
  add_ss(p0, dp)
}

fn gp33(a:&f32x4, b:&f32x4)->f32x4 {
  // (-a0 b0) +
  // (-a0 b1 + a1 b0) e01 +
  // (-a0 b2 + a2 b0) e02 +
  // (-a0 b3 + a3 b0) e03
  let tmp = a.xxxx() * b * f32x4::from_array([-2.0, -1.0, -1.0, -1.0]) + a * b.xxxx();
  tmp * rcp_nr1(&tmp.xxzz().xyxy())
  // TODO, in klein their is an extra `and`
  // flip_signs(tmp, mask32x4::from([false, true, true, true]))
  // mask32x4::from_array([false, true, true, true]).select(tmp, f32x4::splat(0.0))
  // f32x4_and(tmp, f32x4::from([0.0, -1.0, -1.0, -1.0]))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn point_constructor() {
    assert_eq!(Point::new(1.0,2.0,3.0), point(1.0, 2.0, 3.0))
  }
  #[test] fn point_eq() { assert_eq!(X, X*1);assert_eq!(X, X*1.0); }
  #[test] fn point_getters() {
    assert_eq!([X.x(), Y.y(), Z.z(), X.w()], [1.0, 1.0, 1.0, 1.0]);
    assert_eq!([X.e032(), Y.e013(), Z.e021(), X.e123()], [1.0,1.0,1.0,1.0]);
  }
  #[test] fn point_add() { assert_eq!(X + X, 2 * X) }
  #[test] fn point_add_assign() {
    let mut p = point(1.0, 2.0, 3.0);
    p += point(1.0, 2.0, 3.0);
    assert_point(p, 2.0, 4.0, 6.0, 2.0)
  }
  #[test] fn point_sub() {
    assert_point(point(2.0,4.0,6.0)-point(1.0,2.,3.), 1.0,2.0,3.0,0.0)
  }
  #[test] fn point_sub_assign() {
    let mut p = point(2.0,4.0,6.0);
    p -= point(1.0,2.0,3.0);
    assert_point(p, 1.0,2.0,3.0,0.0);
  }
  #[test] fn point_mul() {
    assert_point(point(1.0, 2.0, 3.0)*2.0, 2.0,4.0,6.0,2.0);
  }
  #[test] fn point_mul_assign() {
    let mut p = point(1.0, 2.0, 3.0);
    p *= 2.0;
    assert_point(p, 2.0, 4.0, 6.0, 2.0);
  }
  #[test] fn point_div() {
    assert_point(point(2.0, 4.0, 6.0)/2.0, 1.0,2.0,3.0,0.5);
  }
  #[test] fn point_div_assign() {
    let mut p = point(2.0, 4.0, 6.0);
    p /= 2.0;
    assert_point(p, 1.0, 2.0, 3.0, 0.5);
  }
  #[test] fn point_negative() {
    assert_point(-point(1.0, 2.0, 3.0), -1.0, -2.0, -3.0, 1.0);
  }
  #[test] fn point_reverse() {
    let p = point(1.0, 2.0, 3.0);
    assert_point(p.reverse(), -1.0, -2.0, -3.0, 1.0);
  }
  // #[test] fn point_not() {}
  #[test] #[ignore] fn point_normalized() {}
  #[test] #[ignore] fn point_inverse() {}

  fn assert_point(p:Point,x:f32,y:f32,z:f32,w:f32) {
    assert_eq!([p.x(), p.y(), p.z(), p.w()],[x, y, z, w]);
  }

  #[test] fn point_xyzw() {
    assert_eq!(&<[f32;4]>::from(&point(4.0,3.0,2.0)), &[4f32,3.0,2.0,1.0]);
    assert_eq!(<[f32;4]>::from(point(4.0,3.0,2.0)), [4f32,3.0,2.0,1.0]);
  }
  #[test] fn z_line() { assert_eq!((O & Z).e12(), 1.0); }

  #[test] fn y_line() { assert_eq!((-Y & O).e31(), 1.0); }

  #[test] fn x_line() { assert_eq!((-2.0*X & -X).normalized().e23(), 1.0); }

  #[test] fn plane_construction() {
    let a = point(1.0, 3.0, 2.0);
    let b = point(-1.0, 5.0, 2.0);
    let c = point(2.0, -1.0, -4.0);
    let p = a & b & c;
    assert_eq!(p.e1() + p.e2() * 3.0 + p.e3() * 2.0 + p.e0(), 0.0);
    assert_eq!(-p.e1() + p.e2() * 5.0 + p.e3() * 2.0 + p.e0(), 0.0);
    assert_eq!(p.e1() * 2.0 - p.e2() - p.e3() * 4.0 + p.e0(), 0.0);
  }

  // TODO
  // * join_point_branch
  // * join_point_horizon
  // * join_plane_point

  #[test] fn measure_point_to_point() {
    assert_eq!((X & Y).squared_norm(), 2.0);
  }

  #[test] fn from_array() {
    let a: Point = [1.0, 2.0, 3.0].into();
    assert_eq!(a, Point::new(1.0, 2.0, 3.0));
  }
  #[test] fn from_tuple() {
    let a: Point = (1.0, 2.0, 3.0).into();
    assert_eq!(a, Point::new(1.0, 2.0, 3.0));
  }

  #[test] fn to_array() {
    let a: [f32; 3] = Point::new(1.0, 2.0, 3.0).into();
    assert_eq!(a, [1.0f32, 2.0, 3.0]);
  }
  #[test] fn to_tuple() {
    let a: Point = (1.0, 2.0, 3.0).into();
    assert_eq!(a, Point::new(1.0, 2.0, 3.0));
  }
}
