use std::{fmt::{Display, Formatter, Result},simd::{f32x4},ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg,Fn}};
use crate::{Dual, Point, Line, Horizon, Branch, Motor,maths::{flip_signs, f32x4_abs, hi_dp, hi_dp_bc, rsqrt_nr1, sqrt_nr1, sw00, sw10, sw20, sw30, ext00, ext02, ext03, extpb, gp00, gp03, dot00, dot03, dotpil, dotpl}};
#[cfg(feature = "bevy")] use bevy::prelude::Component;

pub const E0:Plane = Plane([1.0,0.0,0.0,0.0].into());
pub const E1:Plane = Plane([0.0,1.0,0.0,0.0].into());
pub const E2:Plane = Plane([0.0,0.0,1.0,0.0].into());
pub const E3:Plane = Plane([0.0,0.0,0.0,1.0].into());

// form: ax + by + cz + d
pub fn plane(a:f32,b:f32,c:f32,d:f32)->Plane { Plane::new(a,b,c,d) }

// p0: (e0, e1, e2, e3)
#[cfg_attr(feature="bevy",derive(Component))]
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Plane (pub f32x4);

impl Plane {
  #[inline] pub fn d(&self)->f32 { self.0[0] }
  #[inline] pub fn e0(&self)->f32 { self.d() }
  #[inline] pub fn x(&self)->f32 { self.0[1] }
  #[inline] pub fn e1(&self)->f32 { self.x() }
  #[inline] pub fn y(&self)->f32 { self.0[2] }
  #[inline] pub fn e2(&self)->f32 { self.y() }
  #[inline] pub fn z(&self)->f32 { self.0[3] }
  #[inline] pub fn e3(&self)->f32 { self.z() }

  // The constructor performs the rearrangement so the plane can be specified
  // in the familiar form: ax + by + cz + d
  pub fn new(a:f32,b:f32,c:f32,d:f32)->Plane { Plane([d,a,b,c].into())}

  // Normalize this plane $p$ such that $p \cdot p = 1$.
  //
  // In order to compute the cosine of the angle between planes via the
  // inner product operator `|`, the planes must be normalized. Producing a
  // normalized rotor between two planes with the geometric product `*` also
  // requires that the planes are normalized.
  pub fn normalized(&self)->Plane {
    let mut inv_norm  = rsqrt_nr1(&hi_dp_bc(&self.0, &self.0));
    inv_norm = inv_norm + f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    Plane(inv_norm * &self.0)
  }

  // Compute the plane norm, which is often used to compute distances
  // between points and lines.
  //
  // Given a normalized point $P$ and normalized line $\ell$, the plane
  // $P\vee\ell$ containing both $\ell$ and $P$ will have a norm equivalent
  // to the distance between $P$ and $\ell$.
  pub fn norm(&self)->f32 {
    sqrt_nr1(&hi_dp(&self.0, &self.0))[0]
  }

  pub fn inverse(&self)->Plane {
    let inv_norm = &rsqrt_nr1(&hi_dp_bc(&self.0, &self.0));
    Plane(inv_norm * inv_norm * &self.0)
  }

  pub fn approx_eq(&self, other:Plane, epsilon:f32)->bool {
    let diff = f32x4_abs(&self.0 - other.0);
    diff < f32x4::splat(epsilon)
  }

  // Project a plane onto a point. Given a plane $p$ and point $P$, produces the
  // plane through $P$ that is parallel to $p$.
  //
  // Intuitively, the point is represented dually in terms of a _pencil of
  // planes_ that converge on the point itself. When we compute $p | P$, this
  // selects the line perpendicular to $p$ through $P$. Subsequently, taking the
  // inner product with $P$ again selects the plane from the plane pencil of $P$
  // _least like_ that line.
  pub fn project_point(self, a:Point)->Plane { (self | a) | a }

  // Project a plane onto a line. Given a plane $p$ and line $\ell$, produces the
  // plane through $\ell$ that is parallel to $p$ if $p \parallel \ell$.
  //
  // If $p \nparallel \ell$, the result will be the plane $p'$ containing $\ell$
  // that maximizes $p \cdot p'$ (that is, $p'$ is as parallel to $p$ as
  // possible).
  pub fn project_line(self, l:Line)->Plane {  (self | l) | l }
}

impl Display for Plane {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}e1 + {}e2 + {}e2 + {}e0)", self.x(), self.y(), self.z(), self.d())
  }
}

// Reflect another plane $p_2$ through this plane $p_1$. The operation
// performed via this call operator is an optimized routine equivalent to
// the expression $p_1 p_2 p_1$.
impl FnMut<(Plane,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Plane,))->Plane { self.call(args) }}
impl FnOnce<(Plane,)> for Plane { type Output = Plane; extern "rust-call" fn call_once(self, args: (Plane,))->Plane {self.call(args)} }
impl Fn<(Plane,)> for Plane {
  extern "rust-call" fn call(&self, args: (Plane,))->Plane {
    Plane(sw00(&self.0, &args.0.0))
  }
}

// Reflect the point $P$ through this plane $p$. The operation
// performed via this call operator is an optimized routine equivalent to
// the expression $p P p$.
impl FnMut<(Point,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point {self.call(args)} }
impl FnOnce<(Point,)> for Plane { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
impl Fn<(Point,)> for Plane {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    Point(sw30(&self.0, &args.0.0))
  }
}

// Reflect line $\ell$ through this plane $p$. The operation
// performed via this call operator is an optimized routine equivalent to
// the expression $p \ell p$.
impl FnMut<(Line,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Line,))->Line {self.call(args)} }
impl FnOnce<(Line,)> for Plane { type Output = Line; extern "rust-call" fn call_once(self, args: (Line,))->Line { self.call(args) }}
impl Fn<(Line,)> for Plane {
  extern "rust-call" fn call(&self, args: (Line,)) -> Line {
    let l = args.0;
    let (p1, mut p2) = sw10(&self.0, &l.p1);
    let p2_tmp = sw20(&self.0, &l.p2);
    p2 += p2_tmp;
    Line{p1,p2}
  }
}

impl Add<Plane> for Plane {
  type Output = Plane;
  fn add(self, p: Plane) -> Plane { Plane(self.0+p.0) }
}

impl AddAssign for Plane {
  fn add_assign(&mut self, p: Self) { self.0 += p.0 }
}

impl Sub<Plane> for Plane {
  type Output = Plane;
  fn sub(self, p: Plane) -> Plane { Plane(self.0-p.0) }
}

impl SubAssign for Plane {
  fn sub_assign(&mut self, p: Self) { self.0 -= p.0 }
}

impl Mul<f32> for Plane {
  type Output = Plane;
  fn mul(self, s: f32) -> Plane { Plane(self.0*f32x4::splat(s)) }
}

impl MulAssign<f32> for Plane {
  fn mul_assign(&mut self, s: f32) { self.0 *= f32x4::splat(s) }
}

impl Div<f32> for Plane {
  type Output = Plane;
  fn div(self, s: f32) -> Plane { Plane(self.0/f32x4::splat(s)) }
}

impl DivAssign<f32> for Plane {
  fn div_assign(&mut self, s: f32) { self.0 /= f32x4::splat(s) }
}

// Unary minus (leaves displacement from origin untouched, changing orientation only)
impl Neg for Plane {
  type Output = Self;
  fn neg(self)->Self::Output {
      Plane(flip_signs(&self.0, [false,true,true,true].into()) )
  }
}

// Geometric Product *
impl Mul<Point> for Plane {
  type Output = Motor;
  fn mul(self, a: Point) -> Motor {
    let (p1,p2) = gp03(&self.0,&a.0);
    Motor{p1,p2}
  }
}
impl Mul<Plane> for Plane {
  type Output = Motor;
  fn mul(self, p: Plane) -> Motor {
    let (p1,p2) = gp00(&self.0,&p.0);
    Motor{p1,p2}
  }
}

// TODO this is not in klein
// impl Div<Point> for Plane {
//   type Output = Motor;
//   fn div(self, a: Point) -> Motor {
//     self * a.inverse()
//   }
// }

impl Div<Plane> for Plane {
  type Output = Motor;
  fn div(self, p: Plane) -> Motor {
    self * p.inverse()
  }
}

// Inner Product |
impl BitOr<Plane> for Plane {
  type Output = f32;
  fn bitor(self, p:Plane) -> f32 {
    dot00(&self.0,&p.0)[0]
  }
}
impl BitOr<Line> for Plane {
  type Output = Plane;
  fn bitor(self, l:Line) -> Plane {
    let p0 = dotpl(&self.0,&l.p1,&l.p2);
    Plane(p0)
  }
}
impl BitOr<Horizon> for Plane {
  type Output = Plane;
  fn bitor(self, l: Horizon) -> Plane {
    let p0 = dotpil(&self.0,&l.p2);
    Plane(p0)
  }
}
impl BitOr<Point> for Plane {
  type Output = Line;
  fn bitor(self, a:Point) -> Line {
    let (p1,p2) = dot03(&self.0,&a.0);
    Line{p1,p2}
  }
}

// Meet Operator, Exterior Product, ^
impl BitXor<Plane> for Plane {
  type Output = Line;
  fn bitxor(self, p:Plane) -> Line {
    let (p1,p2) = ext00(&self.0,&p.0);
    Line{p1,p2}
  }
}
impl BitXor<Line> for Plane {
  type Output = Point;
  fn bitxor(self, l:Line) -> Point {
    let tmp1 = extpb(&self.0,&l.p1);
    let tmp2 = ext02(&self.0,&l.p2);
    Point(tmp1+tmp2)
  }
}
impl BitXor<Horizon> for Plane {
  type Output = Point;
  fn bitxor(self, l: Horizon) -> Point {
    Point(ext02(&self.0, &l.p2))
  }
}
impl BitXor<Branch> for Plane {
  type Output = Point;
  fn bitxor(self, b:Branch) -> Point {
    Point(extpb(&self.0, &b.0))
  }
}
impl BitXor<Point> for Plane {
  type Output = Dual;
  fn bitxor(self, p:Point) -> Dual {
    let tmp = ext03::<false>(&self.0,&p.0);
    Dual::new(0.0, tmp[0])
  }
}

// Join Operator &
impl BitAnd<Point> for Plane {
  type Output = Dual;
  fn bitand(self, p: Point) -> Dual {
    !(!self ^ !p)
  }
}

impl Not for Plane {type Output = Point;fn not(self)->Point{Point(self.0)}}
