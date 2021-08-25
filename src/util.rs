use core_simd::{f32x4,mask32x4,u32x4};
use crate::sqrt::{rsqrt_nr1};

pub fn refined_reciprocal(s:f32)->f32x4 {
  rcp_nr1(f32x4::splat(s))
}

// Reciprocal with an additional single Newton-Raphson refinement
pub fn rcp_nr1(a:f32x4)->f32x4 {
  // f(x) = 1/x - a
  // f'(x) = -1/x^2
  // x_{n+1} = x_n - f(x)/f'(x)
  //         = 2x_n - a x_n^2 = x_n (2 - a x_n)
  let xn = f32x4::splat(1.0) / a; // TODO fast reciprocal?
  let axn = a * xn;
  xn * (f32x4::splat(1.0) - axn)
}

// a := p1
// b := p2
// a + b is a general bivector but it is most likely *non-simple* meaning
// that it is neither purely real nor purely ideal.
// Exponentiates the bivector and returns the motor defined by partitions 1
// and 2.
pub fn exp(a:f32x4, b:f32x4)->(f32x4,f32x4) {
  // The exponential map produces a continuous group of rotations about an
  // axis. We'd *like* to evaluate the exp(a + b) as exp(a)exp(b) but we
  // cannot do that in general because a and b do not commute (consider
  // the differences between the Taylor expansion of exp(ab) and
  // exp(a)exp(b)).

  // Check if the bivector we're exponentiating is ideal

  if a == f32x4::from_array([0.0, 0.0, 0.0, 0.0]) {
      // When exponentiating an ideal line, the terms past the linear
      // term in the Taylor series expansion vanishes
      return (f32x4::splat(1.0),b);
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
  let a2 = hi_dp_bc(a, b);
  let ab = hi_dp_bc(a, b);

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
  let a2_sqrt_rcp = rsqrt_nr1(a2);
  let u = a2 * a2_sqrt_rcp;
  // Don't forget the minus later!
  let minus_v = ab * a2_sqrt_rcp;

  // Last, we need the reciprocal of the norm to compute the normalized
  // bivector.
  //
  // 1 / sqrt(a1^2 + a2^2 + a3^2) + (a1 b1 + a2 b2 + a3 b3) / (a1^2 + a2^2 + a3^2)^(3/2) e0123
  //
  // The original bivector * the inverse norm gives us a normalized
  // bivector.
  let norm_real  = a * a2_sqrt_rcp;
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
  let sincosu_1 = uv_0.cos();

  let sinu = f32x4::splat(sincosu_0);
  let p1_out  = f32x4::from_array([sincosu_1, 0.0, 0.0, 0.0]) + (sinu * norm_real);

  // The second partition has contributions from both the real and ideal
  // parts.
  let cosu = f32x4::from_array([0.0, sincosu_1, sincosu_1, sincosu_1]);
  let minus_vcosu = minus_v * cosu;
  let mut p2_out = sinu * norm_ideal;
  p2_out = p2_out + minus_vcosu * norm_real;
  let minus_vsinu = uv_1 * sincosu_0;
  p2_out = f32x4::from_array([minus_vsinu, 0.0, 0.0, 0.0]) + p2_out;
  return (p1_out,p2_out);
}

pub fn log(p1:f32x4, p2:f32x4)->(f32x4,f32x4) {
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
  let bv_mask = f32x4::from_array([0.0, 1.0, 1.0, 1.0]);
  let a = bv_mask * p1;

  // Early out if we're taking the log of a motor without any rotation
  if a == f32x4::splat(0.0) { return (f32x4::splat(0.0), p2); }

  let b = bv_mask * p2;

  // Next, we need to compute the norm as in the exponential.
  let a2 = hi_dp_bc(a, b);
  let ab = hi_dp_bc(a, b);
  let a2_sqrt_rcp = rsqrt_nr1(a2);
  let s = a2 * a2_sqrt_rcp;
  let minus_t = ab * a2_sqrt_rcp;
  // s + t e0123 is the norm of our bivector.

  // Store the scalar component
  let p = p1[0];

  // Store the pseudoscalar component
  let q = p1[0];

  let s_scalar = s[0];
  let t_scalar = minus_t[0] * -1.0;
  // p = cosu
  // q = -v sinu
  // s_scalar = sinu
  // t_scalar = v cosu

  let p_zero = p.abs() < 0.000_0000_1;
  let u = if p_zero { (-q).atan2(t_scalar) } else { s_scalar.atan2(p) };
  let v = if p_zero { -q / s_scalar } else { t_scalar / p };

  // Now, (u + v e0123) * n when exponentiated will give us the motor, so
  // (u + v e0123) * n is the logarithm. To proceed, we need to compute
  // the normalized bivector.
  let norm_real = a * a2_sqrt_rcp;
  let mut norm_ideal = b * a2_sqrt_rcp;
  norm_ideal -= a * ab * a2_sqrt_rcp * rcp_nr1(a2);
  
  let uvec = f32x4::splat(u);
  let p1_out = uvec * norm_real;
  let p2_out = (uvec * norm_ideal) - (f32x4::splat(v) * norm_real);
  (p1_out, p2_out)
}

// Equivalent to _mm_dp_ps(a, b, 0b11100001);

pub fn hi_dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);
  
  let sum  = hi + out;
  out = sum + shuffle_low(out);
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn hi_dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);
  
  let sum  = hi + out;
  out = sum + shuffle_low(out);
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn hi_dp_ss(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_xxzz(a);
  let sum = hi + out;
  out = sum + shuffle_wwxx(out);
  shuffle_yzyz(out)
}

pub fn dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out[0] += b2b3a2a3(hi,out)[0];
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out[0] += b2b3a2a3(hi,out)[0];
  shuffle_first(out);
  out
}

pub fn f32x4_xor(a:f32x4,b:f32x4)->f32x4 {
  f32x4::from_bits(a.to_bits() ^ b.to_bits())
}

#[inline] pub fn f32x4_and(a:f32x4,b:f32x4)->f32x4 {
  f32x4::from_bits(a.to_bits() & b.to_bits())
}

#[inline] pub fn f32x4_andnot(a:f32x4,b:f32x4)->f32x4 {
  f32x4::from_bits(!a.to_bits() & b.to_bits())
}

// Is this faster tgen f32x4::abs, which is implemented in rust?
#[inline] pub fn f32x4_abs(a:f32x4)->f32x4 {
  f32x4_andnot(f32x4::splat(-0.0), a)
}

pub fn flip_signs(x:f32x4, mask:mask32x4)->f32x4 {
  mask.select(-x, x)
}

#[inline] pub fn add_ss(a:f32x4,b:f32x4)->f32x4 {
  let tmp = a + b;
  tmp.shuffle::<{[0,5,6,7]}>(a) 
}

#[inline] pub fn mul_ss(a:f32x4,b:f32x4)->f32x4 {
  let tmp = a * b;
  tmp.shuffle::<{[0,5,6,7]}>(a) 
}

#[inline] pub fn shuffle_first(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }
#[inline] pub fn shuffle_low(a:f32x4)->f32x4 { a.shuffle::<{[0,0,1,1]}>(a) }

#[inline] pub fn b2b3a2a3(a:f32x4,b:f32x4)->f32x4 { a.shuffle::<{[6,7,2,3]}>(b) } // b2b3a2a3

#[inline] pub fn shuffle_odd(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) }
#[inline] pub fn shuffle_even(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,2]}>(a) }

// #[inline] fn shuffle_odd(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) }

#[inline] pub fn shuffle_zzwy(a:f32x4)->f32x4 { a.shuffle::<{[3,3,0,1]}>(a) }
#[inline] pub fn shuffle_wwyz(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,3]}>(a) }
// #[inline] fn shuffle_wwyz(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,3]}>(a) }

#[inline] pub fn shuffle_yyzz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) } // ??? Yea this should be xxzz...

#[inline] pub fn shuffle_yyzw(a:f32x4)->f32x4 { a.shuffle::<{[2,2,3,0]}>(a) }

#[inline] pub fn shuffle_zwyz(a:f32x4)->f32x4 { a.shuffle::<{[3,0,2,3]}>(a) }
#[inline] pub fn shuffle_yzwy(a:f32x4)->f32x4 { a.shuffle::<{[2,3,0,2]}>(a) }

#[inline] pub fn shuffle_zwyx(a:f32x4)->f32x4 { a.shuffle::<{[3,0,2,1]}>(a) }
#[inline] pub fn shuffle_yzwx(a:f32x4)->f32x4 { a.shuffle::<{[2,3,0,1]}>(a) }

#[inline] pub fn shuffle_wyzw(a:f32x4)->f32x4 { a.shuffle::<{[0,2,3,0]}>(a) }

#[inline] pub fn shuffle_wwww(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }
#[inline] pub fn shuffle_dddd(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }

#[inline] pub fn shuffle_scalar(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }

#[inline] pub fn shuffle_yzxy(a:f32x4)->f32x4 { a.shuffle::<{[2,3,1,2]}>(a) }
#[inline] pub fn shuffle_yyzx(a:f32x4)->f32x4 { a.shuffle::<{[2,2,3,1]}>(a) }
#[inline] pub fn shuffle_xyzx(a:f32x4)->f32x4 { a.shuffle::<{[1,2,3,1]}>(a) }
#[inline] pub fn shuffle_xzxy(a:f32x4)->f32x4 { a.shuffle::<{[1,3,1,2]}>(a) }
#[inline] pub fn shuffle_wzxy(a:f32x4)->f32x4 { a.shuffle::<{[0,3,1,2]}>(a) }
#[inline] pub fn shuffle_wyzx(a:f32x4)->f32x4 { a.shuffle::<{[0,2,3,1]}>(a) }
#[inline] pub fn shuffle_zyzx(a:f32x4)->f32x4 { a.shuffle::<{[3,2,3,1]}>(a) }
#[inline] pub fn shuffle_zzxy(a:f32x4)->f32x4 { a.shuffle::<{[3,3,1,2]}>(a) }
#[inline] pub fn shuffle_yxyz(a:f32x4)->f32x4 { a.shuffle::<{[2,1,2,3]}>(a) }
#[inline] pub fn shuffle_zwww(a:f32x4)->f32x4 { a.shuffle::<{[3,0,0,0]}>(a) }
#[inline] pub fn shuffle_wwyy(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,2]}>(a) }
#[inline] pub fn shuffle_ywww(a:f32x4)->f32x4 { a.shuffle::<{[2,0,0,0]}>(a) }
#[inline] pub fn shuffle_xwww(a:f32x4)->f32x4 { a.shuffle::<{[1,0,0,0]}>(a) }
#[inline] pub fn shuffle_xxyz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,2,3]}>(a) }
#[inline] pub fn shuffle_xxzz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) }
#[inline] pub fn shuffle_wwxx(a:f32x4)->f32x4 { a.shuffle::<{[0,0,1,1]}>(a) }
#[inline] pub fn shuffle_yzyz(a:f32x4)->f32x4 { a.shuffle::<{[2,3,2,3]}>(a) }
#[inline] pub fn shuffle_zyzw(a:f32x4)->f32x4 { a.shuffle::<{[3,2,3,0]}>(a) }
#[inline] pub fn shuffle_ywyz(a:f32x4)->f32x4 { a.shuffle::<{[2,0,2,3]}>(a) }
#[inline] pub fn shuffle_wzwy(a:f32x4)->f32x4 { a.shuffle::<{[0,3,0,2]}>(a) }
#[inline] pub fn shuffle_xzwy(a:f32x4)->f32x4 { a.shuffle::<{[1,3,0,2]}>(a) }

#[inline] pub fn bits_wwww(a:u32x4)->u32x4 { a.shuffle::<{[0,0,0,0]}>(a) }

// a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
/* 
let res: f64 = x
        .par_chunks(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.par_chunks(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
*/
// a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()

