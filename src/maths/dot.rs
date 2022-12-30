use std::simd::{f32x4};
use crate::maths::util::{Shuffle, add_ss, f32x4_xor, hi_dp, hi_dp_ss, mul_ss, sub_ss, zero_first};

pub fn dot11(a:&f32x4, b:&f32x4)->f32x4 {
  f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &hi_dp_ss(a, b))
}

pub fn dot33(a:&f32x4, b:&f32x4)->f32x4 {
  // -a0 b0
  f32x4::from_array([-1.0, 0.0, 0.0, 0.0]) * mul_ss(a, b)
}

pub fn dotptl(a:&f32x4, b:&f32x4)->f32x4 {
  let dp = &hi_dp_ss(a, b);
  let p0 = &a.xxxx() * b;
  let p0 = &f32x4_xor(&p0, &[0.0, -0.0, -0.0, -0.0].into());
  add_ss(p0, dp)
}

pub fn dotlp(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  let mut p0 = a * b.xzwy();
  p0 -= a.xzwy() * b;
  add_ss(&(p0.xzwy()), &hi_dp_ss(a, c))
}

pub fn dotpil(a:&f32x4, c:&f32x4)->f32x4 {
  f32x4_xor(&hi_dp(a, c), &[-0.0, 0.0, 0.0, 0.0].into())
}
