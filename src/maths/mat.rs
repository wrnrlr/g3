use std::simd::{f32x4};
use crate::maths::{Shuffle, f32x4_xor};

// false, false, for rotor
pub fn mat4x4_12(b:&f32x4)->(f32x4,f32x4,f32x4,f32x4) {
  let buf = *(b * b).as_array();
  let b0_2 = buf[0];
  let b1_2 = buf[1];
  let b2_2 = buf[2];
  let b3_2 = buf[3];

  let mut c0 = b * b.xzxx();
  let mut tmp = b.ywyx() * b.yxwx();
  tmp = f32x4_xor(&[0.0, -0.0, 0.0, 0.0].into(), &tmp); // TODO why reference?
  let one_twos:f32x4 = [1f32, 2.0, 2.0, 0.0].into();
  c0 =  one_twos * (c0 + tmp);
  c0 = c0 - f32x4::splat(b3_2 + b2_2);

  let c1 = b * b.wywx();
  let mut tmp = b.zwxx() * b.ywyx();
  tmp = f32x4_xor(&[0.0, 0.0, -0.0, 0.0].into(), &tmp); // TODO why reference?
  let tmp1 = f32x4::from_array([2.0, -1.0, 2.0, 0.0]);
  let mut c1:f32x4 = tmp1 * (c1 + tmp);
  let duno:f32x4 = [0.0, b0_2+b2_2, 0.0, 0.0].into();
  c1 = c1 + duno;

  let mut c2:f32x4 = f32x4_xor(&[-0.0, 0.0, -0.0, 0.0].into(), &(b * b.zxzx()));
  c2 = c2 + (b.yzxx() * b.wwxx());
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

  let mut c0:f32x4 = b * b.xzxx();
  let mut tmp = b.ywyx() * b.yxwx();
  tmp = f32x4_xor(&[0.0, -0.0, 0.0, 0.0].into(), &tmp);
  let one_twos:f32x4 = [1f32, 2.0, 2.0, 0.0].into();
  c0 = one_twos * (c0 + tmp);
  let tmp2:f32x4 = [b3_2 + b2_2, 0.0, 0.0, 0.0].into();
  c0 = c0 - tmp2;

  let c1 = b * b.wywx();
  let mut tmp = b.zwxx() * b.ywyx();
  tmp = f32x4_xor(&[0.0, 0.0, -0.0, 0.0].into(), &tmp);
  let mut c1 = &<[f32; 4] as Into<f32x4>>::into([2.0, -1.0, 2.0, 0.0]) * (c1 + tmp);
  c1 = c1 + &[0.0, b0_2+b2_2, 0.0, 0.0].into();

  let mut c2 = f32x4_xor(&[-0.0, 0.0, -0.0, 0.0].into(), &(b * b.zxzx()));
  c2 = c2 + (b.yzxx() * b.wwxx());
  c2 *= <[f32;4] as Into<f32x4>>::into([2.0, 2.0, 1.0, 0.0]);
  c2 += <[f32; 4] as Into<f32x4>>::into([0.0, 0.0, b3_2 - b1_2, 0.0]);

  let mut c3 = b * c.ywyx();
  c3 = c3 + b.wxxx() * c.zzwx();
  c3 = c3 + b.yzwx() * c.xxxx();
  tmp = b.zwyx() * c.wyzx();
  c3 = <[f32; 4] as Into<f32x4>>::into([2.0,2.0,2.0,0.0]) * (tmp - c3);

  // c3 = _mm_add_ps(c3, _mm_set_ps(b0_2 + b1_2 + b2_2 + b3_2, 0.f, 0.f, 0.f));
  c3 = c3 + <[f32; 4] as Into<f32x4>>::into([0.0, 0.0, 0.0, b0_2 + b1_2 + b2_2 + b3_2]);

  (c0,c1,c2,c3)
}
