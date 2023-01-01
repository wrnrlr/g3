use std::simd::{f32x4,mask32x4};
use crate::maths::*;

/// a + b*e23 + c*e31 + d*e12 + e*e01 + f*e02 + g*e03 + h*e0123
// plane * plane
pub fn gp00(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (a1b1 + a2b2 + a3b3) + (a2b3 - a3b2)e23 + (a3b1 - a1b3)e31 + (a1b2 - a2b1)e12 +
  // (a0b1 - a1b0)e01 + (a0b2 - a2b0)e02 + (a0b3 - a3b0)e03
  let mut p1_out = a.yzwy() * b.ywyz();
  p1_out = p1_out - (f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(a.zwyz() * b.zzwy())));
  // Add a3 b3 to the lowest component
  p1_out = add_ss(&p1_out, &(a.wxxx() * b.wxxx()));
  // (a0 b0, a0 b1, a0 b2, a0 b3)
  let mut p2_out = a.xxxx() * b;
  // Sub (a0 b0, a1 b0, a2 b0, a3 b0)
  // Note that the lowest component cancels
  p2_out = p2_out - a * b.xxxx();
  return (p1_out, p2_out);
}

// point * plane
pub fn gp30(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let mut p1 = a * b.xxxx();
  p1 = b0a1a2a3(&p1, &f32x4::splat(0.0));

  // (_, a3 b2, a1 b3, a2 b1)
  let mut p2 = a.xwyz() * b.xzwy();
  p2 -= a.xzwy() * b.xwyz();
  // Compute a0 b0 + a1 b1 + a2 b2 + a3 b3 and store it in the low component
  let mut tmp = dp(a, b);
  tmp = flip_signs(&tmp, [true, false, false, false].into());
  p2 = b0a1a2a3(&p2, &tmp);
  (p1,p2)
}

// plane * point
pub fn gp03(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (_, a3 b2, a1 b3, a2 b1)
  // Compute a0 b0 + a1 b1 + a2 b2 + a3 b3 and store it in the low component
  (b0a1a2a3(&(a * b.xxxx()), &f32x4::splat(0.0)),
   a.xwyz() * b.xzwy() - a.xzwy() * b.xwyz() + dp(a, b))
}

// p1: (1, e23, e31, e12)
pub fn gp11(a:&f32x4, b:&f32x4)->f32x4 {
  // (a0 b0 - a1 b1 - a2 b2 - a3 b3) +
  // (a0 b1 - a2 b3 + a1 b0 + a3 b2)*e23
  // (a0 b2 - a3 b1 + a2 b0 + a1 b3)*e31
  // (a0 b3 - a1 b2 + a3 b0 + a2 b1)*e12

  // We use abcd to refer to the slots to avoid conflating bivector/scalar
  // coefficients with cartesian coordinates

  // In general, we can get rid of at most one swizzle
  let mut p1_out = a.xxxx() * b;
  p1_out -= a.yzwy() * b.ywyz();
  // In a separate register, accumulate the later components so we can
  // negate the lower single-precision element with a single instruction
  let tmp1 = a.zyzw() * b.zxxx();
  let tmp2 = a.wwyz() * b.wzwy();
  let tmp = f32x4_xor(&(tmp1 + tmp2), &[-0.0, 0.0, 0.0, 0.0].into());
  p1_out + tmp
}

pub fn gptr(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a2 b3 - a3 b2) e01 +
  // (a0 b2 + a3 b1 - a1 b3) e02 +
  // (a0 b3 + a1 b2 - a2 b1) e03
  let mut p2 = a.yxxx() * b.yyzw();
  p2 += a.zzwy() * b.zwyz();
  let tmp = a.wwyz() * b.wzwy();
  p2 - f32x4_xor(&tmp, &[-0.0,0.0,0.0,0.0].into())
}

pub fn gprt(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a3 b2 - a2 b3) e01 +
  // (a0 b2 + a1 b3 - a3 b1) e02 +
  // (a0 b3 + a2 b1 - a1 b2) e03
  let mut p2 = a.yxxx() * b.yyzw();
  p2 += a.zwyz() * b.zzwy();
  let tmp = a.wzwy() * b.wwyz();
  p2 - f32x4_xor(&tmp, &[-0.0,0.0,0.0,0.0].into())
}

pub fn gp12(a:&f32x4, b:&f32x4)->f32x4 {
  let p2 = gprt(a, b);
  let tmp = a * b.xxxx();
  p2 - flip_signs(&tmp, mask32x4::from_array([true,false,false,false]))
}

pub fn gp21(a:&f32x4, b:&f32x4)->f32x4 {
  let p2 = gptr(a, b);
  let tmp = a * b.xxxx();
  p2 - flip_signs(&tmp, mask32x4::from_array([true,false,false,false]))
}

pub(crate) fn gpll(a:&f32x4, d:&f32x4, b:&f32x4, c:&f32x4)->(f32x4, f32x4) {
  let flip = &[-0.0,0.0,0.0,0.0].into();
  let mut p1 = a.yzyw() * b.yywz();
  p1 = f32x4_xor(&p1, flip);
  p1 -= a.wywz() * b.wzyw();
  let a2 = a.zzww();
  let b2 = b.zzww();
  let p1 = sub_ss(&p1, mul_ss(&a2, &b2));

  let mut p2 = a.ywyz() * c.yzwy();
  p2 -= f32x4_xor(flip, &(a.wzwy() * c.wwyz()));
  p2 += b.yzwy() * d.ywyz();
  p2 -= f32x4_xor(flip, &(b.wwyz() * d.wzwy()));
  let c2 = c.zzww();
  let d2 = d.zzww();
  p2 = add_ss(&p2, &(a2*c2));
  p2 = add_ss(&p2, &(b2*d2));
  (p1, p2)
}

/// Motor * Motor Operation
pub fn gpmm(a:&f32x4, b:&f32x4, c:&f32x4, d:&f32x4)->(f32x4,f32x4) {
  // (a0 c0 - a1 c1 - a2 c2 - a3 c3) +
  // (a0 c1 + a3 c2 + a1 c0 - a2 c3) e23 +
  // (a0 c2 + a1 c3 + a2 c0 - a3 c1) e31 +
  // (a0 c3 + a2 c1 + a3 c0 - a1 c2) e12 +
  //
  // (a0 d0 + b0 c0 + a1 d1 + b1 c1 + a2 d2 + a3 d3 + b2 c2 + b3 c3) e0123 +
  // (a0 d1 + b1 c0 + a3 d2 + b3 c2 - a1 d0 - a2 d3 - b0 c1 - b2 c3) e01 +
  // (a0 d2 + b2 c0 + a1 d3 + b1 c3 - a2 d0 - a3 d1 - b0 c2 - b3 c1) e02 +
  // (a0 d3 + b3 c0 + a2 d1 + b2 c1 - a3 d0 - a1 d2 - b0 c3 - b1 c2) e03

  let a_xxxx = a.xxxx();
  let a_zyzw = a.zyzw();
  let a_ywyz = a.ywyz();
  let a_wzwy = a.wzwy();
  let c_wwyz = c.wwyz();
  let c_yzwy = c.yzwy();
  let s_flip = mask32x4::from_array([true, false, false, false]);

  let mut e = a_xxxx * c;
  let mut t = a_ywyz * c_yzwy;

  t += a_zyzw * c.zxxx();
  t = flip_signs(&t, s_flip);

  e = e + t;
  e = e - a_wzwy * c_wwyz;

  let f = a_xxxx * d + b * c.xxxx() + a_ywyz * d.yzwy() + b.ywyz() * c_yzwy;

  let mut t = a_zyzw * d.zxxx();
  t += a_wzwy * d.wwyz();
  t += b.zxxx() * c.zyzw();
  t += b.wzwy() * c_wwyz;
  t = f32x4_xor(&t, &[-0.0,0.0,0.0,0.0].into());

  return (e, f-t);
}

pub fn gpdl(u:f32, v:f32, b:&f32x4, c:&f32x4)->(f32x4,f32x4) {
  // b1 u e23 +
  // b2 u e31 +
  // b3 u e12 +
  // (-b1 v + c1 u) e01 +
  // (-b2 v + c2 u) e02 +
  // (-b3 v + c3 u) e03
  let u_vec = f32x4::splat(u);
  let v_vec = f32x4::splat(v);
  let p1 = u_vec * b;
  let p2 = c * u_vec - b * v_vec;
  (p1,p2)
}
