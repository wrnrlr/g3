use std::simd::{f32x4};
use crate::maths::*;

pub fn logarithm(p1:&f32x4, p2:&f32x4) ->(f32x4, f32x4) {
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

#[cfg(test)]
mod tests {

}