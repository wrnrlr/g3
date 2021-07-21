use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg};
use core_simd::{f32x4,Mask32};
use crate::{Dual,Plane,Line,Motor,Translator};
use crate::util::{f32x4_flip_signs,rcp_nr1,shuffle_wwww};
use crate::geometric::{gp03,gp33};
use crate::inner::{dotptl,dot33};
use crate::exterior::{ext03};

pub fn point(x:f32,y:f32,z:f32)->Point { Point::new(x,y,z) }

// A point is represented as the multivector
// $x\mathbf{e}_{032} + y\mathbf{e}_{013} + z\mathbf{e}_{021} +
// \mathbf{e}_{123}$. The point has a trivector representation because it is
// the fixed point of 3 planar reflections (each of which is a grade-1
// multivector). In practice, the coordinate mapping can be thought of as an
// implementation detail.
// p3: (w,    x,    y,    z)
// p3: (e123, e032, e013, e021)
#[derive(Default,Debug,Clone,PartialEq)]
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

    // pub fn normalized(&self)->Point {
    //     let tmp = refined_reciprocal(shuffle!(self.p3,[0,0,0,0]));
    //     Point{ p3: self.p3 * tmp }
    // }

    pub fn inverse(&self)->Point {
        let inv_norm = rcp_nr1(shuffle_wwww(self.p3));
        let p3 = inv_norm * self.p3;
        Point{p3:inv_norm * p3}
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
    fn mul(self, s: f32) -> Point { Point { p3:self.p3*s } }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, s: f32) { self.p3 = self.p3*s }
}

impl Div<f32> for Point {
    type Output = Point;
    fn div(self, s: f32) -> Point { Point { p3:self.p3/s } }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, s: f32) { self.p3 = self.p3/s }
}

// Reversion
impl Neg for Point {
    type Output = Self;
    fn neg(self)->Self::Output {
        Point { p3:f32x4_flip_signs(self.p3, Mask32::from_array([false,true,true,true])) }
    }
}

// TODO ~ flip all sign, the ~ is not available in rust ...

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
// Do not exist in klein
// impl BitXor<Line> for Point {
//   type Output = Point;
//   fn bitxor(self, l:Line) -> Point { todo!() }
// }
// impl BitXor<Point> for Point {
//   type Output = Dual;
//   fn bitxor(self, a:Point) -> Dual { todo!() }
// }
  
// Join Operator &
impl BitAnd<Point> for Point {
  type Output = Line;
  fn bitand(self, other: Point) -> Line { !(!self ^ !other) }
}
impl BitAnd<Line> for Point {
  type Output = Plane;
  fn bitand(self, l: Line) -> Plane { !((!self) ^ (!l)) }
}
impl BitAnd<Plane> for Point {
  type Output = Dual;
  fn bitand(self, p: Plane)->Dual { !(!self ^ !p) }
}

// impl BitAndAssign<Branch> for Plane {
//   fn bitand_assign(&mut self, a: Point) { todo!() }
// }
// impl BitAndAssign<IdealLine> for Plane {
//   fn bitand_assign(&mut self, a: Point) { todo!() }
// }
