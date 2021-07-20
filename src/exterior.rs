use core_simd::{f32x4};

use crate::util::{dp};

pub fn ext00(_a:f32x4,_b:f32x4)->(f32x4,f32x4) {
  todo!()
}

pub fn ext02(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

// p0 ^ p3 = -p3 ^ p0
pub fn ext03<const F:bool>(a:f32x4, b:f32x4)->f32x4 {
  // (a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123
  let mut p2 = dp(a,b);
  if F { p2 = -p2 }
  p2
}

pub fn extpb(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}
