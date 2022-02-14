use std::fmt::{Display,Formatter,Result};
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg};
use core_simd::{f32x4,mask32x4};
use crate::{Dual, Plane, Line, Horizon, Branch, Motor, Translator};
use crate::maths::{flip_signs, rcp_nr1, shuffle_xxxx, gp03, gp33, dotptl, dot33, ext03};

pub struct Origin {}

impl Origin {
  pub fn to_point()->Point { Point::new(0.0,0.0,0.0) }
}

pub const E032:Point = Point{p3:f32x4::from_array([1.0,1.0,0.0,0.0])};
pub const E012:Point = Point{p3:f32x4::from_array([1.0,0.1,0.0,0.0])};
pub const E023:Point = Point{p3:f32x4::from_array([1.0,0.0,0.1,0.0])};

pub fn point(x:f32,y:f32,z:f32)->Point { Point::new(x,y,z) }

// A point is represented as the multivector
// $x\mathbf{e}_{032} + y\mathbf{e}_{013} + z\mathbf{e}_{021} +
// \mathbf{e}_{123}$. The point has a trivector representation because it is
// the fixed point of 3 planar reflections (each of which is a grade-1
// multivector). In practice, the coordinate mapping can be thought of as an
// implementation detail.
// p3: (w,    x,    y,    z)
// p3: (e123, e032, e013, e021)
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Point {
    pub p3:f32x4
}

impl Point {
    #[inline] pub fn w(&self)->f32 { self.p3[0] }
    #[inline] pub fn e123(&self)->f32 { self.w() }
    #[inline] pub fn x(&self)->f32 { self.p3[1] }
    #[inline] pub fn e032(&self)->f32 { self.x() }
    #[inline] pub fn y(&self)->f32 { self.p3[2] }
    #[inline] pub fn e013(&self)->f32 { self.y() }
    #[inline] pub fn z(&self)->f32 { self.p3[3] }
    #[inline] pub fn e021(&self)->f32 { self.z() }

    // Component-wise constructor where homogeneous coordinate is automatically initialized to 1.
    pub fn new(x:f32,y:f32,z:f32)->Point { Point{p3:f32x4::from_array([1.0,x,y,z])} }

    pub fn normalized(&self)->Point {
        let tmp = rcp_nr1(shuffle_xxxx(self.p3));
        Point{ p3: self.p3 * tmp }
    }

    pub fn inverse(&self)->Point {
        let inv_norm = rcp_nr1(shuffle_xxxx(self.p3));
        Point{p3:inv_norm * inv_norm * self.p3}
    }

    pub fn reverse(&self)->Point {
      Point{p3: flip_signs(self.p3, mask32x4::from_array([false,true,true,true]))}
    }

    // Project a point onto a line
    pub fn project_line(self, l:Line)->Point { (self | l) ^ l }

    // Project a point onto a plane
    pub fn project_plane(self, p:Plane)->Point { (self | p) ^ p }
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "(x:{}, y:{}, z:{}, w:{})", self.x(), self.y(), self.z(), self.w())
  }
}

impl Into<[f32;3]> for Point {
  fn into(self) -> [f32; 3] {
    [self.x(), self.y(), self.z()]
  }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point { Point { p3:self.p3+other.p3 } }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) { self.p3 = self.p3+other.p3 }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point { Point { p3:self.p3-other.p3 } }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) { self.p3 = self.p3-other.p3 }
}

impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, s: f32) -> Point { Point { p3:self.p3*f32x4::splat(s) } }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, s: f32) { self.p3 = self.p3*f32x4::splat(s) }
}

impl Div<f32> for Point {
    type Output = Point;
    fn div(self, s: f32) -> Point { Point { p3:self.p3/f32x4::splat(s) } }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, s: f32) { self.p3 = self.p3/f32x4::splat(s) }
}

// Reversion
impl Neg for Point {
  type Output = Point;
  fn neg(self)->Point { Point{ p3: -self.p3 } }
}

impl Not for Point {
  type Output = Plane;
  fn not(self)->Plane { Plane { p0: self.p3 }}
}

// Geometric Product *
impl Mul<Point> for Point {
  type Output = Translator;
  fn mul(self, other: Point) -> Translator {
    let p2 = gp33(self.p3, other.p3);
    Translator{p2}
  }
}
impl Mul<Plane> for Point {
  type Output = Motor;
  fn mul(self, p: Plane) -> Motor {
      let (p1,p2) = gp03::<false>(p.p0, self.p3);
      Motor{p1,p2}
  }
}
// Inverse Geometric Product
impl Div<Point> for Point {
  type Output = Translator;
  fn div(self, other: Point) -> Translator {
    self * other.inverse()
  }
}

// TODO Does not exist, strange?
// impl Div<Plane> for Point {
//   type Output = Motor;
//   fn div(self, p: Plane) -> Motor {
//     other.invert();
//     self * other
//   }
// }

// Inner Product |
impl BitOr<Plane> for Point {
  type Output = Line;
  fn bitor(self, p:Plane) -> Line { p | self }
}
impl BitOr<Line> for Point {
  type Output = Plane;
  fn bitor(self, l:Line) -> Plane {
    let p0 = dotptl(self.p3,l.p1);
    Plane{p0:p0}
  }
}
impl BitOr<Point> for Point {
  type Output = f32;
  fn bitor(self, a:Point) -> f32 {
    let out = dot33(self.p3,a.p3);
    out[0]
  }
}

// Meet Operator ^ (aka Wedge/Exteriour Product)
impl BitXor<Plane> for Point {
  type Output = Dual;
  fn bitxor(self, other:Plane) -> Dual {
    let out = ext03::<true>(other.p0,self.p3);
    Dual::new(0.0,out[0])
  }
}
// Do not exist because
// impl BitXor<Line> for Point {
//   type Output = Point;
//   fn bitxor(self, l:Line) -> Point {} }
// }
// impl BitXor<Point> for Point {
//   type Output = Dual;
//   fn bitxor(self, a:Point) -> Dual {}
// }

// Join Operation, Regressive Product, &
impl BitAnd<Point> for Point {
  type Output = Line;
  fn bitand(self, other: Point) -> Line { !(!self ^ (!other)) }
}
impl BitAnd<Line> for Point {
  type Output = Plane;
  fn bitand(self, l: Line) -> Plane { !(!self ^ !l) }
}
impl BitAnd<Horizon> for Point {
  type Output = Plane;
  fn bitand(self, l: Horizon) -> Plane { !(!self ^ !l) }
}
impl BitAnd<Branch> for Point {
  type Output = Plane;
  fn bitand(self, b: Branch) -> Plane { !(!self ^ !b) }
}
impl BitAnd<Plane> for Point {
  type Output = Dual;
  fn bitand(self, p: Plane)->Dual { !(!self ^ !p) }
}
