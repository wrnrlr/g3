use core_simd::{Mask32, f32x4};
use crate::util::{f32x4_flip_signs, hi_dp, mul_ss};

pub fn dotptl(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn dotpl<const F:bool>(_p0:f32x4,_p1:f32x4,_p2:f32x4)->f32x4 {
  todo!()
}

// p1_out = _mm_xor_ps(_mm_set_ss(-0.f), hi_dp_ss(a, b));
pub fn dot11(a:f32x4, b:f32x4)->f32x4 {
  f32x4_flip_signs(a * b, Mask32::from_array([true,false,false,false]))
}

pub fn dot33(a:f32x4, b:f32x4)->f32x4 {
  // -a0 b0
  f32x4::from_array([-1.0, 0.0, 0.0, 0.0]) * mul_ss(a, b)
}

pub fn dot00(a:f32x4, b:f32x4)->f32x4 {
  // a1 b1 + a2 b2 + a3 b3
  hi_dp(a,b)
}

pub fn dot03(_a:f32x4,_b:f32x4)->(f32x4,f32x4) {
  todo!()
}
