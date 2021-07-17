// pub trait F32 { fn f32(self)->f32; }
// impl F32 for f32 { fn f32(self)->f32 { self } }
// impl F32 for i64 { fn f32(self)->f32 { self as f32 } }

// #[macro_export] macro_rules! dual {
//   ($p:expr, $q:expr) => {
//     { Dual::new(F32::f32($p), F32::f32($q)) };
//   };
// }

// A dual number is a multivector of the form $p + q\mathbf{e}_{0123}$.
// #[macro_export] macro_rules! point {
//   ($p:expr, $q:expr) => {
//     { Dual::new(F32::f32(&$p),F32::f32(&$q)) };
//   };
// }

// Component-wise constructor (homogeneous coordinate is automatically initialized to 1)
// xe023 + ye013 + ze021 + w
// #[macro_export] macro_rules! point {
//   ($x:expr, $y:expr, $z:expr) => {
//     { Point::new(F32.f32(&$x),F32.f32(&$y),F32.f32(&$z)) }
//   };
// }

// The constructor performs the rearrangement so the plane can be specified
// in the familiar form: ax + by + cz + d
// #[macro_export] macro_rules! plane {
//   ($a:expr,$b:expr,$c:expr,$d:expr) => {
//     { Plane::new(F32.f32(&$a),F32.f32(&$b),F32.f32(&$c),F32.f32(&$d)) }
//   };
// }

// A line is specifed by 6 coordinates which correspond to the line's
// [Plücker coordinates](https://en.wikipedia.org/wiki/Pl%C3%BCcker_coordinates).
// The coordinates specified in this way correspond to the following multivector:
//
// $$a\mathbf{e}_{01} + b\mathbf{e}_{02} + c\mathbf{e}_{03} +\
// d\mathbf{e}_{23} + e\mathbf{e}_{31} + f\mathbf{e}_{12}$$
// #[macro_export] macro_rules! line {
//   ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => {
//     { Line::new(F32.f32(&$a),F32.f32(&$b),F32.f32(&$c),F32.f32(&$d),F32.f32(&$e),F32.f32(&$f)) }
//   };
// }
