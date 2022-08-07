use std::simd::{f32x4};
use crate::maths::{f32x4_xor, shuffle_wwxx, shuffle_wxxx, shuffle_wywx, shuffle_wyzx, shuffle_xxxx, shuffle_xzxx, shuffle_ywyx, shuffle_yxwx, shuffle_yzwx, shuffle_yzxx, shuffle_zwxx, shuffle_zwyx, shuffle_zxzx, shuffle_zzwx};

// false, false, for rotor
pub fn mat4x4_12(b:&f32x4)->(f32x4,f32x4,f32x4,f32x4) {
  let buf = *(b * b).as_array();
  let b0_2 = buf[0];
  let b1_2 = buf[1];
  let b2_2 = buf[2];
  let b3_2 = buf[3];

  let mut c0 = b * shuffle_xzxx(b);
  let mut tmp = shuffle_ywyx(b) * shuffle_yxwx(b);
  tmp = f32x4_xor(&[0.0, -0.0, 0.0, 0.0].into(), &tmp); // TODO why reference?
  let one_twos:f32x4 = [1f32, 2.0, 2.0, 0.0].into();
  c0 =  one_twos * (c0 + tmp);
  c0 = c0 - f32x4::splat(b3_2 + b2_2);

  let c1 = b * shuffle_wywx(b);
  let mut tmp = shuffle_zwxx(b) * shuffle_ywyx(b);
  tmp = f32x4_xor(&[0.0, 0.0, -0.0, 0.0].into(), &tmp); // TODO why reference?
  let tmp1 = f32x4::from_array([2.0, -1.0, 2.0, 0.0]);
  let mut c1:f32x4 = tmp1 * (c1 + tmp);
  let duno:f32x4 = [0.0, b0_2+b2_2, 0.0, 0.0].into();
  c1 = c1 + duno;

  let mut c2:f32x4 = f32x4_xor(&[-0.0, 0.0, -0.0, 0.0].into(), &(b * shuffle_zxzx(b)));
  c2 = c2 + (shuffle_yzxx(b) * shuffle_wwxx(b));
  c2 *= <[f32;4]as Into<f32x4>>::into([2.0, 2.0, 1.0, 0.0]);
  c2 += <[f32;4]as Into<f32x4>>::into([0.0, 0.0, b3_2 - b1_2, 0.0]);

  // TODO why is c3 here
  // c3 = _mm_add_ps(c3, _mm_set_ps(b0_2 + b1_2 + b2_2 + b3_2, 0.f, 0.f, 0.f));
  let c3 = [0.0, 0.0, 0.0, b0_2 + b1_2 + b2_2 + b3_2].into();
  (c0,c1,c2,c3)
}
// true, false, for motor
pub fn mat4x4_12_m(b:&f32x4, c:&f32x4)->(f32x4,f32x4,f32x4,f32x4) {
  let buf = *(b * b).as_array();
  let b0_2 = buf[0];
  let b1_2 = buf[1];
  let b2_2 = buf[2];
  let b3_2 = buf[3];

  let mut c0:f32x4 = b * shuffle_xzxx(b);
  let mut tmp = shuffle_ywyx(b) * shuffle_yxwx(b);
  tmp = f32x4_xor(&[0.0, -0.0, 0.0, 0.0].into(), &tmp);
  let one_twos:f32x4 = [1f32, 2.0, 2.0, 0.0].into();
  c0 = one_twos * (c0 + tmp);
  let tmp2:f32x4 = [b3_2 + b2_2, 0.0, 0.0, 0.0].into();
  c0 = c0 - tmp2;

  let c1 = b * shuffle_wywx(b);
  let mut tmp = shuffle_zwxx(b) * shuffle_ywyx(b);
  tmp = f32x4_xor(&[0.0, 0.0, -0.0, 0.0].into(), &tmp);
  let mut c1 = &<[f32; 4] as Into<f32x4>>::into([2.0, -1.0, 2.0, 0.0]) * (c1 + tmp);
  c1 = c1 + &[0.0, b0_2+b2_2, 0.0, 0.0].into();

  let mut c2 = f32x4_xor(&[-0.0, 0.0, -0.0, 0.0].into(), &(b * shuffle_zxzx(b)));
  c2 = c2 + (shuffle_yzxx(b) * shuffle_wwxx(b));
  c2 *= <[f32;4] as Into<f32x4>>::into([2.0, 2.0, 1.0, 0.0]);
  c2 += <[f32; 4] as Into<f32x4>>::into([0.0, 0.0, b3_2 - b1_2, 0.0]);

  // c2 = _mm_xor_ps(_mm_set_ps(0.f, -0.f, 0.f, -0.f), _mm_mul_ps(b, KLN_SWIZZLE(b, 0, 2, 0, 2)));
  // c2 = _mm_add_ps(c2, _mm_mul_ps(KLN_SWIZZLE(b, 0, 0, 2, 1), KLN_SWIZZLE(b, 0, 0, 3, 3)));
  // c2 = _mm_mul_ps(c2, _mm_set_ps(0.f, 1.f, 2.f, 2.f));
  // c2 = _mm_add_ps(c2, _mm_set_ps(0.f, b3_2 - b1_2, 0.f, 0.f));

  let mut c3 = b * shuffle_ywyx(c);
  c3 = c3 + shuffle_wxxx(b) * shuffle_zzwx(c);
  c3 = c3 + shuffle_yzwx(b) * shuffle_xxxx(c);
  tmp = shuffle_zwyx(b) * shuffle_wyzx(c);
  c3 = <[f32; 4] as Into<f32x4>>::into([2.0,2.0,2.0,0.0]) * (tmp - c3);

  // __m128& c3 = out[3];
  // c3 = _mm_mul_ps(b, KLN_SWIZZLE(*c, 0, 1, 3, 1));
  // c3 = _mm_add_ps(c3, _mm_mul_ps(KLN_SWIZZLE(b, 0, 0, 0, 3), KLN_SWIZZLE(*c, 0, 3, 2, 2)));
  // c3 = _mm_add_ps(c3, _mm_mul_ps(KLN_SWIZZLE(b, 0, 3, 2, 1), KLN_SWIZZLE(*c, 0, 0, 0, 0)));
  // tmp = _mm_mul_ps(KLN_SWIZZLE(b, 0, 1, 3, 2), KLN_SWIZZLE(*c, 0, 2, 1, 3));
  // c3  = _mm_mul_ps(_mm_set_ps(0.f, 2.f, 2.f, 2.f), _mm_sub_ps(tmp, c3));

  // c3 = _mm_add_ps(c3, _mm_set_ps(b0_2 + b1_2 + b2_2 + b3_2, 0.f, 0.f, 0.f));
  c3 = c3 + <[f32; 4] as Into<f32x4>>::into([0.0, 0.0, 0.0, b0_2 + b1_2 + b2_2 + b3_2]);

  (c0,c1,c2,c3)
}
