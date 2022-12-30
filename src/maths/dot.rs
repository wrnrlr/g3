use std::simd::{f32x4};
use crate::maths::util::{Shuffle, add_ss, f32x4_xor, hi_dp, hi_dp_ss, mul_ss, sub_ss, zero_first};

// a1 b1 + a2 b2 + a3 b3
pub fn dot00(a:&f32x4, b:&f32x4)->f32x4 {
  hi_dp(a,b)
}

pub fn dot03(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (a2b1 - a1b2)e03 + (a3b2 - a2b3)e01 + (a1b3 - a3b1)e02 + a1b0e23 + a2b0e31 + a3b0e12
  let mut p1_out = a * b.xxxx();
  p1_out = zero_first(p1_out);
  (p1_out, (a.xzwy()*b - a*b.xzwy()).xzwy())
}

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

pub fn dotpl(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  let mut p0 = a.xzwy() * b;
  p0 -= a * b.xzwy();
  sub_ss(&(p0.xzwy()), hi_dp_ss(a, c))
}

pub fn dotlp(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  let mut p0 = a * b.xzwy();
  p0 -= a.xzwy() * b;
  add_ss(&(p0.xzwy()), &hi_dp_ss(a, c))
}

pub fn dotpil(a:&f32x4, c:&f32x4)->f32x4 {
  f32x4_xor(&hi_dp(a, c), &[-0.0, 0.0, 0.0, 0.0].into())
}
