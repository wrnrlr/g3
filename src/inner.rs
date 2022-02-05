use core_simd::{f32x4};
use crate::util::{add_ss, f32x4_xor, hi_dp, hi_dp_ss, mul_ss, shuffle_xxxx, shuffle_xzwy, sse_hi_dp_ss, sub_ss, zero_first};

pub fn dot00(a:f32x4, b:f32x4)->f32x4 {
  // a1 b1 + a2 b2 + a3 b3
  hi_dp(a,b)
}

pub fn dot03(a:f32x4, b:f32x4)->(f32x4,f32x4) {
  // (a2 b1 - a1 b2) e03 +
  // (a3 b2 - a2 b3) e01 +
  // (a1 b3 - a3 b1) e02 +
  // a1 b0 e23 +
  // a2 b0 e31 +
  // a3 b0 e12
  let mut p1_out = a * shuffle_xxxx(b);
  p1_out = zero_first(p1_out);
  let p2_out = shuffle_xzwy(shuffle_xzwy(a)*b - a*shuffle_xzwy(b));
  (p1_out, p2_out)
}

pub fn dot11(a:f32x4, b:f32x4)->f32x4 {
  f32x4_xor(f32x4::from([-0.0, 0.0, 0.0, 0.0]), hi_dp_ss(a, b))
}

pub fn dot33(a:f32x4, b:f32x4)->f32x4 {
  // -a0 b0
  f32x4::from_array([-1.0, 0.0, 0.0, 0.0]) * mul_ss(a, b)
}

pub fn dotptl(a:f32x4, b:f32x4)->f32x4 {
  let dp = hi_dp_ss(a, b);
  let p0 = shuffle_xxxx(a) * b;
  let p0 = f32x4_xor(p0, f32x4::from([0.0, -0.0, -0.0, -0.0]));
  add_ss(p0, dp)
}

pub fn dotpl(a:f32x4, b:f32x4, c:f32x4)->f32x4 {
  let mut p0 = shuffle_xzwy(a) * b;
  p0 -= a * shuffle_xzwy(b);
  sub_ss(shuffle_xzwy(p0), hi_dp_ss(a, c))
}

pub fn dotlp(a:f32x4, b:f32x4, c:f32x4)->f32x4 {
  let mut p0 = a * shuffle_xzwy(b);
  p0 -= shuffle_xzwy(a) * b;
  add_ss(shuffle_xzwy(p0), hi_dp_ss(a, c))
}

pub fn dotpil(a:f32x4, c:f32x4)->f32x4 {
  f32x4_xor(dotilp(a, c), f32x4::from([-0.0, 0.0, 0.0, 0.0]))
}

pub fn dotilp(a:f32x4, c:f32x4)->f32x4 {
  hi_dp(a, c)
}
