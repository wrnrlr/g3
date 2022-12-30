use std::{fmt::{Display, Formatter, Result},simd::{f32x4,mask32x4,simd_swizzle},mem::transmute,ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg}};
use crate::{Dual, Plane, Line, Horizon, Branch, Motor, Translator,maths::{Shuffle, flip_signs, dp, rcp_nr1, gp33, dotptl, dot33, gp30}};

pub const E032:Point = point(1.0,0.0,0.0); // ???
pub const E012:Point = point(1.0,0.0,0.0); // ???
pub const E023:Point = point(0.0,1.0,0.0); // ???
pub const ORIGIN:Point = point(0.0,0.0,0.0);

pub const fn point(x:f32,y:f32,z:f32)->Point { Point::new(x,y,z) }

/// e₁₂₃
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Origin {} impl Into<Point> for Origin { fn into(self)->Point { Point::new(0.0,0.0,0.0) } }

/// xe₀₃₂ + ye₀₁₃ + ze₀₂₁ + e₁₂₃
#[cfg_attr(feature = "bytemuck", repr(C), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Point (pub(crate) f32x4);

impl Point {
  #[inline] pub fn w(&self)->f32 { self.0[0] }
  #[inline] pub fn e123(&self)->f32 { self.w() }
  #[inline] pub fn x(&self)->f32 { self.0[1] }
  #[inline] pub fn e032(&self)->f32 { self.x() }
  #[inline] pub fn y(&self)->f32 { self.0[2] }
  #[inline] pub fn e013(&self)->f32 { self.y() }
  #[inline] pub fn z(&self)->f32 { self.0[3] }
  #[inline] pub fn e021(&self)->f32 { self.z() }
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
}

impl Add for Point {type Output=Self;fn add(self,p:Self)->Self{Self(self.0+p.0)}}
impl Sub for Point {type Output=Self;fn sub(self,p:Self)->Self{Self(self.0-p.0)}}
impl AddAssign for Point {fn add_assign(&mut self,p:Self){self.0+=p.0}}
impl SubAssign for Point {fn sub_assign(&mut self,p:Self){self.0-=p.0}}
impl Mul<f32> for Point {type Output=Self;fn mul(self, s:f32) -> Self { Point(self.0*f32x4::splat(s)) } }
impl Mul<Point> for f32 {type Output=Point;fn mul(self, p:Point) -> Point { p*self } }
impl Div<f32> for Point {type Output=Self;fn div(self, s:f32) -> Self { Point(self.0/f32x4::splat(s)) } }
impl MulAssign<f32> for Point {fn mul_assign(&mut self, s: f32) { self.0 = self.0*f32x4::splat(s) }}
impl DivAssign<f32> for Point {fn div_assign(&mut self, s: f32) { self.0 = self.0/f32x4::splat(s) }}
impl Neg for Point {type Output = Self;fn neg(self)->Point{Point(-self.0)}}
/// let p:Plane = !point(1.0,0.0,0.0);
impl Not for Point {type Output = Plane;fn not(self)->Plane {Plane(self.0)}}
impl Mul<Point> for Point {type Output=Translator;fn mul(self,p:Point)->Translator{Translator{p2:gp33(&self.0, &p.0)}}}
impl Mul<Plane> for Point {type Output=Motor;fn mul(self,p:Plane)->Motor{let(p1,p2)=gp30(&p.0,&self.0);Motor{p1,p2}}}
impl Div<Point> for Point {type Output=Translator;fn div(self,p:Point)->Translator{self*p.inverse()}}
impl BitOr<Plane> for Point {type Output=Line;fn bitor(self, p:Plane) -> Line { p | self }}
impl BitOr<Line> for Point {type Output=Plane;fn bitor(self, l:Line) -> Plane { Plane(dotptl(&self.0,&l.p1)) }}
impl BitOr<Point> for Point {type Output=f32;fn bitor(self, a:Point) -> f32 {let out = dot33(&self.0,&a.0);out[0]}}
/// (a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123
impl BitXor<Plane> for Point {type Output=Dual;fn bitxor(self, p:Plane) -> Dual {let out = -dp(&p.0,&self.0);Dual::new(0.0,out[0])} }
impl BitAnd<Point> for Point {type Output=Line;fn bitand(self, p: Point) -> Line { !(!self ^ (!p))}}
impl BitAnd<Line> for Point {type Output=Plane;fn bitand(self, l: Line) -> Plane { !(!self ^ !l)}}
impl BitAnd<Horizon> for Point {type Output=Plane;fn bitand(self, l: Horizon) -> Plane { !(!self ^ !l)}}
impl BitAnd<Branch> for Point {type Output=Plane;fn bitand(self, b: Branch) -> Plane { !(!self ^ !b)}}
impl BitAnd<Plane> for Point {type Output=Dual;fn bitand(self, p: Plane)->Dual { !(!self ^ !p)}}
impl Display for Point { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "(x:{}, y:{}, z:{}, w:{})", self.x(), self.y(), self.z(), self.w()) } }
impl Into<[f32;3]> for Point { fn into(self) -> [f32; 3] {[self.x(), self.y(), self.z()] } }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn point_constructor() {
    assert_eq!(Point::new(1.0,2.0,3.0), point(1.0, 2.0, 3.0))
  }
  #[test] fn point_eq() {
    assert_eq!(point(1.0, 2.0, 3.0), point(1.0, 2.0, 3.0));
    assert_ne!(point(1.0, 2.0, 3.0), point(3.0, 2.0, 1.0));
  }
  #[test] fn point_getters() {
    let p = point(4.0, 2.0, 3.0);
    assert_eq!([p.x(), p.y(), p.z(), p.w()], [4.0, 2.0, 3.0, 1.0]);
    assert_eq!([p.e032(), p.e013(), p.e021(), p.e123()], [4.0,2.0,3.0,1.0]);
  }
  #[test] fn point_add() {
    assert_point(point(1.0, 2.0, 3.0)+point(1.0, 2.0, 3.0), 2.0,4.0,6.0,2.0)
  }
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
    let p = point(1.0, 2.0, 3.0);
    assert_point(-p, -1.0, -2.0, -3.0, -1.0);
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
}
