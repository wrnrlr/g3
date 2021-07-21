use core_simd::{f32x4,mask32x4,Mask32};
use crate::util::{b2b3a2a3, dp, f32x4_flip_signs, rcp_nr1, shuffle_wwww,
  shuffle_wwyy, shuffle_wyzx, shuffle_wzxy, shuffle_xwww, shuffle_xxyz,
  shuffle_xyzx, shuffle_xzxy, shuffle_ywww, shuffle_yxyz, shuffle_yyzx,
  shuffle_yzxy, shuffle_zwww, shuffle_zyzx, shuffle_zzxy, add_ss};

// plane * plane
pub fn gp00(a:f32x4,b:f32x4)->(f32x4,f32x4) {
  // (a1 b1 + a2 b2 + a3 b3) +
  //
  // (a2 b3 - a3 b2) e23 +
  // (a3 b1 - a1 b3) e31 +
  // (a1 b2 - a2 b1) e12 +
  //
  // (a0 b1 - a1 b0) e01 +
  // (a0 b2 - a2 b0) e02 +
  // (a0 b3 - a3 b0) e03
  let mut p1_out = shuffle_xyzx(a) * shuffle_xzxy(b);
  p1_out = p1_out - (-(shuffle_yzxy(a) * shuffle_yyzx(b)));
  // Add a3 b3 to the lowest component
  p1_out = add_ss(p1_out, shuffle_zwww(a) * shuffle_zwww(b));
  // (a0 b0, a0 b1, a0 b2, a0 b3)
  let mut p2_out = shuffle_wwww(a) * b;
  // Sub (a0 b0, a1 b0, a2 b0, a3 b0)
  // Note that the lowest component cancels
  p2_out = p2_out - a * shuffle_wwww(b);
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
  let mut p1 = a * shuffle_wwww(b);
  p1 = mask32x4::from_array([false, true, true, true]).select(p1, f32x4::splat(0.0));
  // (_, a3 b2, a1 b3, a2 b1)
  let mut p2 = shuffle_wzxy(a) * shuffle_wyzx(b);
  p2 -= shuffle_wyzx(a) * shuffle_wzxy(b);
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
  let mut p1_out = shuffle_wwww(a) * b;
  p1_out = p1_out - (shuffle_xyzx(a) * shuffle_xzxy(b));
  // In a separate register, accumulate the later components so we can
  // negate the lower single-precision element with a single instruction
  let tmp1 = shuffle_yxyz(a) * shuffle_ywww(b);
  let tmp2 = shuffle_zzxy(a) * shuffle_zyzx(b);
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
  let mut tmp = shuffle_wwww(a) * b;
  tmp = tmp * f32x4::from_array([-2.0, -1.0, -1.0, -1.0]);
  tmp = tmp + a * shuffle_wwww(b);
  // (0, 1, 2, 3) -> (0, 0, 2, 2)
  let mut ss = shuffle_wwyy(tmp);
  ss = b2b3a2a3(ss,ss);
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
  let mut p2 = shuffle_xwww(a) * shuffle_xxyz(b);
  p2 = p2 + shuffle_yyzx(a) * shuffle_yzxy(b);
  let tmp = if F { shuffle_zzxy(a) * shuffle_zyzx(b)} else { shuffle_zyzx(a) * shuffle_zzxy(b) };
  p2 - f32x4_flip_signs(tmp, Mask32::from_array([true,false,false,false])) // Correct?
}

pub fn gp12<const F:bool>(a:f32x4, b:f32x4)->f32x4 {
  let p2 = gprt::<F>(a,b);
  let tmp = a * shuffle_wwww(b);
  p2 - f32x4_flip_signs(tmp, Mask32::from_array([true,false,false,false]))
}

pub fn gpmm(_a:f32x4,_b:f32x4)->(f32x4,f32x4) {
  // (a0 c0 - a1 c1 - a2 c2 - a3 c3) +
  // (a0 c1 + a3 c2 + a1 c0 - a2 c3) e23 +
  // (a0 c2 + a1 c3 + a2 c0 - a3 c1) e31 +
  // (a0 c3 + a2 c1 + a3 c0 - a1 c2) e12 +
  //
  // (a0 d0 + b0 c0 + a1 d1 + b1 c1 + a2 d2 + a3 d3 + b2 c2 + b3 c3) e0123 +
  // (a0 d1 + b1 c0 + a3 d2 + b3 c2 - a1 d0 - a2 d3 - b0 c1 - b2 c3) e01 +
  // (a0 d2 + b2 c0 + a1 d3 + b1 c3 - a2 d0 - a3 d1 - b0 c2 - b3 c1) e02 +
  // (a0 d3 + b3 c0 + a2 d1 + b2 c1 - a3 d0 - a1 d2 - b0 c3 - b1 c2) e03
  todo!()
}

