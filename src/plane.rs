use std::fmt::{Display,Formatter,Result};
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,BitAnd,BitOr,BitXor,Not,Neg,Fn};
use core_simd::{f32x4,mask32x4};
use crate::{Dual,Point,Line,IdealLine,Branch,Motor};
use crate::maths::{flip_signs, f32x4_abs, hi_dp, hi_dp_bc, rsqrt_nr1, sqrt_nr1, sw00, sw10, sw20, sw30, ext00, ext02, ext03, extpb, gp00, gp03, dot00, dot03, dotpil, dotpl};

pub const E0:Plane = Plane{p0:f32x4::from_array([1.0,0.0,0.0,0.0])};
pub const E1:Plane = Plane{p0:f32x4::from_array([0.0,1.0,0.0,0.0])};
pub const E2:Plane = Plane{p0:f32x4::from_array([0.0,0.0,1.0,0.0])};
pub const E3:Plane = Plane{p0:f32x4::from_array([0.0,0.0,0.0,1.0])};

// form: ax + by + cz + d
pub fn plane(a:f32,b:f32,c:f32,d:f32)->Plane { Plane::new(a,b,c,d) }

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Plane {
  // p0: (e0, e1, e2, e3)
  // d, a, b, c
  pub p0:f32x4
}

impl Plane {
  #[inline] pub fn d(&self)->f32 { self.p0[0] }
  #[inline] pub fn e0(&self)->f32 { self.d() }
  #[inline] pub fn x(&self)->f32 { self.p0[1] }
  #[inline] pub fn e1(&self)->f32 { self.x() }
  #[inline] pub fn y(&self)->f32 { self.p0[2] }
  #[inline] pub fn e2(&self)->f32 { self.y() }
  #[inline] pub fn z(&self)->f32 { self.p0[3] }
  #[inline] pub fn e3(&self)->f32 { self.z() }

  // The constructor performs the rearrangement so the plane can be specified
  // in the familiar form: ax + by + cz + d
  pub fn new(a:f32,b:f32,c:f32,d:f32)->Plane { Plane{p0:f32x4::from_array([d,a,b,c]) }}

  // Normalize this plane $p$ such that $p \cdot p = 1$.
  //
  // In order to compute the cosine of the angle between planes via the
  // inner product operator `|`, the planes must be normalized. Producing a
  // normalized rotor between two planes with the geometric product `*` also
  // requires that the planes are normalized.
  pub fn normalized(&self)->Plane {
    let mut inv_norm  = rsqrt_nr1(hi_dp_bc(self.p0, self.p0));
    inv_norm = inv_norm + f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    Plane{p0: inv_norm * self.p0}
  }

  // Compute the plane norm, which is often used to compute distances
  // between points and lines.
  //
  // Given a normalized point $P$ and normalized line $\ell$, the plane
  // $P\vee\ell$ containing both $\ell$ and $P$ will have a norm equivalent
  // to the distance between $P$ and $\ell$.
  pub fn norm(&self)->f32 {
    sqrt_nr1(hi_dp(self.p0, self.p0))[0]
  }

  pub fn inverse(&self)->Plane {
    let inv_norm = rsqrt_nr1(hi_dp_bc(self.p0, self.p0));
    Plane{p0: inv_norm * self.p0 * self.p0}
  }

  pub fn approx_eq(&self, other:Plane, epsilon:f32)->bool {
    f32x4_abs(self.p0 - other.p0) < f32x4::splat(epsilon)
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
    Plane{p0:sw00(self.p0, args.0.p0)}
  }
}

// Reflect the point $P$ through this plane $p$. The operation
// performed via this call operator is an optimized routine equivalent to
// the expression $p P p$.
impl FnMut<(Point,)> for Plane { extern "rust-call" fn call_mut(&mut self, args: (Point,))->Point {self.call(args)} }
impl FnOnce<(Point,)> for Plane { type Output = Point; extern "rust-call" fn call_once(self, args: (Point,))->Point { self.call(args) }}
impl Fn<(Point,)> for Plane {
  extern "rust-call" fn call(&self, args: (Point,))->Point {
    Point{p3:sw30(self.p0, args.0.p3)}
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
    let (p1, mut p2) = sw10(self.p0, l.p1);
    let p2_tmp = sw20(self.p0, l.p2);
    p2 += p2_tmp;
    Line{p1,p2}
  }
}

impl Add<Plane> for Plane {
  type Output = Plane;
  fn add(self, other: Plane) -> Plane { Plane { p0:self.p0+other.p0 } }
}

impl AddAssign for Plane {
  fn add_assign(&mut self, other: Self) { self.p0 = self.p0+other.p0 }
}

impl Sub<Plane> for Plane {
  type Output = Plane;
  fn sub(self, other: Plane) -> Plane { Plane { p0:self.p0-other.p0 } }
}

impl SubAssign for Plane {
  fn sub_assign(&mut self, other: Self) { self.p0 = self.p0-other.p0 }
}

impl Mul<f32> for Plane {
  type Output = Plane;
  fn mul(self, s: f32) -> Plane { Plane { p0:self.p0*f32x4::splat(s) } }
}

impl MulAssign<f32> for Plane {
  fn mul_assign(&mut self, s: f32) { self.p0 = self.p0*f32x4::splat(s) }
}

impl Div<f32> for Plane {
  type Output = Plane;
  fn div(self, s: f32) -> Plane { Plane { p0:self.p0/f32x4::splat(s) } }
}

impl DivAssign<f32> for Plane {
  fn div_assign(&mut self, s: f32) { self.p0 = self.p0/f32x4::splat(s) }
}

// Unary minus (leaves displacement from origin untouched, changing orientation only)
impl Neg for Plane {
  type Output = Self;
  fn neg(self)->Self::Output {
      Plane { p0:flip_signs(self.p0, mask32x4::from_array([false,true,true,true])) }
  }
}

// Geometric Product *
impl Mul<Point> for Plane {
  type Output = Motor;
  fn mul(self, a: Point) -> Motor {
    let (p1,p2) = gp03::<false>(self.p0,a.p3);
    Motor{p1,p2}
  }
}
impl Mul<Plane> for Plane {
  type Output = Motor;
  fn mul(self, other: Plane) -> Motor {
    let (p1,p2) = gp00(self.p0,other.p0);
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
  fn div(self, other: Plane) -> Motor {
    self * other.inverse()
  }
}

// Inner Product |
impl BitOr<Plane> for Plane {
  type Output = f32;
  fn bitor(self, other:Plane) -> f32 {
    dot00(self.p0,other.p0)[0]
  }
}
impl BitOr<Line> for Plane {
  type Output = Plane;
  fn bitor(self, l:Line) -> Plane {
    let p0 = dotpl(self.p0,l.p1,l.p2);
    Plane{p0}
  }
}
impl BitOr<IdealLine> for Plane {
  type Output = Plane;
  fn bitor(self, l:IdealLine) -> Plane {
    let p0 = dotpil(self.p0,l.p2);
    Plane{p0}
  }
}
impl BitOr<Point> for Plane {
  type Output = Line;
  fn bitor(self, a:Point) -> Line {
    let (p1,p2) = dot03(self.p0,a.p3);
    Line{p1,p2}
  }
}

// Meet Operator, Exterior Product, ^
impl BitXor<Plane> for Plane {
  type Output = Line;
  fn bitxor(self, other:Plane) -> Line {
    let (p1,p2) = ext00(self.p0,other.p0);
    Line{p1,p2}
  }
}
impl BitXor<Line> for Plane {
  type Output = Point;
  fn bitxor(self, l:Line) -> Point {
    let tmp1 = extpb(self.p0,l.p1);
    let tmp2 = ext02(self.p0,l.p2);
    Point{p3: tmp1+tmp2}
  }
}
impl BitXor<IdealLine> for Plane {
  type Output = Point;
  fn bitxor(self, l:IdealLine) -> Point {
    Point{p3: ext02(self.p0, l.p2)}
  }
}
impl BitXor<Branch> for Plane {
  type Output = Point;
  fn bitxor(self, b:Branch) -> Point {
    Point{p3:extpb(self.p0, b.p1)}
  }
}
impl BitXor<Point> for Plane {
  type Output = Dual;
  fn bitxor(self, p:Point) -> Dual {
    let tmp = ext03::<false>(self.p0,p.p3);
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

impl Not for Plane {
  type Output = Point;
  fn not(self)->Point { Point { p3: self.p0 }}
}
