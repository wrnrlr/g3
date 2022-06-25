use std::simd::{f32x4,mask32x4};
use crate::maths::{b0a1a2a3, shuffle_xxzz, shuffle_xyxy,util::{dp, flip_signs, rcp_nr1, shuffle_xxxx, shuffle_yyzw, shuffle_wxxx, shuffle_yzwy, shuffle_ywyz, shuffle_zyzw, shuffle_zxxx, shuffle_wwyz, shuffle_zzwy, shuffle_yxxx, shuffle_xwyz, shuffle_xzwy, shuffle_wzwy, shuffle_zwyz, add_ss, f32x4_xor, sub_ss, mul_ss, shuffle_yzyw, shuffle_yywz, shuffle_wywz, shuffle_wzyw, shuffle_zzww}};

// plane * plane
pub fn gp00(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // (a1 b1 + a2 b2 + a3 b3) +
  //
  // (a2 b3 - a3 b2) e23 +
  // (a3 b1 - a1 b3) e31 +
  // (a1 b2 - a2 b1) e12 +
  //
  // (a0 b1 - a1 b0) e01 +
  // (a0 b2 - a2 b0) e02 +
  // (a0 b3 - a3 b0) e03
  let mut p1_out = shuffle_yzwy(a) * shuffle_ywyz(b);
  p1_out = p1_out - (f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(shuffle_zwyz(a) * shuffle_zzwy(b))));
  // Add a3 b3 to the lowest component
  p1_out = add_ss(&p1_out, &(shuffle_wxxx(a) * shuffle_wxxx(b)));
  // (a0 b0, a0 b1, a0 b2, a0 b3)
  let mut p2_out = shuffle_xxxx(a) * b;
  // Sub (a0 b0, a1 b0, a2 b0, a3 b0)
  // Note that the lowest component cancels
  p2_out = p2_out - a * shuffle_xxxx(b);
  return (p1_out, p2_out);
}

// point * plane
pub fn gp30(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let mut p1 = a * shuffle_xxxx(b);
  p1 = b0a1a2a3(&p1, &f32x4::splat(0.0));

  // (_, a3 b2, a1 b3, a2 b1)
  let mut p2 = shuffle_xwyz(a) * shuffle_xzwy(b);
  p2 -= shuffle_xzwy(a) * shuffle_xwyz(b);
  // Compute a0 b0 + a1 b1 + a2 b2 + a3 b3 and store it in the low component
  let mut tmp = dp(a, b);
  tmp = flip_signs(&tmp, [true, false, false, false].into());
  p2 = b0a1a2a3(&p2, &tmp);
  (p1,p2)
}

// plane * point
pub fn gp03(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let mut p1 = a * shuffle_xxxx(b);
  p1 = b0a1a2a3(&p1, &f32x4::splat(0.0));

  // (_, a3 b2, a1 b3, a2 b1)
  let mut p2 = shuffle_xwyz(a) * shuffle_xzwy(b);
  p2 -= shuffle_xzwy(a) * shuffle_xwyz(b);
  // Compute a0 b0 + a1 b1 + a2 b2 + a3 b3 and store it in the low component
  let tmp = dp(a, b);
  p2 = p2 + tmp;
  (p1,p2)
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
  let mut p1_out = shuffle_xxxx(a) * b;
  p1_out -= shuffle_yzwy(a) * shuffle_ywyz(b);
  // In a separate register, accumulate the later components so we can
  // negate the lower single-precision element with a single instruction
  let tmp1 = shuffle_zyzw(a) * shuffle_zxxx(b);
  let tmp2 = shuffle_wwyz(a) * shuffle_wzwy(b);
  let tmp = f32x4_xor(&(tmp1 + tmp2), &[-0.0, 0.0, 0.0, 0.0].into());
  p1_out + tmp
}

pub fn gp33(a:&f32x4, b:&f32x4)->f32x4 {
  // (-a0 b0) +
  // (-a0 b1 + a1 b0) e01 +
  // (-a0 b2 + a2 b0) e02 +
  // (-a0 b3 + a3 b0) e03
  //
  // Produce a translator by dividing all terms by a0 b0
  let mut tmp = shuffle_xxxx(a) * b;
  // -2a0b0        | -a0b1        | -a0b2        | -a0b3
  tmp *= f32x4::from_array([-2.0, -1.0, -1.0, -1.0]);
  // -2a0b0 + a0b0 | -a0b1 + a1b0 | -a0b2 + a2b0 | -a0b3 + a3b0
  // -a0b0         | -a0b1 + a1b0 | -a0b2 + a2b0 | -a0b3 + a3b0
  tmp += a * shuffle_xxxx(b);

  // (0, 1, 2, 3) -> (0, 0, 2, 2)
  let mut ss = shuffle_xxzz(&tmp);
  ss = shuffle_xyxy(&ss);
  tmp = tmp * rcp_nr1(&ss);

  // TODO, in klein their is an extra `and`
  // flip_signs(tmp, mask32x4::from([false, true, true, true]))
  // mask32x4::from_array([false, true, true, true]).select(tmp, f32x4::splat(0.0))
  // f32x4_and(tmp, f32x4::from([0.0, -1.0, -1.0, -1.0]))
  tmp
}

pub fn gptr(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a2 b3 - a3 b2) e01 +
  // (a0 b2 + a3 b1 - a1 b3) e02 +
  // (a0 b3 + a1 b2 - a2 b1) e03
  let mut p2 = shuffle_yxxx(a) * shuffle_yyzw(b);
  p2 += shuffle_zzwy(a) * shuffle_zwyz(b);
  let tmp = shuffle_wwyz(a) * shuffle_wzwy(b);
  p2 - flip_signs(&tmp, [true,false,false,false].into())
}

pub fn gprt(a:&f32x4, b:&f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a3 b2 - a2 b3) e01 +
  // (a0 b2 + a1 b3 - a3 b1) e02 +
  // (a0 b3 + a2 b1 - a1 b2) e03
  let mut p2 = shuffle_yxxx(a) * shuffle_yyzw(b);
  p2 += shuffle_zwyz(a) * shuffle_zzwy(b);
  let tmp = shuffle_wzwy(a) * shuffle_wwyz(b);
  p2 - flip_signs(&tmp, mask32x4::from_array([true,false,false,false]))
}

pub fn gp12(a:&f32x4, b:&f32x4)->f32x4 {
  let p2 = gprt(a, b);
  let tmp = a * shuffle_xxxx(b);
  p2 - flip_signs(&tmp, mask32x4::from_array([true,false,false,false]))
}

pub fn gp21(a:&f32x4, b:&f32x4)->f32x4 {
  let p2 = gptr(a, b);
  let tmp = a * shuffle_xxxx(b);
  p2 - flip_signs(&tmp, mask32x4::from_array([true,false,false,false]))
}

pub fn gpll(a:&f32x4, d:&f32x4, b:&f32x4, c:&f32x4)->(f32x4, f32x4) {
  let flip = &[-0.0,0.0,0.0,0.0].into();
  let mut p1 = shuffle_yzyw(a) * shuffle_yywz(b);
  p1 = f32x4_xor(&p1, flip);
  p1 -= shuffle_wywz(a) * shuffle_wzyw(b);
  let a2 = &shuffle_zzww(a);
  let b2 = &shuffle_zzww(b);
  let p1 = sub_ss(&p1, mul_ss(a2, b2));

  let mut p2 = shuffle_ywyz(a) * shuffle_yzwy(c);
  p2 -= f32x4_xor(flip, &(shuffle_wzwy(a) * shuffle_wwyz(c)));
  p2 += shuffle_yzwy(b) * shuffle_ywyz(d);
  p2 -= f32x4_xor(flip, &(shuffle_wwyz(b) * shuffle_wzwy(d)));
  let c2 = shuffle_zzww(c);
  let d2 = shuffle_zzww(d);
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

  let a_xxxx = &shuffle_xxxx(a);
  let a_zyzw = &shuffle_zyzw(a);
  let a_ywyz = &shuffle_ywyz(a);
  let a_wzwy = &shuffle_wzwy(a);
  let c_wwyz = &shuffle_wwyz(c);
  let c_yzwy = &shuffle_yzwy(c);
  let s_flip = mask32x4::from_array([true, false, false, false]);

  let mut e = a_xxxx * c;
  let mut t = a_ywyz * c_yzwy;

  t += a_zyzw * shuffle_zxxx(c);
  t = flip_signs(&t, s_flip);

  e = e + t;
  e = e - a_wzwy * c_wwyz;

  let mut f = a_xxxx * d;
  f += b * shuffle_xxxx(c);
  f += a_ywyz * shuffle_yzwy(d);
  f += shuffle_ywyz(b) * c_yzwy;

  let mut t = a_zyzw * shuffle_zxxx(d);
  t += a_wzwy * shuffle_wwyz(d);
  t += shuffle_zxxx(b) * shuffle_zyzw(c);
  t += shuffle_wzwy(b) * c_wwyz;
  t = f32x4_xor(&t, &[-0.0,0.0,0.0,0.0].into());

  f = f - t;

  return (e, f);
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
