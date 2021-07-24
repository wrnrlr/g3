use core_simd::{f32x4};

pub fn sqrt_nr1(a:f32x4)->f32x4 {
  a * rsqrt_nr1(a)
}

// Reciprocal sqrt with an additional single Newton-Raphson refinement.
pub fn rsqrt_nr1(a:f32x4)->f32x4 {
  // f(x) = 1/x^2 - a
  // f'(x) = -1/(2x^(3/2))
  // Let x_n be the estimate, and x_{n+1} be the refinement
  // x_{n+1} = x_n - f(x)/f'(x)
  //         = 0.5 * x_n * (3 - a x_n^2)

  // TODO find portable version of _mm_rsqrt_ps in core_simd
  // From Intel optimization manual: expected performance is ~5.2x
  // baseline (sqrtps + divps) with ~22 bits of accuracy
  let xn = f32x4::splat(1.0) / a.sqrt();
  let axn2 = a * xn * xn;
  let xn3 = f32x4::splat(3.0) - axn2;
  f32x4::splat(0.5) * xn * xn3
}
