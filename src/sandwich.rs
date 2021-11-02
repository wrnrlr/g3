use core_simd::{f32x4,mask32x4};
use crate::util::{add_ss, flip_signs, f32x4_xor, hi_dp, rcp_nr1,
  shuffle_wwww, shuffle_wwyz, shuffle_wyzw, shuffle_wyzx, shuffle_wzxy, shuffle_yyzw,
  shuffle_yyzz, shuffle_yzwy, shuffle_zwyz, shuffle_zzwy, shuffle_zyzw, shuffle_ywyz,
  shuffle_wzwy, shuffle_xzwy, shuffle_yyzx, shuffle_zzxy, shuffle_xxyz};

// p3: (w,    x,    y,    z)
// p3: (e123, e032, e013, e021)

// p0: (e0, e1, e2, e3)
// d, x, y, z

// Partition memory layouts
//     LSB --> MSB
// p0: (e0, e1, e2, e3)
// p1: (1, e23, e31, e12)
// p2: (e0123, e01, e02, e03)
// p3: (e123, e032, e013, e021)

// Reflect a plane through another plane
// b * a * b
pub fn sw00(a:f32x4,b:f32x4)->f32x4 {
  // (2a0(a2 b2 + a3 b3 + a1 b1) - b0(a1^2 + a2^2 + a3^2)) e0 +
  // (2a1(a2 b2 + a3 b3)         + b1(a1^2 - a2^2 - a3^2)) e1 +
  // (2a2(a3 b3 + a1 b1)         + b2(a2^2 - a3^2 - a1^2)) e2 +
  // (2a3(a1 b1 + a2 b2)         + b3(a3^2 - a1^2 - a2^2)) e3
  let a_zzwy = shuffle_zzwy(a);
  let a_wwyz = shuffle_wwyz(a);

  // Left block
  let mut tmp = a_zzwy * shuffle_zzwy(b);
  tmp = tmp + a_wwyz * shuffle_wwyz(b);

  let a1 = shuffle_yyzz(a);
  let b1 = shuffle_yyzz(b);
  tmp = tmp + a1 * b1;
  tmp = tmp * (a + a);

  // Right block
  let a_yyzw = shuffle_yyzw(a);
  let mut tmp2 = f32x4_xor(a_yyzw * a_yyzw, f32x4::splat(-0.0));
  tmp2 = tmp2 - a_zzwy * a_zzwy;
  tmp2 = tmp2 - a_wwyz * a_wwyz;
  tmp2 = tmp2 * b;

  return tmp + tmp2
}

pub fn sw10(a:f32x4,b:f32x4)->(f32x4,f32x4) {
  //                       b0(a1^2 + a2^2 + a3^2) +
  // (2a3(a1 b1 + a2 b2) + b3(a3^2 - a1^2 - a2^2)) e12 +
  // (2a1(a2 b2 + a3 b3) + b1(a1^2 - a2^2 - a3^2)) e23 +
  // (2a2(a3 b3 + a1 b1) + b2(a2^2 - a3^2 - a1^2)) e31 +
  //
  // 2a0(a1 b2 - a2 b1) e03
  // 2a0(a2 b3 - a3 b2) e01 +
  // 2a0(a3 b1 - a1 b3) e02

  let a_zyzw = shuffle_zyzw(a);
  let a_ywyz = shuffle_ywyz(a);
  let a_wzwy = shuffle_wzwy(a);

  let b_xzwy = shuffle_xzwy(b);

  let two_zero = f32x4::from_array([0.0, 2.0, 2.0, 2.0]);
  let mut p1 = a * b;
  p1 = p1 + a_wzwy * b_xzwy;
  p1 = a_ywyz *  two_zero * p1;

  let mut tmp = a_zyzw * a_zyzw;
  tmp = tmp + a_wzwy * a_wzwy;
  tmp = -tmp;
  tmp = (a_ywyz * a_ywyz) - tmp;
  tmp = shuffle_wzxy(b) * tmp;

  let p1 = shuffle_wyzx(p1 + tmp);

  let mut p2 = a_zyzw * b_xzwy;
  p2 = p2 - a_wzwy * b;
  p2 = p2 * shuffle_wwww(a) * two_zero;
  p2 = shuffle_wyzx(p2);

  (p1,p2)
}

pub fn sw20(a:f32x4,b:f32x4)->f32x4 {
  //                       -b0(a1^2 + a2^2 + a3^2) e0123 +
  // (-2a3(a1 b1 + a2 b2) + b3(a1^2 + a2^2 - a3^2)) e03
  // (-2a1(a2 b2 + a3 b3) + b1(a2^2 + a3^2 - a1^2)) e01 +
  // (-2a2(a3 b3 + a1 b1) + b2(a3^2 + a1^2 - a2^2)) e02 +
  let a_zzwy = shuffle_yyzx(a);
  let a_wwyz = shuffle_zzxy(a);

  let mut p2 = a * b;
  p2 = p2 + a_zzwy * shuffle_wyzx(b);
  p2 = p2 * (a_wwyz * f32x4::from_array([0.0, -2.0, -2.0, -2.0]));

  let a_yyzw = shuffle_xxyz(a);
  let mut tmp = a_yyzw * a_yyzw;
  tmp = -(tmp + (a_zzwy * a_zzwy));
  tmp = tmp - (a_wwyz * a_wwyz);
  p2 = p2 + tmp * shuffle_wzxy(b);
  shuffle_wyzx(p2)
}

// reflect point through plane
pub fn sw30(a:f32x4, b:f32x4) ->f32x4 {
  //                                b0(a1^2 + a2^2 + a3^2)  e123 +
  // (-2a1(a0 b0 + a3 b3 + a2 b2) + b1(a2^2 + a3^2 - a1^2)) e032 +
  // (-2a2(a0 b0 + a1 b1 + a3 b3) + b2(a3^2 + a1^2 - a2^2)) e013 +
  // (-2a3(a0 b0 + a2 b2 + a1 b1) + b3(a1^2 + a2^2 - a3^2)) e021

  let a_zwyz = shuffle_zwyz(a);
  let a_yzwy = shuffle_yzwy(a);
  let a_wyzw = shuffle_wyzw(a);

  let mut p3_out = shuffle_wwww(a) * shuffle_wwww(b);
  p3_out += a_zwyz * shuffle_wzxy(b);
  p3_out += a_yzwy * shuffle_wyzx(b);
  p3_out *= a * f32x4::from_array([0.0,-2.0,-2.0,-2.0]);

  let mut tmp = a_yzwy * a_yzwy;
  tmp += (a_zwyz * a_zwyz);
  tmp -= f32x4_xor(a_wyzw * a_wyzw, f32x4::from_array([-0.0,0.0,0.0,0.0]));

  p3_out + b * tmp
}

pub fn sw012<const N:bool,const F:bool>(_p0:f32x4,_p1:f32x4)->f32x4 {
  todo!()
}

pub fn swmm<const N:bool,const F:bool,const P:bool>(_a:f32x4,_b:f32x4,_c:Option<f32x4>)->(f32x4,f32x4) { // todo, c doesn't seem to be used add count argument
  todo!()
}

// Apply a translator to a plane.
// Assumes e0123 component of p2 is exactly 0
// p0: (e0, e1, e2, e3)
// p2: (e0123, e01, e02, e03)
// b * a * ~b
// The low component of p2 is expected to be the scalar component instead
pub fn sw02(a:f32x4, b:f32x4)->f32x4 {
  // (a0 b0^2 + 2a1 b0 b1 + 2a2 b0 b2 + 2a3 b0 b3) e0 +
  // (a1 b0^2) e1 +
  // (a2 b0^2) e2 +
  // (a3 b0^2) e3
  //
  // Because the plane is projectively equivalent on multiplication by a
  // scalar, we can divide the result through by b0^2
  //
  // (a0 + 2a1 b1 / b0 + 2a2 b2 / b0 + 2a3 b3 / b0) e0 +
  // a1 e1 +
  // a2 e2 +
  // a3 e3
  //
  // The additive term clearly contains a dot product between the plane's
  // normal and the translation axis, demonstrating that the plane
  // "doesn't care" about translations along its span. More precisely, the
  // plane translates by the projection of the translator on the plane's
  // normal.

  // a1*b1 + a2*b2 + a3*b3 stored in the low component of tmp
  let mut tmp = hi_dp(a, b);
  let mut inv_b = rcp_nr1(b);
  // 2 / b0
  inv_b = add_ss(inv_b, inv_b);
  // p1_out = _mm_and_ps(p1_out, _mm_castsi128_ps(_mm_set_epi32(-1, -1, -1, 0)));
  inv_b = mask32x4::from_array([true, false, false, false]).select(inv_b, f32x4::splat(0.0));
  tmp = tmp * inv_b;
  a * tmp
}

// Apply a translator to a point.
// Assumes e0123 component of p2 is exactly 0
// p2: (e0123, e01, e02, e03)
// p3: (e123, e032, e013, e021)
// b * a * ~b
pub fn sw32(a:f32x4, b:f32x4)->f32x4 {
  // a0 e123 +
  // (a1 - 2 a0 b1) e032 +
  // (a2 - 2 a0 b2) e013 +
  // (a3 - 2 a0 b3) e021
  let mut tmp = shuffle_wwww(a) * b;
  tmp *= f32x4::from_array([0.0, -2.0, -2.0, -2.0]);
  a + tmp
}

// Apply a translator to a line
// a := p1 input
// d := p2 input
// c := p2 translator
// out points to the start address of a line (p1, p2)
pub fn swl2(a:f32x4, d:f32x4, c:f32x4)->(f32x4, f32x4) {
  // a0 + a1 e23 + a2 e31 + a3 e12 +
  //
  // (2a0 c0 + d0) e0123 +
  // (2(a2 c3 - a3 c2 - a1 c0) + d1) e01 +
  // (2(a3 c1 - a1 c3 - a2 c0) + d2) e02 +
  // (2(a1 c2 - a2 c1 - a3 c0) + d3) e03
  let mut p2_out = shuffle_wyzx(a) * shuffle_wzxy(c);
  // Add and subtract the same quantity in the low component to produce a cancellation
  p2_out -= shuffle_wzxy(a) * shuffle_wyzx(c);
  p2_out -= flip_signs(a * shuffle_wwww(c), mask32x4::from_array([true, false, false, false]));
  (a, p2_out + p2_out + d)
}

pub fn sw312<const N:bool,const F:bool>(_a:f32x4,_b:f32x4,_c:f32x4)->f32x4 { // todo count param
  todo!()
}

pub fn swo12(_a:f32x4, _b:f32x4)->f32x4 {
  todo!()
}
