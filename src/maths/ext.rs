use std::simd::{f32x4,mask32x4};
use crate::maths::util::{dp, flip_signs, add_ss, hi_dp, Shuffle};

pub fn ext00(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (a1 b2 - a2 b1) e12 + (a2 b3 - a3 b2) e23 + (a3 b1 - a1 b3) e31 +
  // (a0 b1 - a1 b0) e01 + (a0 b2 - a2 b0) e02 + (a0 b3 - a3 b0) e03
  // For both outputs above, we don't zero the lowest component because
  // we've arranged a cancelation TODO wdym???
  return ((a * b.xzwy() - a.xzwy() * b).xzwy(), a.xxxx() * b - a * b.xxxx());
}

// p0 ^ p2 = p2 ^ p0
pub fn ext02(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b2 - a2 b1) e021 + (a2 b3 - a3 b2) e032 + (a3 b1 - a1 b3) e013 +
  (a * b.xzwy() - a.xzwy() * b).xzwy()
}

pub fn extpb(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e123 + (-a0 b1) e032 + (-a0 b2) e013 + (-a0 b3) e021
  let p3_out = &flip_signs(&(a.yxxx() * b), mask32x4::from_array([false,true,true,true]));
  return add_ss(p3_out, &hi_dp(a,b));
}
