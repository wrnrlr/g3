use core_simd::{f32x4,mask32x4, simd_swizzle};
use crate::util::{dp, flip_signs, rcp_nr1, shuffle_xxxx, shuffle_yyzw, shuffle_wxxx,
                  shuffle_yzwy, shuffle_ywyz, shuffle_zyzw, shuffle_zxxx, shuffle_wwyz,
                  shuffle_zzwy, shuffle_yxxx, shuffle_xwyz, shuffle_xzwy, shuffle_wzwy,
                  shuffle_zwyz, add_ss};

// plane * plane
pub fn gp00(a:f32x4, b:f32x4)->(f32x4,f32x4) {
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
  p1_out = p1_out - (-(shuffle_zwyz(a) * shuffle_zzwy(b)));
  // Add a3 b3 to the lowest component
  p1_out = add_ss(p1_out, shuffle_wxxx(a) * shuffle_wxxx(b));
  // (a0 b0, a0 b1, a0 b2, a0 b3)
  let mut p2_out = shuffle_xxxx(a) * b;
  // Sub (a0 b0, a1 b0, a2 b0, a3 b0)
  // Note that the lowest component cancels
  p2_out = p2_out - a * shuffle_xxxx(b);
  return (p1_out, p2_out);
}

pub fn gp03<const F:bool>(a:f32x4, b:f32x4)->(f32x4,f32x4) {
  // a1 b0 e23 +
  // a2 b0 e31 +
  // a3 b0 e12 +
  // (a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a3 b2 - a2 b3) e01 +
  // (a1 b3 - a3 b1) e02 +
  // (a2 b1 - a1 b2) e03
  //
  // With flip:
  //
  // a1 b0 e23 +
  // a2 b0 e31 +
  // a3 b0 e12 +
  // -(a0 b0 + a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a3 b2 - a2 b3) e01 +
  // (a1 b3 - a3 b1) e02 +
  // (a2 b1 - a1 b2) e03
  let mut p1 = a * shuffle_xxxx(b);
  p1 = mask32x4::from_array([false, true, true, true]).select(p1, f32x4::splat(0.0));
  // (_, a3 b2, a1 b3, a2 b1)
  let mut p2 = shuffle_xwyz(a) * shuffle_xzwy(b);
  p2 -= shuffle_xzwy(a) * shuffle_xwyz(b);
  // Compute a0 b0 + a1 b1 + a2 b2 + a3 b3 and store it in the low component
  let mut tmp = dp(a, b);
  if F { tmp = -tmp}
  p2 = p2 - tmp;
  return (p1,p2);
}

// p1: (1, e23, e31, e12)
pub fn gp11(a:f32x4, b:f32x4)->f32x4 {
  // (a0 b0 - a1 b1 - a2 b2 - a3 b3) +
  // (a0 b1 - a2 b3 + a1 b0 + a3 b2)*e23
  // (a0 b2 - a3 b1 + a2 b0 + a1 b3)*e31
  // (a0 b3 - a1 b2 + a3 b0 + a2 b1)*e12

  // We use abcd to refer to the slots to avoid conflating bivector/scalar
  // coefficients with cartesian coordinates

  // In general, we can get rid of at most one swizzle
  let mut p1_out = shuffle_xxxx(a) * b;
  p1_out = p1_out - (shuffle_yzwy(a) * shuffle_ywyz(b));
  // In a separate register, accumulate the later components so we can
  // negate the lower single-precision element with a single instruction
  let tmp1 = shuffle_zyzw(a) * shuffle_zxxx(b);
  let tmp2 = shuffle_wwyz(a) * shuffle_wzwy(b);
  let tmp = -(tmp1 + tmp2);
  p1_out + tmp
}

pub fn gp33(a:f32x4, b:f32x4)->f32x4 {
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

  let ss = shuffle_xxxx(tmp);

  tmp = tmp * rcp_nr1(ss);
  mask32x4::from_array([false, true, true, true]).select(tmp, f32x4::splat(0.0))
}

// pub fn gpDL()->(f32x4,f32x4) { todo!() }

pub fn gp133()->(f32x4,f32x4) {
  todo!()
}

pub fn gprt<const F:bool>(a:f32x4, b:f32x4)->f32x4 {
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a2 b3 - a3 b2) e01 +
  // (a0 b2 + a3 b1 - a1 b3) e02 +
  // (a0 b3 + a1 b2 - a2 b1) e03
  // or
  // (a1 b1 + a2 b2 + a3 b3) e0123 +
  // (a0 b1 + a3 b2 - a2 b3) e01 +
  // (a0 b2 + a1 b3 - a3 b1) e02 +
  // (a0 b3 + a2 b1 - a1 b2) e03
  let mut p2 = shuffle_yxxx(a) * shuffle_yyzw(b);
  p2 = p2 + shuffle_zzwy(a) * shuffle_zwyz(b); // TODO other flip is different
  let tmp = if F { shuffle_wwyz(a) * shuffle_wzwy(b)} else { shuffle_wzwy(a) * shuffle_wwyz(b) };
  p2 - flip_signs(tmp, mask32x4::from_array([true,false,false,false])) // TODO Correct?
}

pub fn gp12<const F:bool>(a:f32x4, b:f32x4)->f32x4 {
  let p2 = gprt::<F>(a,b);
  let tmp = a * shuffle_xxxx(b);
  p2 - flip_signs(tmp, mask32x4::from_array([true,false,false,false]))
}

/// Motor * Motor Operation
pub fn gpmm(a:f32x4, b:f32x4, c:f32x4, d:f32x4)->(f32x4,f32x4) {
  // (a0 c0 - a1 c1 - a2 c2 - a3 c3) +
  // (a0 c1 + a3 c2 + a1 c0 - a2 c3) e23 +
  // (a0 c2 + a1 c3 + a2 c0 - a3 c1) e31 +
  // (a0 c3 + a2 c1 + a3 c0 - a1 c2) e12 +
  //
  // (a0 d0 + b0 c0 + a1 d1 + b1 c1 + a2 d2 + a3 d3 + b2 c2 + b3 c3) e0123 +
  // (a0 d1 + b1 c0 + a3 d2 + b3 c2 - a1 d0 - a2 d3 - b0 c1 - b2 c3) e01 +
  // (a0 d2 + b2 c0 + a1 d3 + b1 c3 - a2 d0 - a3 d1 - b0 c2 - b3 c1) e02 +
  // (a0 d3 + b3 c0 + a2 d1 + b2 c1 - a3 d0 - a1 d2 - b0 c3 - b1 c2) e03

  let a_xxxx = simd_swizzle!(a, [0,0,0,0]);
  let a_zyzw = simd_swizzle!(a, [2,1,2,3]);
  let a_ywyz = simd_swizzle!(a, [1,3,1,2]);
  let a_wzwy = simd_swizzle!(a, [3,2,3,1]);
  let c_wwyz = simd_swizzle!(c, [3,3,1,2]);
  let c_yzwy = simd_swizzle!(c, [1,2,3,1]);
  let s_flip = mask32x4::from_array([false, false, false, true]);

  let mut e = a_xxxx * c;
  let mut t = a_ywyz * c_yzwy;

  t = t + (a_zyzw * simd_swizzle!(c, [2,0,0,0]));
  t = flip_signs(t, s_flip);

  e = e + t;
  e = e - a_wzwy * c_wwyz;

  let mut f = a_xxxx * d;
  f = f + b * simd_swizzle!(c, [0,0,0,0]);
  f = f + a_ywyz * simd_swizzle!(d, [1,2,3,1]);
  f = f + simd_swizzle!(b, [1,2,1,2]) + c_yzwy;

  t = a_zyzw * simd_swizzle!(d, [2,0,0,0]);
  t = t + a_wzwy * simd_swizzle!(d, [3,3,1,2]);
  t = t + simd_swizzle!(b, [2,0,0,0]) * simd_swizzle!(c, [2,1,2,3]);
  t = t + simd_swizzle!(b, [3,2,3,1]) * c_wwyz;
  t = flip_signs(t, s_flip);

  f = f - t;

  return (e, f);
}
