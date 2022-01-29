use core_simd::{f32x4};
use crate::util::{add_ss, f32x4_and, f32x4_xor, hi_dp, hi_dp_ss, mul_ss, shuffle_xxxx, shuffle_xzwy};

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
  p1_out = f32x4_and(p1_out, f32x4::from([0.0, -1.0, -1.0, -1.0]));
  let p2_out = shuffle_xzwy(shuffle_xzwy(a)*b - a*shuffle_xzwy(b));
  (p1_out, p2_out)
}

// p1_out = _mm_xor_ps(_mm_set_ss(-0.f), hi_dp_ss(a, b));
pub fn dot11(_a:f32x4,_b:f32x4)->f32x4 {
  //f32x4_flip_signs(a * b, Mask32::from_array([true,false,false,false]))
  todo!()
}

pub fn dot33(a:f32x4, b:f32x4)->f32x4 {
  // -a0 b0
  f32x4::from_array([-1.0, 0.0, 0.0, 0.0]) * mul_ss(a, b)
}

pub fn dotptl(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn dotpl<const F:bool>(_p0:f32x4,_p1:f32x4,_p2:f32x4)->f32x4 {
  todo!()
}

pub fn dotlp<const F:bool>(a:f32x4, b:f32x4, c:f32x4)->f32x4 {
  let mut p0 = a * shuffle_xzwy(b);
  p0 -= a * shuffle_xzwy(b);
  add_ss(shuffle_xzwy(p0), hi_dp_ss(a, c))
}

pub fn dotpil(a:f32x4, c:f32x4)->f32x4 {
  f32x4_xor(dotilp(a, c), f32x4::from([-0.0, 0.0, 0.0, 0.0]))
}

pub fn dotilp(a:f32x4, c:f32x4)->f32x4 {
  hi_dp(a, c)
}
