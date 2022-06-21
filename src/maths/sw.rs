use std::simd::{f32x4,mask32x4,simd_swizzle as swizzle,Which::{First,Second}};
use crate::maths::util::{add_ss, flip_signs, f32x4_xor, hi_dp, rcp_nr1, shuffle_xxxx, shuffle_wwyz, shuffle_wyzw, shuffle_yyzw, shuffle_yyww, shuffle_yzwy, shuffle_zwyz, shuffle_zyzw, shuffle_ywyz, shuffle_wzwy, shuffle_xzwy, shuffle_zzwy, shuffle_xwyz, shuffle_yxxx, shuffle_zxxx, shuffle_wxxx, mul_ss, shuffle_zwwy, shuffle_wzyz};

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
  let a_zzwy = &shuffle_zzwy(a);
  let a_wwyz = &shuffle_wwyz(a);

  // Left block
  let mut tmp = a_zzwy * shuffle_zzwy(b);
  tmp += a_wwyz * shuffle_wwyz(b);

  let a1 = &shuffle_yyww(a);
  let b1 = &shuffle_yyww(b);
  tmp = add_ss(&tmp, &mul_ss(a1, b1));
  tmp *= a + a;

  // Right block
  let a_yyzw = &shuffle_yyzw(a);
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

  let a_zyzw = &shuffle_zyzw(a);
  let a_ywyz = &shuffle_ywyz(a);
  let a_wzwy = &shuffle_wzwy(a);
  let b_xzwy = &shuffle_xzwy(b);

  let two_zero = [0.0, 2.0, 2.0, 2.0].into();
  let mut p1 = a * b;
  p1 += a_wzwy * b_xzwy;
  p1 *= a_ywyz * two_zero;

  let mut tmp = a_zyzw * a_zyzw;
  tmp += a_wzwy * a_wzwy;
  tmp = f32x4_xor(&tmp, &[-0.0, 0.0, 0.0, 0.0].into());
  tmp = (a_ywyz * a_ywyz) - tmp;
  tmp = shuffle_xwyz(b) * tmp;

  let p1 = shuffle_xzwy(&(p1 + tmp));

  let mut p2 = a_zyzw * b_xzwy;
  p2 = p2 - a_wzwy * b;
  p2 = p2 * shuffle_xxxx(a) * two_zero;
  p2 = shuffle_xzwy(&p2);

  (p1,p2)
}

pub fn sw20(a:&f32x4,b:&f32x4)->f32x4 {
  //                       -b0(a1^2 + a2^2 + a3^2) e0123 +
  // (-2a3(a1 b1 + a2 b2) + b3(a1^2 + a2^2 - a3^2)) e03
  // (-2a1(a2 b2 + a3 b3) + b1(a2^2 + a3^2 - a1^2)) e01 +
  // (-2a2(a3 b3 + a1 b1) + b2(a3^2 + a1^2 - a2^2)) e02 +
  let a_zzwy = &shuffle_zzwy(a);
  let a_wwyz = &shuffle_wwyz(a);

  let mut p2 = a * b;
  p2 += a_zzwy * shuffle_xzwy(b);
  p2 *= a_wwyz * [0.0, -2.0, -2.0, -2.0].into();

  let a_yyzw = &shuffle_yyzw(a);
  let mut tmp = a_yyzw * a_yyzw;
  tmp = f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(tmp + a_zzwy * a_zzwy));
  tmp -= a_wwyz * a_wwyz;
  p2 += tmp * shuffle_xwyz(b);
  shuffle_xzwy(&p2)
}

// reflect point through plane
pub fn sw30(a:&f32x4, b:&f32x4) ->f32x4 {
  //                                b0(a1^2 + a2^2 + a3^2)  e123 +
  // (-2a1(a0 b0 + a3 b3 + a2 b2) + b1(a2^2 + a3^2 - a1^2)) e032 +
  // (-2a2(a0 b0 + a1 b1 + a3 b3) + b2(a3^2 + a1^2 - a2^2)) e013 +
  // (-2a3(a0 b0 + a2 b2 + a1 b1) + b3(a1^2 + a2^2 - a3^2)) e021

  let a_zwyz = &shuffle_zwyz(a); // a2, a3, a1, a2
  let a_yzwy = &shuffle_yzwy(a); // a1, a2, a3, a1
  let a_wyzw = &shuffle_wyzw(a); // a3, a1, a2, a3

  //     a0 b0              |      a0 b0              |      a0 b0              |      a0 b0
  let mut p3_out = shuffle_xxxx(a) * shuffle_xxxx(b);
  //     a0 b0+a2 b0        |      a0 b0+a3 b3        |      a0 b0+a1 b1        |      a0 b0+a3 b2
  p3_out += a_zwyz * shuffle_xwyz(b);
  //     a0 b0+a2 b0+a1 b0  |      a0 b0+a3 b3+a2 b2  |      a0 b0+a1 b0+a3 b3  |      a0 b0+a3 b2+a1 b1
  p3_out += a_yzwy * shuffle_xzwy(b);
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
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);
  let b_xxxx = &shuffle_xxxx(b);

  let mut tmp1 = shuffle_zxxx(b) * shuffle_zwyz(b);
  tmp1 += shuffle_yzwy(b) * shuffle_yyzw(b);
  tmp1 *= dc_scale;

  let mut tmp2 = b * b_xwyz;
  tmp2 -= flip_signs(&(shuffle_wxxx(b) * shuffle_wzwy(b)), &[true,false,false,false].into());
  tmp2 *= dc_scale;

  let mut tmp3 = b * b;
  tmp3 -= b_xwyz * b_xwyz;
  tmp3 += b_xxxx * b_xxxx;
  tmp3 -= b_xzwy * b_xzwy;

  let mut out:f32x4;

  out = tmp1 * shuffle_xzwy(a);
  out += tmp2 * shuffle_xwyz(a);
  out += tmp3 * a;

  out
}

// motor(plane), motor(point)
pub fn sw012(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  // Double-cover scale
  let dc_scale = f32x4::from_array([1.0,2.0,2.0,2.0]);
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);
  let b_xxxx = &shuffle_xxxx(b);

  let mut tmp1 = shuffle_zxxx(b) * shuffle_zwyz(b);
  tmp1 += shuffle_yzwy(b) * shuffle_yyzw(b);
  // Scale later with (a0, a2, a3, a1)
  tmp1 *= dc_scale;

  let mut tmp2 = b * b_xwyz;
  tmp2 -= f32x4_xor(&[-0.0, 0.0, 0.0, 0.0].into(), &(shuffle_wxxx(b) * shuffle_wzwy(b)));
  // Scale later with (a0, a3, a1, a2)
  tmp2 *= dc_scale;

  // Alternately add and subtract to improve low component stability
  let mut tmp3 = b * b;
  tmp3 -= b_xwyz * b_xwyz;
  tmp3 += b_xxxx * b_xxxx;
  tmp3 -= b_xzwy * b_xzwy;

  let mut tmp4 = b_xxxx * c;
  tmp4 += b_xzwy * shuffle_xwyz(c);
  tmp4 += b * shuffle_xxxx(c);

  // NOTE: The high component of tmp4 is meaningless here
  tmp4 -= b_xwyz * shuffle_xzwy(c);
  tmp4 *= dc_scale;

  let mut p = tmp1 * shuffle_xzwy(a); // TODO a[1]...
  p += tmp2 * shuffle_xwyz(a);
  p += tmp3 * a; // TODO should be a[1]

  let tmp5 = hi_dp(&tmp4, a);
  let out = p + tmp5;
  out
}

// motor(line), swmm<false, true, true>
pub fn swml(a1:&f32x4, a2:&f32x4, b:&f32x4, c:&f32x4)->(f32x4,f32x4) {
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);
  let b_yxxx = &shuffle_yxxx(b);
  let b_yxxx_2 = b_yxxx * b_yxxx;

  let mut tmp = b * b;
  tmp = tmp + b_yxxx_2;
  let b_tmp = &shuffle_zwyz(b);
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = &shuffle_wzwy(b);
  tmp2 += b_tmp * b_tmp;
  tmp -= flip_signs(&tmp2, &[true, false, false, false].into());

  let b_xxxx = &shuffle_xxxx(b);
  let scale = [0.0, 2.0, 2.0, 2.0].into();
  tmp2 = b_xxxx * b_xwyz;
  tmp2 += b * b_xzwy;
  tmp2 = tmp2 * scale;

  let mut tmp3 = b * b_xwyz;
  tmp3 -= b_xxxx * b_xzwy;
  tmp3 = tmp3 * scale;

  let czero = &shuffle_xxxx(c);
  let c_xzwy = &shuffle_xzwy(c);
  let c_xwyz = &shuffle_xwyz(c);

  let mut tmp4 = b * c;
  tmp4 -= b_yxxx * shuffle_yxxx(c);
  tmp4 -= shuffle_zwwy(b) * shuffle_zwwy(c);
  tmp4 -= shuffle_wzyz(b) * shuffle_wzyz(c);
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

  let p1_in_xzwy = &shuffle_xzwy(a1);
  let p1_in_xwyz = &shuffle_xwyz(a1);

  let mut p1_out = tmp * a1;
  p1_out = p1_out + tmp2 * p1_in_xzwy;
  p1_out = p1_out + tmp3 * p1_in_xwyz;

  let mut p2_out = tmp * a2;
  p2_out += tmp2 * shuffle_xzwy(a2);
  p2_out += tmp3 * shuffle_xwyz(a2);

  p2_out += tmp4 * a1;
  p2_out += tmp5 * p1_in_xwyz;
  p2_out += tmp6 * p1_in_xzwy;

  (p1_out, p2_out)
}

pub fn swrl(a1:&f32x4, a2:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);
  let b_yxxx = &shuffle_yxxx(b);
  let b_yxxx_2 = b_yxxx * b_yxxx;

  let mut tmp = b * b;
  tmp = tmp + b_yxxx_2;
  let b_tmp = &shuffle_zwyz(b);
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = &shuffle_wzwy(b);
  tmp2 += b_tmp * b_tmp;
  tmp -= flip_signs(&tmp2, [true, false, false, false].into());

  let b_xxxx = &shuffle_xxxx(b);
  let scale = [0.0, 2.0, 2.0, 2.0].into();
  tmp2 = b_xxxx * b_xwyz;
  tmp2 += b * b_xzwy;
  tmp2 = tmp2 * scale;

  let mut tmp3 = b * b_xwyz;
  tmp3 -= b_xxxx * b_xzwy;
  tmp3 = tmp3 * scale;

  let p1_in_xzwy = shuffle_xzwy(a1);
  let p1_in_xwyz = shuffle_xwyz(a1);

  let mut p1_out = tmp * a1;
  p1_out = p1_out + tmp2 * p1_in_xzwy;
  p1_out = p1_out + tmp3 * p1_in_xwyz;

  let mut p2_out = tmp * a2;
  p2_out += tmp2 * shuffle_xzwy(a2);
  p2_out += tmp3 * shuffle_xwyz(a2);

  (p1_out, p2_out)
}

// swmm<false, false, false>
pub fn swrb(a:&f32x4,b:&f32x4)->f32x4 {
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);
  let b_yxxx = &shuffle_yxxx(b);
  let b_yxxx_2 = b_yxxx * b_yxxx;

  let mut tmp = b * b;
  tmp += b_yxxx_2;
  let b_tmp = &shuffle_zwyz(b);
  let mut tmp2 = b_tmp * b_tmp;
  let b_tmp = &shuffle_wzwy(b);
  tmp2 += b_tmp * b_tmp;
  tmp -= f32x4_xor(&tmp2, &[-0.0, 0.0, 0.0, 0.0].into());

  let b_xxxx = &shuffle_xxxx(b);
  let scale = [0.0, 2.0, 2.0, 2.0].into();
  let mut tmp2 = b_xxxx * b_xwyz;
  tmp2 += b * b_xzwy;
  tmp2 *= scale;

  let mut tmp3 = b * b_xwyz;
  tmp3 -= b_xxxx * b_xzwy;
  tmp3 *= scale;

  let a_xzwy = shuffle_xzwy(a);
  let a_xwyz = shuffle_xwyz(a);

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
  let mut tmp = hi_dp(a, b);
  let mut inv_b = &rcp_nr1(b);
  // 2 / b0
  inv_b = &add_ss(inv_b, inv_b);
  inv_b = swizzle!(&inv_b, f32x4::splat(0.0), [First(0),Second(1),Second(2),Second(3)]); // TODO faster?
  a + mul_ss(&tmp, inv_b)
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
  a + shuffle_xxxx(&a) * b * f32x4::from_array([0.0, -2.0, -2.0, -2.0])
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
  let mut p2_out = shuffle_xzwy(a) * shuffle_xwyz(c);
  // Add and subtract the same quantity in the low component to produce a cancellation
  p2_out -= shuffle_xwyz(a) * shuffle_xzwy(c);
  p2_out -= flip_signs(a * shuffle_xxxx(c), mask32x4::from_array([true, false, false, false]));
  (*a.clone(), p2_out + p2_out + d)
}

pub fn sw312(a:&f32x4, b:&f32x4, c:&f32x4)->f32x4 {
  // <const N:bool,const F:bool>
  // for point: false, true
  // todo: add count param, support direction (variadic)
  let two = f32x4::from_array([0.0, 2.0, 2.0, 2.0]);
  let b_xxxx = &shuffle_xxxx(b);
  let b_xwyz = &shuffle_xwyz(b);
  let b_xzwy = &shuffle_xzwy(b);

  let tmp1 = (     b * b_xwyz - b_xxxx * b_xzwy) * two;
  let tmp2 = (b_xxxx * b_xwyz + b_xzwy * b) * two;

  let mut tmp3 = b * b;
  let mut b_tmp = &shuffle_yxxx(b);
  tmp3 += b_tmp * b_tmp;
  b_tmp = &shuffle_zwyz(b);

  let mut tmp4 = b_tmp * b_tmp;
  b_tmp = &shuffle_wzwy(b);
  tmp4 += b_tmp * b_tmp;
  tmp3 -= flip_signs(tmp4, mask32x4::from_array([true, false, false, false]));

  tmp4 = b_xzwy * shuffle_xwyz(c);
  tmp4 -= b_xxxx * c;
  tmp4 -= b_xwyz * shuffle_xzwy(c);
  tmp4 -= b * shuffle_xxxx(c);

  tmp4 = tmp4 * two;

  let mut p = tmp1 * shuffle_xwyz(a);
  p += tmp2 * shuffle_xzwy(a);
  p += tmp3 * a;

  p + tmp4 * shuffle_xxxx(a)
}

// Conjugate origin with motor. Unlike other operations the motor MUST be
// normalized prior to usage, b is the rotor component (p1) c is the
// translator component (p2)
pub fn swo12(b:&f32x4, c:&f32x4)->f32x4 {
  //  (b0^2 + b1^2 + b2^2 + b3^2) e123 +
  // 2(b2 c3 - b1 c0 - b0 c1 - b3 c2) e032 +
  // 2(b3 c1 - b2 c0 - b0 c2 - b1 c3) e013 +
  // 2(b1 c2 - b3 c0 - b0 c3 - b2 c1) e021
  let mut tmp = b * shuffle_xxxx(c);
  tmp += shuffle_xxxx(b) * c;
  tmp += shuffle_xwyz(b) * shuffle_xzwy(c);
  tmp = (shuffle_xzwy(b) * shuffle_xwyz(c)) - tmp;
  tmp *= [0.0, 2.0, 2.0, 2.0].into();
  // b0^2 + b1^2 + b2^2 + b3^2 assumed to equal 1
  // Set the low component to unity
  tmp + [1.0, 0.0, 0.0, 0.0].into()
}
