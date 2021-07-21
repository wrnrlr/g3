use core_simd::{f32x4};

use crate::util::{dp, shuffle_wwww, shuffle_wyzx};

pub fn ext00(a:f32x4, b:f32x4)->(f32x4,f32x4) {
  // (a1 b2 - a2 b1) e12 +
  // (a2 b3 - a3 b2) e23 +
  // (a3 b1 - a1 b3) e31 +
  // (a0 b1 - a1 b0) e01 +
  // (a0 b2 - a2 b0) e02 +
  // (a0 b3 - a3 b0) e03
  let mut p1_out = a * shuffle_wyzx(b);
  p1_out = shuffle_wyzx(p1_out - shuffle_wyzx(a) * b);
  let mut p2_out = shuffle_wwww(a) * b;
  p2_out = p2_out - a * shuffle_wwww(b);
  // For both outputs above, we don't zero the lowest component because
  // we've arranged a cancelation TODO wdym???
  return (p1_out,p2_out)
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
