use std::simd::{f32x4};
use crate::maths::{hi_dp_bc, rcp_nr1, rsqrt_nr1};

// a + b is a general bivector but it is most likely *non-simple* meaning
// that it is neither purely real nor purely ideal.
// Exponentiates the bivector and returns the motor defined by partitions 1
// and 2.
pub fn exp(a:&f32x4, b:&f32x4)->(f32x4,f32x4) {
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

#[cfg(test)]
mod tests {

}
