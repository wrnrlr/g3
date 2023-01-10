
use std::simd::{f32x4};

// a + b is a general bivector but it is most likely *non-simple* meaning
// that it is neither purely real nor purely ideal.
// Exponentiates the bivector and returns the motor defined by partitions 1
// and 2.
pub(crate) fn exp(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
  // The exponential map produces a continuous group of rotations about an
  // axis. We'd *like* to evaluate the exp(a + b) as exp(a)exp(b) but we
  // cannot do that in general because a and b do not commute (consider
  // the differences between the Taylor expansion of exp(ab) and
  // exp(a)exp(b)).

  // Check if the bivector we're exponentiating is ideal

  if *a == f32x4::splat(0.0) {
    // When exponentiating an ideal line, the terms past the linear
    // term in the Taylor series expansion vanishes
    return ([1.0, 0.0, 0.0, 0.0].into(),b.clone());
  }

  // First, we need to decompose the bivector into the sum of two
  // commutative bivectors (the product of these two parts will be a
  // scalar multiple of the pseudoscalar; see "Bivector times its ideal
  // axis and vice versa in demo.klein"). To do this, we compute the
  // squared norm of the bivector:
  //
  // NOTE: a sign flip is introduced since the square of a Euclidean
  // line is negative
  //
  // (a1^2 + a2^2 + a3^2) - 2(a1 b1 + a2 b2 + a3 b3) e0123

  // Broadcast dot(a, a) ignoring the scalar component to all components
  // of a2
  let a2 = &hi_dp_bc(a, a);
  let ab = &hi_dp_bc(a, b);

  // Next, we need the sqrt of that quantity. Since e0123 squares to 0,
  // this has a closed form solution.
  //
  // sqrt(a1^2 + a2^2 + a3^2)
  //  - (a1 b1 + a2 b2 + a3 b3) / sqrt(a1^2 + a2^2 + a3^2) e0123
  //
  // (relabeling) = u + vI
  //
  // (square the above quantity yourself to quickly verify the claim)
  // Maximum relative error < 1.5*2e-12
  let a2_sqrt_rcp = &rsqrt_nr1(a2);
  let u:f32x4 = a2 * a2_sqrt_rcp;
  // Don't forget the minus later!
  let minus_v = ab * a2_sqrt_rcp;

  // Last, we need the reciprocal of the norm to compute the normalized
  // bivector.
  //
  // 1 / sqrt(a1^2 + a2^2 + a3^2) + (a1 b1 + a2 b2 + a3 b3) / (a1^2 + a2^2 + a3^2)^(3/2) e0123
  //
  // The original bivector * the inverse norm gives us a normalized
  // bivector.
  let norm_real:&f32x4  = &(a * a2_sqrt_rcp);
  let mut norm_ideal = b * a2_sqrt_rcp;
  // The real part of the bivector also interacts with the pseudoscalar to
  // produce a portion of the normalized ideal part
  // e12 e0123 = -e03, e31 e0123 = -e02, e23 e0123 = -e01
  // Notice how the products above actually commute
  norm_ideal = norm_ideal - a * ab * a2_sqrt_rcp * rcp_nr1(a2);

  // The norm * our normalized bivector is the original bivector (a + b).
  // Thus, we have:
  //
  // (u + vI)n = u n + v n e0123
  //
  // Note that n and n e0123 are perpendicular (n e0123 lies on the ideal
  // plane, and all ideal components of n are extinguished after
  // polarization). As a result, we can now decompose the exponential.
  //
  // e^(u n + v n e0123) = e^(u n) e^(v n e0123) =
  // (cosu + sinu n) * (1 + v n e0123) =
  // cosu + sinu n + v n cosu e0123 + v sinu n^2 e0123 =
  // cosu + sinu n + v n cosu e0123 - v sinu e0123
  //
  // where we've used the fact that n is normalized and squares to -1.
  let uv_0 = u[0];
  // Note the v here corresponds to minus_v
  let uv_1 = minus_v[0];

  let sincosu_0 = uv_0.sin();
  let sincosu_1:f32 = uv_0.cos();

  let sinu = f32x4::splat(sincosu_0);
  let tmp:f32x4 = [sincosu_1, 0.0, 0.0, 0.0].into();
  let p1_out:f32x4  = tmp + (sinu * norm_real.clone());

  // The second partition has contributions from both the real and ideal
  // parts.
  let cosu:f32x4 = [0.0, sincosu_1, sincosu_1, sincosu_1].into();
  let minus_vcosu = minus_v * cosu;
  let mut p2_out = sinu * norm_ideal;
  p2_out = p2_out + minus_vcosu * norm_real;
  let minus_vsinu = uv_1 * sincosu_0;
  let tmp:f32x4 = [minus_vsinu, 0.0, 0.0, 0.0].into();
  p2_out = tmp + p2_out;
  return (p1_out,p2_out);
}

pub(crate) fn logarithm(p1:&f32x4, p2:&f32x4) ->(f32x4, f32x4) {
  // The logarithm follows from the derivation of the exponential. Working
  // backwards, we ended up computing the exponential like so:
  //
  // cosu + sinu n + v n cosu e0123 - v sinu e0123 =
  // (cosu - v sinu e0123) + (sinu + v cosu e0123) n
  //
  // where n is the normalized bivector. If we compute the norm, that will
  // allow us to match it to sinu + vcosu e0123, which will then allow us
  // to deduce u and v.

  // The first thing we need to do is extract only the bivector components
  // from the motor.
  let bv_mask:f32x4 = [0.0, 1.0, 1.0, 1.0].into();
  let a = bv_mask * p1;

  // Early out if we're taking the log of a motor without any rotation
  if a == f32x4::splat(0.0) { return (a, p2.clone()); }

  let b = bv_mask * p2;

  // Next, we need to compute the norm as in the exponential.
  let a2 = &hi_dp_bc(&a, &a);
  let ab = hi_dp_bc(&a, &b);
  let a2_sqrt_rcp = &rsqrt_nr1(a2);
  let s = a2 * a2_sqrt_rcp;
  let minus_t = ab * a2_sqrt_rcp;
  // s + t e0123 is the norm of our bivector.

  // Store the scalar component
  let p = p1[0];

  // Store the pseudoscalar component
  let q = p2[0];

  let s_scalar = s[0];
  let t_scalar = minus_t[0] * -1.0;
  // p = cosu
  // q = -v sinu
  // s_scalar = sinu
  // t_scalar = v cosu

  let p_zero = p.abs() < 0.000_001;
  let u = if p_zero { (-q).atan2(t_scalar) } else { s_scalar.atan2(p) };
  let v = if p_zero { -q / s_scalar } else { t_scalar / p };

  // Now, (u + v e0123) * n when exponentiated will give us the motor, so
  // (u + v e0123) * n is the logarithm. To proceed, we need to compute
  // the normalized bivector.
  let norm_real = a * a2_sqrt_rcp;
  let mut norm_ideal = b * a2_sqrt_rcp;
  norm_ideal -= &a * &ab * a2_sqrt_rcp * rcp_nr1(a2);

  let uvec = f32x4::splat(u);
  let p1_out = uvec * norm_real;
  let p2_out = (uvec * norm_ideal) - (f32x4::splat(v) * norm_real);
  (p1_out, p2_out)
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

use std::simd::{*, simd_swizzle as swizzle, Which::{First, Second}, StdFloat};

// Workaround for to_bits issue, TODO remove if fixed
pub fn to_bits(a:&f32x4)->u32x4 { unsafe { std::mem::transmute::<f32x4, u32x4>((*a).clone()) } }
fn rsqrt(a:&f32x4)->f32x4 { f32x4::splat(1.0) / a.sqrt() } // TODO fast rsqrt...
#[inline] fn rcp(a:&f32x4)->f32x4 { f32x4::splat(1.0) / a }
#[inline] pub fn f32x4_xor(a:&f32x4,b:&f32x4)->f32x4 {
  f32x4::from_bits(to_bits(&a) ^ to_bits(&b))
}
#[inline] pub fn refined_reciprocal(s:f32)->f32x4 { rcp_nr1(&f32x4::splat(s)) }
#[inline] pub fn sqrt_nr1(a:&f32x4)->f32x4 { a * rsqrt_nr1(&a) } // TODO either write faster rsqrt_nr1, or derive sqrt_nr1 yourself...

// Reciprocal sqrt with an additional single Newton-Raphson refinement.
// f(x) = 1/x^2 - a
// f'(x) = -1/(2x^(3/2))
// Let x_n be the estimate, and x_{n+1} be the refinement
// x_{n+1} = x_n - f(x)/f'(x)
//         = 0.5 * x_n * (3 - a x_n^2)
pub fn rsqrt_nr1(a:&f32x4)->f32x4 {
  // TODO find portable version of _mm_rsqrt_ps in core_simd
  // From Intel optimization manual: expected performance is ~5.2x
  // baseline (sqrtps + divps) with ~22 bits of accuracy
  let xn = &rsqrt(&a);
  let axn2 = xn * xn * a;
  let xn3 = f32x4::splat(3.0) - axn2;
  f32x4::splat(0.5) * xn * xn3
}

// Reciprocal with an additional single Newton-Raphson refinement
// f(x) = 1/x - a
// f'(x) = -1/x^2
// x_{n+1} = x_n - f(x)/f'(x)
//         = 2x_n - a x_n^2 = x_n (2 - a x_n)
#[inline] pub fn rcp_nr1(a:&f32x4)->f32x4 {
  let xn = &rcp(a);
  xn * (f32x4::splat(2.0) - a * xn)
}

pub fn hi_dp(a:&f32x4, b:&f32x4)->f32x4 {
  let ab = a*b;
  swizzle!((ab.yyww() + ab + ab.xxyy()).zwzw(), f32x4::splat(0.0), [First(0),Second(1),Second(2),Second(3)]) // TODO make faster???
}

pub fn hi_dp_bc(a:&f32x4, b:&f32x4)->f32x4 {
  let ab = a*b;
  (ab.yyww() + ab + ab.xxyy()).zzzz()
}

pub fn hi_dp_ss(a:&f32x4, b:&f32x4)->f32x4 {
  let mut ab = a * b;
  let hi = &ab.yyww();
  let sum = hi + ab;
  ab = sum + ab.xxyy();
  ab.zwzw()
}

pub fn dp(a:&f32x4, b:&f32x4)->f32x4 {
  let mut ab = a * b;
  let hi = &ab.yyww();

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  ab = hi + ab;
  ab[0] += b2b3a2a3(hi, &ab)[0];
  let true_falses = mask32x4::from_array([true, false, false, false]);
  true_falses.select(ab, f32x4::splat(0.0))
}

pub fn dp_bc(a:&f32x4, b:&f32x4)->f32x4 {
  let mut ab = a * b;
  let hi = &ab.yyww();

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  ab = hi + ab;
  ab = add_ss(&ab, &b2b3a2a3(hi, &ab));
  ab.xxxx()
}

#[inline] pub fn zero_first(a:f32x4)->f32x4 { swizzle!(a, f32x4::splat(0.0), [Second(0), First(1), First(2), First(3)]) } // TODO find a faster way
#[inline] pub fn f32x4_and(a:f32x4,b:f32x4)->f32x4 { f32x4::from_bits(a.to_bits() & b.to_bits()) }
#[inline] pub fn flip_signs(x:&f32x4, mask:mask32x4)->f32x4 { mask.select(-x.clone(), x.clone())}
pub fn add_ss(a:&f32x4,b:&f32x4)->f32x4 { swizzle!(a + b, a.clone(), [First(0), Second(1), Second(2), Second(3)]) }
#[inline] pub fn sub_ss(a:&f32x4,b:f32x4)->f32x4 { swizzle!(a - b, a.clone(), [First(0), Second(1), Second(2), Second(3)]) }
pub fn mul_ss(a:&f32x4,b:&f32x4)->f32x4 { swizzle!(a * b, a.clone(), [First(0), Second(1), Second(2), Second(3)]) }
#[inline] pub fn b2b3a2a3(a:&f32x4,b:&f32x4)->f32x4 { swizzle!(a.clone(), b.clone(), [Second(2),Second(3),First(2),First(3)]) }
#[inline] pub fn b0a1a2a3(a:&f32x4,b:&f32x4)->f32x4 { swizzle!(a.clone(), b.clone(), [Second(0),First(1),First(2),First(3)]) }

pub trait Shuffle:Clone {
  fn xxxx(&self)->Self;
  fn xyxy(&self)->Self;
  fn xxyy(&self)->Self;
  fn xxzz(&self)->Self;
  fn xzxx(&self)->Self;
  fn xzwy(&self)->Self;
  fn xwyz(&self)->Self;
  fn xwzy(&self)->Self;
  fn yxxx(&self)->Self;
  fn yxwx(&self)->Self;
  fn yyyy(&self)->Self;
  fn yyzw(&self)->Self;
  fn yywz(&self)->Self;
  fn yyww(&self)->Self;
  fn yzxx(&self)->Self;
  fn yzyz(&self)->Self;
  fn yzyw(&self)->Self;
  fn yzwx(&self)->Self;
  fn yzwy(&self)->Self;
  fn ywyx(&self)->Self;
  fn ywyz(&self)->Self;
  fn zxxx(&self)->Self;
  fn zxzx(&self)->Self;
  fn zyzw(&self)->Self;
  fn zzzz(&self)->Self;
  fn zzwx(&self)->Self;
  fn zzwy(&self)->Self;
  fn zzww(&self)->Self;
  fn zwxx(&self)->Self;
  fn zwxy(&self)->Self;
  fn zwyx(&self)->Self;
  fn zwyz(&self)->Self;
  fn zwzw(&self)->Self;
  fn zwwy(&self)->Self;
  fn wxxx(&self)->Self;
  fn wyzx(&self)->Self;
  fn wyzw(&self)->Self;
  fn wywx(&self)->Self;
  fn wywz(&self)->Self;
  fn wzyz(&self)->Self;
  fn wzyw(&self)->Self;
  fn wzwy(&self)->Self;
  fn wwxx(&self)->Self;
  fn wwyz(&self)->Self;
  fn wwww(&self)->Self;
}

// impl<T:Clone+StdFloat> Shuffle for T {}
impl Shuffle for f32x4 {
  #[inline] fn xxxx(&self)->Self { swizzle!(self.clone(), [0,0,0,0]) }
  #[inline] fn xyxy(&self)->Self { swizzle!(self.clone(), [0,1,0,1]) }
  #[inline] fn xxyy(&self)->Self { swizzle!(self.clone(), [0,0,1,1]) }
  #[inline] fn xxzz(&self)->Self { swizzle!(self.clone(), [0,0,2,2]) }
  #[inline] fn xzxx(&self)->Self { swizzle!(self.clone(), [0,2,0,0]) }
  #[inline] fn xzwy(&self)->Self { swizzle!(self.clone(), [0,2,3,1]) }
  #[inline] fn xwyz(&self)->Self { swizzle!(self.clone(), [0,3,1,2]) }
  #[inline] fn xwzy(&self)->Self { swizzle!(self.clone(), [0,3,2,1]) }
  #[inline] fn yxxx(&self)->Self { swizzle!(self.clone(), [1,0,0,0]) }
  #[inline] fn yxwx(&self)->Self { swizzle!(self.clone(), [1,0,3,0]) }
  #[inline] fn yyyy(&self)->Self { swizzle!(self.clone(), [1,1,1,1]) }
  #[inline] fn yyzw(&self)->Self { swizzle!(self.clone(), [1,1,2,3]) }
  #[inline] fn yywz(&self)->Self { swizzle!(self.clone(), [1,1,3,2]) }
  #[inline] fn yyww(&self)->Self { swizzle!(self.clone(), [1,1,3,3]) }
  #[inline] fn yzxx(&self)->Self { swizzle!(self.clone(), [1,2,0,0]) }
  #[inline] fn yzyz(&self)->Self { swizzle!(self.clone(), [1,2,1,2]) }
  #[inline] fn yzyw(&self)->Self { swizzle!(self.clone(), [1,2,1,3]) }
  #[inline] fn yzwx(&self)->Self { swizzle!(self.clone(), [1,2,3,0]) }
  #[inline] fn yzwy(&self)->Self { swizzle!(self.clone(), [1,2,3,1]) }
  #[inline] fn ywyx(&self)->Self { swizzle!(self.clone(), [1,3,1,0]) }
  #[inline] fn ywyz(&self)->Self { swizzle!(self.clone(), [1,3,1,2]) }
  #[inline] fn zxxx(&self)->Self { swizzle!(self.clone(), [2,0,0,0]) }
  #[inline] fn zxzx(&self)->Self { swizzle!(self.clone(), [2,0,2,0]) }
  #[inline] fn zyzw(&self)->Self { swizzle!(self.clone(), [2,1,2,3]) }
  #[inline] fn zzzz(&self)->Self { swizzle!(self.clone(), [2,2,2,2]) }
  #[inline] fn zzwx(&self)->Self { swizzle!(self.clone(), [2,2,3,0]) }
  #[inline] fn zzwy(&self)->Self { swizzle!(self.clone(), [2,2,3,1]) }
  #[inline] fn zzww(&self)->Self { swizzle!(self.clone(), [2,2,3,3]) }
  #[inline] fn zwxx(&self)->Self { swizzle!(self.clone(), [2,3,0,0]) }
  #[inline] fn zwxy(&self)->Self { swizzle!(self.clone(), [2,3,0,1]) }
  #[inline] fn zwyx(&self)->Self { swizzle!(self.clone(), [2,3,1,0]) }
  #[inline] fn zwyz(&self)->Self { swizzle!(self.clone(), [2,3,1,2]) }
  #[inline] fn zwzw(&self)->Self { swizzle!(self.clone(), [2,3,2,3]) }
  #[inline] fn zwwy(&self)->Self { swizzle!(self.clone(), [2,3,3,1]) }
  #[inline] fn wxxx(&self)->Self { swizzle!(self.clone(), [3,0,0,0]) }
  #[inline] fn wyzx(&self)->Self { swizzle!(self.clone(), [3,1,2,0]) }
  #[inline] fn wyzw(&self)->Self { swizzle!(self.clone(), [3,1,2,3]) }
  #[inline] fn wywx(&self)->Self { swizzle!(self.clone(), [3,1,3,0]) }
  #[inline] fn wywz(&self)->Self { swizzle!(self.clone(), [3,1,3,2]) }
  #[inline] fn wzyz(&self)->Self { swizzle!(self.clone(), [3,2,1,2]) }
  #[inline] fn wzyw(&self)->Self { swizzle!(self.clone(), [3,2,1,3]) }
  #[inline] fn wzwy(&self)->Self { swizzle!(self.clone(), [3,2,3,1]) }
  #[inline] fn wwxx(&self)->Self { swizzle!(self.clone(), [3,3,0,0]) }
  #[inline] fn wwyz(&self)->Self { swizzle!(self.clone(), [3,3,1,2]) }
  #[inline] fn wwww(&self)->Self { swizzle!(self.clone(), [3,3,3,3]) }
}

#[inline] pub fn bits_wwww(a:u32x4)->u32x4 { swizzle!(a, [0,0,0,0]) }

#[cfg(test)]
pub fn approx_eq(result: [f32; 4], expected: [f32; 4]) {
  const EPSILON: f32 = 0.02;
  assert_eq!(result.len(), expected.len());
  for (i, a) in result.iter().enumerate() {
    let b = expected[i];
    assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{simd::{f32x4}};

  #[test] fn hi_dp_ss_test() {
    let a:f32x4 = [1.0, 2.0, 3.0, 5.0].into();
    let b = [-4.0, -3.0, -2.0, -1.0].into();
    assert_eq!(hi_dp_ss(&a, &b), [-17.0, -16.0, -17.0, -16.0].into());
  }

  #[test] fn add_first() {
    let a:f32x4 = [2.0, 2.0, 3.0, 4.0].into();
    assert_eq!(add_ss(&a, &a), [4.0, 2.0, 3.0, 4.0].into());
  }

  #[test] fn rcp_test() {
    let a:f32x4 = [2.0, 4.0, 8.0, 10.0].into();
    approx_eq(rcp(&a).into(), [0.5, 0.25, 0.125, 0.1].into());
  }

  #[test] fn multiply_first() {
    let a = <f32x4>::from([2.0, 2.0, 3.0, 4.0]);
    assert_eq!(mul_ss(&a, &a), [4.0, 2.0, 3.0, 4.0].into());
  }

  #[test] fn inverse_sqrt() {
    let a = <f32x4>::from([4.0, 9.0, 16.0, 25.0]);
    assert_eq!(a.sqrt(), [2.0, 3.0, 4.0, 5.0].into());
    assert_eq!(f32x4::splat(1.0)/a.sqrt(), [1.0/2.0, 1.0/3.0, 1.0/4.0, 1.0/5.0].into());
  }

  #[test] fn rcp_nr1_test() {
    let a = <f32x4>::from([1.0, 2.0, 3.0, 4.0]);
    let b = rcp_nr1(&a);
    approx_eq(*b.as_array(), [1.0, 0.5, 1.0/3.0, 0.25]);
  }

  #[test] #[ignore] fn sqrt_nr1_test() {}

  #[test] #[ignore] fn rsqrt_nr1_test() {}

  #[test] fn test_f32_sign_flipping() {
    let v1 = f32x4::from_array([1.0,2.0,3.0,4.0]);
    let expected = f32x4::from_array([-1.0,-2.0,-3.0,4.0]);
    assert_eq!(f32x4_xor(&v1, &f32x4::from_array([-0.0,-0.0,-0.0,0.0])), expected);
    assert_eq!(flip_signs(&v1, [true,true,true,false].into()), expected);
  }
}
