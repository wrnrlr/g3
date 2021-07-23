use core_simd::{f32x4,Mask32,mask32x4};
use crate::sqrt::{rsqrt_nr1};

pub fn refined_reciprocal(s:f32)->f32x4 {
  rcp_nr1(f32x4::splat(s))
}

pub fn rcp_nr1(_a:f32x4)->f32x4 {
  todo!()
}

pub fn rcp_rc1(_a:f32x4)->f32x4 {
  todo!()
}

pub fn exp(_a:f32x4,_b:f32x4)->(f32x4,f32x4) { todo!(); }

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

pub fn dot_product(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn f32x4_xor(a:f32x4,b:f32x4)->f32x4 {
  f32x4::from_bits(a.to_bits() ^ b.to_bits())
}

pub fn f32x4_flip_signs(x:f32x4, mask:Mask32<4>)->f32x4 {
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

#[inline] pub fn shuffle_yyzz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) } // ???

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

