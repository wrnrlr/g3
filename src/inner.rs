use core_simd::{f32x4};

pub fn dotptl(_a:f32x4,_b:f32x4)->f32x4 { todo!() }

pub fn dotpl<const F:bool>(_p0:f32x4,_p1:f32x4,_p2:f32x4)->f32x4 {
  todo!()
}

//  p1_out = _mm_mul_ps(_mm_set_ss(-1.f), _mm_mul_ss(a, b));
pub fn dot33(_a:f32x4,_b:f32x4)->f32 {
  // multiply homogenious coordinate
  todo!()
}

// p1_out = _mm_xor_ps(_mm_set_ss(-0.f), hi_dp_ss(a, b));
pub fn dot11(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn dot00(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn dot03(_a:f32x4,_b:f32x4)->(f32x4,f32x4) {
  todo!()
}
