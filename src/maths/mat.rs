use core_simd::{f32x4};
use crate::maths::{f32x4_xor, shuffle_wwxx, shuffle_wxxx, shuffle_wywx, shuffle_xxxx, shuffle_xzxx, shuffle_ywyx, shuffle_yxwx, shuffle_yzxx, shuffle_zwxx, shuffle_zxzx};

// false, false, for rotor
pub fn mat4x4_12(b:f32x4)->(f32x4,f32x4,f32x4,f32x4) {
  let buf = *(b * b).as_array();
  let b0_2 = buf[0];
  let b1_2 = buf[1];
  let b2_2 = buf[2];
  let b3_2 = buf[3];

  let mut c0 = b * shuffle_xzxx(b);
  let mut tmp = shuffle_ywyx(b) * shuffle_yxwx(b);
  tmp = f32x4_xor(f32x4::from([0.0, -0.0, 0.0, 0.0]), tmp); // TODO why reference?
  c0 = f32x4::from([1.0, 2.0, 2.0, 0.0]) * (c0 + tmp);
  c0 = c0 - f32x4::splat(b3_2 + b2_2);

  let c1 = b * shuffle_wywx(b);
  let mut tmp = shuffle_zwxx(b) * shuffle_ywyx(b);
  tmp = f32x4_xor(f32x4::from([0.0, 0.0, -0.0, 0.0]), tmp); // TODO why reference?
  let mut c1 = f32x4::from([2.0, -1.0, 2.0, 0.0]) * (c1 + tmp);
  c1 = c1 + f32x4::from([0.0, b0_2+b2_2, 0.0, 0.0]);

  let mut c2 = f32x4_xor(f32x4::from([-0.0, 0.0, -0.0, 0.0]), b * shuffle_zxzx(b));
  c2 = c2 + (shuffle_yzxx(b) * shuffle_wwxx(b));
  c2 = c2 * f32x4::from([2.0, 2.0, 1.0, 0.0]);
  c2 = c2 + f32x4::from([0.0, 0.0, b3_2 - b1_2, 0.0]);

  // TODO why is c3 here
  // c3 = _mm_add_ps(c3, _mm_set_ps(b0_2 + b1_2 + b2_2 + b3_2, 0.f, 0.f, 0.f));
  let c3 = f32x4::from([0.0, 0.0, 0.0, b0_2 + b1_2 + b2_2 + b3_2]);
  (c0,c1,c2,c3)
}

// true, false, for motor
pub fn mat4x4_12_m(b:f32x4, c:f32x4)->(f32x4,f32x4,f32x4,f32x4) {
  let buf = *(b * b).as_array();
  let b0_2 = buf[0];
  let b1_2 = buf[1];
  let b2_2 = buf[2];
  let b3_2 = buf[3];

  let mut c0 = b * shuffle_xzxx(b);
  let mut tmp = shuffle_ywyx(b) * shuffle_yxwx(b);
  tmp = f32x4_xor(f32x4::from([0.0, -0.0, 0.0, 0.0]), tmp); // TODO why reference?
  c0 = f32x4::from([1.0, 2.0, 2.0, 0.0]) * (c0 + tmp);
  c0 = c0 - f32x4::splat(b3_2 + b2_2);

  let c1 = b * shuffle_wywx(b);
  let mut tmp = shuffle_zwxx(b) * shuffle_ywyx(b);
  tmp = f32x4_xor(f32x4::from([0.0, 0.0, -0.0, 0.0]), tmp); // TODO why reference?
  let mut c1 = f32x4::from([2.0, -1.0, 2.0, 0.0]) * (c1 + tmp);
  c1 = c1 + f32x4::from([0.0, b0_2+b2_2, 0.0, 0.0]);

  let mut c2 = f32x4_xor(f32x4::from([-0.0, 0.0, -0.0, 0.0]), b * shuffle_zxzx(b));
  c2 = c2 + (shuffle_yzxx(b) * shuffle_wwxx(b));
  c2 = c2 * f32x4::from([2.0, 2.0, 1.0, 0.0]);
  c2 = c2 + f32x4::from([0.0, 0.0, b3_2 - b1_2, 0.0]);

  let mut c3 = b * shuffle_ywyx(c);
  c3 = c3 + shuffle_wxxx(b) * shuffle_zzwx(c);
  c3 = c3 + shuffle_yzwx(b) * shuffle_xxxx(c);
  tmp = shuffle_zwyx(b) * shuffle_wyzx(c);
  c3 = f32x4::from([2.0,2.0,2.0,0.0]) * (tmp - c3);

  c3 = c3 + f32x4::from([b0_2 + b1_2 + b2_2 + b3_2, 0.0, 0.0, 0.0]);

  (c0,c1,c2,c3)
}
