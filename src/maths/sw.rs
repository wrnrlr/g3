use std::simd::{f32x4,mask32x4,simd_swizzle as swizzle,Which::{First,Second}};
use crate::maths::util::{Shuffle, add_ss, flip_signs, f32x4_xor, hi_dp, rcp_nr1,mul_ss};

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
pub fn sw00(a:&f32x4,b:&f32x4)->f32x4 {
  // (2a0(a2 b2 + a3 b3 + a1 b1) - b0(a1^2 + a2^2 + a3^2)) e0 +
  // (2a1(a2 b2 + a3 b3)         + b1(a1^2 - a2^2 - a3^2)) e1 +
  // (2a2(a3 b3 + a1 b1)         + b2(a2^2 - a3^2 - a1^2)) e2 +
  // (2a3(a1 b1 + a2 b2)         + b3(a3^2 - a1^2 - a2^2)) e3
  let a_zzwy = a.zzwy();
  let a_wwyz = a.wwyz();

  // Left block
  let mut tmp = a_zzwy * b.zzwy();
  tmp += a_wwyz * b.wwyz();

  let a1 = &a.yyww();
  let b1 = &b.yyww();
  tmp = add_ss(&tmp, &mul_ss(a1, b1));
  tmp *= a + a;

  // Right block
  let a_yyzw = &a.yyzw();
  let mut tmp2 = f32x4_xor(&(a_yyzw * a_yyzw), &[-0.0, 0.0, 0.0, 0.0].into());
  tmp2 -= a_zzwy * a_zzwy;
  tmp2 -= a_wwyz * a_wwyz;
  tmp2 *= b;

  tmp + tmp2
}

pub fn sw10(a:&f32x4,b:&f32x4)->(f32x4,f32x4) {
  //                       b0(a1^2 + a2^2 + a3^2) +
  // (2a3(a1 b1 + a2 b2) + b3(a3^2 - a1^2 - a2^2)) e12 +
  // (2a1(a2 b2 + a3 b3) + b1(a1^2 - a2^2 - a3^2)) e23 +
  // (2a2(a3 b3 + a1 b1) + b2(a2^2 - a3^2 - a1^2)) e31 +
  //
  // 2a0(a1 b2 - a2 b1) e03
  // 2a0(a2 b3 - a3 b2) e01 +
  // 2a0(a3 b1 - a1 b3) e02

  let a_zyzw = &a.zyzw();
  let a_ywyz = &a.ywyz();
  let a_wzwy = &a.wzwy();
  let b_xzwy = &b.xzwy();

  let two_zero:f32x4 = [0.0, 2.0, 2.0, 2.0].into(); // TODO is this right?
  let mut p1 = a * b;
  p1 += a_wzwy * b_xzwy;
  p1 *= a_ywyz * two_zero;

  let mut tmp = a_zyzw * a_zyzw;
  tmp += a_wzwy * a_wzwy;
  tmp = f32x4_xor(&tmp, &[-0.0, 0.0, 0.0, 0.0].into());
  tmp = (a_ywyz * a_ywyz) - tmp;
  tmp = b.xwyz() * tmp;

  let p1 = (p1 + tmp).xzwy();

  let mut p2 = a_zyzw * b_xzwy;
  p2 = p2 - a_wzwy * b;
  p2 = p2 * a.xxxx() * two_zero;
  p2 = p2.xzwy();

  (p1,p2)
}

pub fn sw20(a:&f32x4,b:&f32x4)->f32x4 {
  //                       -b0(a1^2 + a2^2 + a3^2) e0123 +
  // (-2a3(a1 b1 + a2 b2) + b3(a1^2 + a2^2 - a3^2)) e03
  // (-2a1(a2 b2 + a3 b3) + b1(a2^2 + a3^2 - a1^2)) e01 +
  // (-2a2(a3 b3 + a1 b1) + b2(a3^2 + a1^2 - a2^2)) e02 +
  let a_zzwy = a.zzwy();
  let a_wwyz = a.wwyz();

  let mut p2 = a * b;
  p2 += a_zzwy * b.xzwy();
  p2 *= a_wwyz * &[0.0, -2.0, -2.0, -2.0].into();

  let a_yyzw = a.yyzw();
  let mut tmp = a_yyzw * a_yyzw;
  tmp = f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(tmp + a_zzwy * a_zzwy));
  tmp -= a_wwyz * a_wwyz;
  p2 += tmp * b.xwyz();
  p2.xzwy()
}

// reflect point through plane
pub fn sw30(a:&f32x4, b:&f32x4) ->f32x4 {
  //                                b0(a1^2 + a2^2 + a3^2)  e123 +
  // (-2a1(a0 b0 + a3 b3 + a2 b2) + b1(a2^2 + a3^2 - a1^2)) e032 +
  // (-2a2(a0 b0 + a1 b1 + a3 b3) + b2(a3^2 + a1^2 - a2^2)) e013 +
  // (-2a3(a0 b0 + a2 b2 + a1 b1) + b3(a1^2 + a2^2 - a3^2)) e021

  let a_zwyz = &a.zwyz(); // a2, a3, a1, a2
  let a_yzwy = &a.yzwy(); // a1, a2, a3, a1
  let a_wyzw = &a.wyzw(); // a3, a1, a2, a3

  //     a0 b0              |      a0 b0              |      a0 b0              |      a0 b0
  let mut p3_out = a.xxxx() * b.xxxx();
  //     a0 b0+a2 b0        |      a0 b0+a3 b3        |      a0 b0+a1 b1        |      a0 b0+a3 b2
  p3_out += a_zwyz * b.xwyz();
  //     a0 b0+a2 b0+a1 b0  |      a0 b0+a3 b3+a2 b2  |      a0 b0+a1 b0+a3 b3  |      a0 b0+a3 b2+a1 b1
  p3_out += a_yzwy * b.xzwy();
  // 0b0(a0 b0+a2 b0+a1 b0) | -2a1(a0 b0+a3 b3+a2 b2) | -2a2(a0 b0+a1 b0+a3 b3) | -2a3(a0 b0+a3 b2+a1 b1)
  p3_out *= a * f32x4::from_array([0.0,-2.0,-2.0,-2.0]);
  //                        | -2a1(a0 b0+a3 b3+a2 b2) | -2a2(a0 b0+a1 b0+a3 b3) | -2a3(a0 b0+a3 b2+a1 b1)

  // a1^2           | a2^2           | a3^2           | a1^2
  let mut tmp = a_yzwy * a_yzwy;
  // a1^2+a2^2      | a2^2+a3^2      | a3^2+a1^2      | a1^2+a2^2
  tmp += a_zwyz * a_zwyz;
  // a1^2+a2^2+a3^2 | a2^2+a3^2-a1^2 | a3^2+a1^2-a2^2 | a1^2+a2^2-a3^2
  tmp -= f32x4_xor(&(a_wyzw * a_wyzw), &f32x4::from_array([-0.0,0.0,0.0,0.0]));

  p3_out = p3_out + b * tmp;

  p3_out
}

// rotor(point), rotor(plane), rotor(direction): false, false
pub fn sw01(a:&f32x4, b:&f32x4)->f32x4 {
  let dc_scale = f32x4::from_array([1.0,2.0,2.0,2.0]);
  let b_xwyz = &b.xwyz();
  let b_xzwy = &b.xzwy();
  let b_xxxx = &b.xxxx();

  let mut tmp1 = b.zxxx() * b.zwyz();
  tmp1 += b.yzwy() * b.yyzw();
  tmp1 *= dc_scale;

  let mut tmp2 = b * b_xwyz;
  let true_falses:mask32x4 = [true,false,false,false].into();
  tmp2 -= flip_signs(&(b.wxxx() * b.wzwy()), true_falses);
  tmp2 *= dc_scale;

  let mut tmp3 = b * b;
  tmp3 -= b_xwyz * b_xwyz;
  tmp3 += b_xxxx * b_xxxx;
  tmp3 -= b_xzwy * b_xzwy;

  let mut out:f32x4;

  out = tmp1 * a.xzwy();
  out += tmp2 * a.xwyz();
  out += tmp3 * a;

  out
}

// motor(plane), motor(point)
pub fn sw012(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  // Double-cover scale
  let dc_scale = f32x4::from_array([1.0,2.0,2.0,2.0]);
  let b_xwyz = b.xwyz();
  let b_xzwy = b.xzwy();
  let b_xxxx = b.xxxx();

  let mut tmp1 = b.zxxx() * b.zwyz();
  tmp1 += b.yzwy() * b.yyzw();
  // Scale later with (a0, a2, a3, a1)
  tmp1 *= dc_scale;

  let mut tmp2 = b * b_xwyz;
  tmp2 -= f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(b.wxxx() * b.wzwy()));
  // Scale later with (a0, a3, a1, a2)
  tmp2 *= dc_scale;

  // Alternately add and subtract to improve low component stability
  let mut tmp3 = b * b;
  tmp3 -= b_xwyz * b_xwyz;
  tmp3 += b_xxxx * b_xxxx;
  tmp3 -= b_xzwy * b_xzwy;

  let mut tmp4 = b_xxxx * c;
  tmp4 += b_xzwy * c.xwyz();
  tmp4 += b * c.xxxx();

  // NOTE: The high component of tmp4 is meaningless here
  tmp4 -= b_xwyz * c.xzwy();
  tmp4 *= dc_scale;

  let mut p = tmp1 * a.xzwy(); // TODO a[1]...
  p += tmp2 * a.xwyz();
  p += tmp3 * a; // TODO should be a[1]

  let tmp5 = hi_dp(&tmp4, a);
  let out = p + tmp5;
  out
}

// motor(line), swmm<false, true, true>
pub fn swml(a1:&f32x4, a2:&f32x4, b:&f32x4, c:&f32x4)->(f32x4,f32x4) {
  let b_xwyz = b.xwyz();
  let b_xzwy = b.xzwy();
  let b_yxxx = b.yxxx();
  let b_yxxx_2 = b_yxxx * b_yxxx;

  let mut tmp = b * b;
  tmp = tmp + b_yxxx_2;
  let b_tmp = b.zwyz();
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = b.wzwy();
  tmp2 += b_tmp * b_tmp;
  let true_falses:mask32x4 = [true,false,false,false].into();
  tmp -= flip_signs(&tmp2, true_falses);

  let b_xxxx = b.xxxx();
  let zero_twos = f32x4::from_array([0.0, 2.0, 2.0, 2.0]);
  let scale = &zero_twos;
  tmp2 = b_xxxx * b_xwyz;
  tmp2 += b * b_xzwy;
  tmp2 = tmp2 * scale;

  let mut tmp3 = b * b_xwyz;
  tmp3 -= b_xxxx * b_xzwy;
  tmp3 = tmp3 * scale;

  let czero = c.xxxx();
  let c_xzwy = c.xzwy();
  let c_xwyz = c.xwyz();

  let mut tmp4 = b * c;
  tmp4 -= b_yxxx * c.yxxx();
  tmp4 -= b.zwwy() * c.zwwy();
  tmp4 -= b.wzyz() * c.wzyz();
  tmp4 = tmp4 + tmp4;

  let mut tmp5 = b * c_xwyz;
  tmp5 += b_xzwy * czero;
  tmp5 += b_xwyz * c;
  tmp5 -= b_xxxx * c_xzwy;
  tmp5 = tmp5 * scale;

  let mut tmp6 = b * c_xzwy;
  tmp6 += b_xxxx * c_xwyz;
  tmp6 += b_xzwy * c;
  tmp6 -= b_xwyz * czero;
  tmp6 = tmp6 * scale;

  let p1_in_xzwy = a1.xzwy();
  let p1_in_xwyz = a1.xwyz();

  let mut p1_out = tmp * a1;
  p1_out = p1_out + tmp2 * p1_in_xzwy;
  p1_out = p1_out + tmp3 * p1_in_xwyz;

  let mut p2_out = tmp * a2;
  p2_out += tmp2 * a2.xzwy();
  p2_out += tmp3 * a2.xwyz();

  p2_out += tmp4 * a1;
  p2_out += tmp5 * p1_in_xwyz;
  p2_out += tmp6 * p1_in_xzwy;

  (p1_out, p2_out)
}

pub fn swrl(a1:&f32x4, a2:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let b_xwyz = b.xwyz();
  let b_xzwy = b.xzwy();
  let b_yxxx = b.yxxx();
  let b_xxxx = b.xxxx();
  let mut tmp = b * b;
  tmp = tmp + b_yxxx * b_yxxx;
  let b_tmp = b.zwyz();
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = b.wzwy();
  tmp2 += b_tmp * b_tmp;
  tmp -= flip_signs(&tmp2, [true, false, false, false].into());
  let scale = &[0.0, 2.0, 2.0, 2.0].into();
  let tmp2 = (b_xxxx * b_xwyz + b * b_xzwy) * scale;
  let tmp3 = (b * b_xwyz - b_xxxx * b_xzwy) * scale;
  (tmp * a1 + tmp2 * a1.xzwy() + tmp3 * a1.xwyz(),
   tmp * a2 + tmp2 * a2.xzwy() + tmp3 * a2.xwyz())
}

// swmm<false, false, false>
pub fn swrb(a:&f32x4,b:&f32x4)->f32x4 {
  let b_xwyz = b.xwyz();
  let b_xzwy = b.xzwy();
  let b_yxxx = b.yxxx();
  let b_yxxx_2 = b_yxxx * b_yxxx;

  let mut tmp = b * b;
  tmp += b_yxxx_2;
  let b_tmp = b.zwyz();
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = b.wzwy();
  tmp2 += b_tmp * b_tmp;
  tmp -= f32x4_xor(&tmp2, &[-0.0, 0.0, 0.0, 0.0].into());

  let b_xxxx = b.xxxx();
  let scale:f32x4 = [0.0, 2.0, 2.0, 2.0].into();
  let mut tmp2 = b_xxxx * b_xwyz;
  tmp2 += b * b_xzwy;
  tmp2 *= scale;

  let mut tmp3 = b * b_xwyz;
  tmp3 -= b_xxxx * b_xzwy;
  tmp3 *= &scale;

  let a_xzwy = a.xzwy();
  let a_xwyz = a.xwyz();

  let mut out = tmp * a;
  out += tmp2 * a_xzwy;
  out += tmp3 * a_xwyz;

  out
}

// Apply a translator to a plane.
// Assumes e0123 component of p2 is exactly 0
// p0: (e0, e1, e2, e3)
// p2: (e0123, e01, e02, e03)
// b * a * ~b
// The low component of p2 is expected to be the scalar component instead
pub fn sw02(a:&f32x4, b:&f32x4)->f32x4 {
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
  let tmp = hi_dp(a, b);
  let mut inv_b = rcp_nr1(b);
  // 2 / b0
  inv_b = add_ss(&inv_b, &inv_b);
  inv_b = swizzle!(inv_b.clone(), f32x4::splat(0.0), [First(0),Second(1),Second(2),Second(3)]); // TODO faster?
  a + mul_ss(&tmp, &inv_b)
}

// Apply a translator to a point.
// Assumes e0123 component of p2 is exactly 0
// p2: (e0123, e01, e02, e03)
// p3: (e123, e032, e013, e021)
// b * a * ~b
pub fn sw32(a:&f32x4, b:&f32x4)->f32x4 {
  // a0 e123 +
  // (a1 - 2 a0 b1) e032 +
  // (a2 - 2 a0 b2) e013 +
  // (a3 - 2 a0 b3) e021
  a + a.xxxx() * b * f32x4::from_array([0.0, -2.0, -2.0, -2.0])
}

// Apply a translator to a line
// a := p1 input
// d := p2 input
// c := p2 translator
// out points to the start address of a line (p1, p2)
pub fn swl2(a:&f32x4, d:&f32x4, c:&f32x4)->(f32x4, f32x4) {
  // a0 + a1 e23 + a2 e31 + a3 e12 +
  //
  // (2a0 c0 + d0) e0123 +
  // (2(a2 c3 - a3 c2 - a1 c0) + d1) e01 +
  // (2(a3 c1 - a1 c3 - a2 c0) + d2) e02 +
  // (2(a1 c2 - a2 c1 - a3 c0) + d3) e03
  let mut p2_out = a.xzwy() * c.xwyz();
  // Add and subtract the same quantity in the low component to produce a cancellation
  p2_out -= a.xwyz() * c.xzwy();
  p2_out -= flip_signs(&(a * c.xxxx()), mask32x4::from_array([true, false, false, false]));
  (a.clone(), p2_out + p2_out + d)
}

pub fn sw312(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  // <const N:bool,const F:bool>
  // for point: false, true
  // todo: add count param, support direction (variadic)
  let two = f32x4::from_array([0.0, 2.0, 2.0, 2.0]);
  let b_xxxx = b.xxxx();
  let b_xwyz = b.xwyz();
  let b_xzwy = b.xzwy();

  let tmp1 = (     b * b_xwyz - b_xxxx * b_xzwy) * two;
  let tmp2 = (b_xxxx * b_xwyz + b_xzwy * b) * two;

  let mut tmp3 = b * b;
  let mut b_tmp = b.yxxx();
  tmp3 += b_tmp * &b_tmp;
  b_tmp = b.zwyz();

  let mut tmp4 = &b_tmp * &b_tmp;
  b_tmp = b.wzwy();
  tmp4 += &b_tmp * &b_tmp;
  tmp3 -= flip_signs(&tmp4, mask32x4::from_array([true, false, false, false]));

  tmp4 = b_xzwy * c.xwyz();
  tmp4 -= b_xxxx * c;
  tmp4 -= b_xwyz * c.xzwy();
  tmp4 -= b * c.xxxx();

  tmp4 = tmp4 * two;

  let mut p = tmp1 * a.xwyz();
  p += tmp2 * a.xzwy();
  p += tmp3 * a;

  p + tmp4 * a.xxxx()
}

// Conjugate origin with motor. Unlike other operations the motor MUST be
// normalized prior to usage, b is the rotor component (p1) c is the
// translator component (p2)
pub fn swo12(b:&f32x4, c:&f32x4)->f32x4 {
  //  (b0^2 + b1^2 + b2^2 + b3^2) e123 +
  // 2(b2 c3 - b1 c0 - b0 c1 - b3 c2) e032 +
  // 2(b3 c1 - b2 c0 - b0 c2 - b1 c3) e013 +
  // 2(b1 c2 - b3 c0 - b0 c3 - b2 c1) e021
  let mut tmp:f32x4 = b * c.xxxx();
  tmp += b.xxxx() * c;
  tmp += b.xwyz() * c.xzwy();
  tmp = (b.xzwy() * c.xwyz()) - tmp;
  // b0^2 + b1^2 + b2^2 + b3^2 assumed to equal 1
  // Set the low component to unity
  tmp * <f32x4>::from([0.0, 2.0, 2.0, 2.0]) + <f32x4>::from([1.0, 0.0, 0.0, 0.0])
}
