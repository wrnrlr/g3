use std_float::StdFloat as _;
use core_simd::{f32x4,mask32x4,u32x4,simd_swizzle as swizzle};
use core_simd::simd::Which::{First,Second};

pub fn refined_reciprocal(s:f32)->f32x4 {
  rcp_nr1(f32x4::splat(s))
}

pub fn sqrt_nr1(a:f32x4)->f32x4 {
  a * rsqrt_nr1(a) // TODO either write faster rsqrt_nr1, or derive sqrt_nr1 yourself...
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
  let a_sqrt = a.sqrt();
  let xn = f32x4::splat(1.0) / a_sqrt;
  let axn2 = xn * xn * a;
  let xn3 = f32x4::splat(3.0) - axn2;
  f32x4::splat(0.5) * xn * xn3
}

#[inline] pub fn rcp(a:f32x4)->f32x4 {
  f32x4::splat(1.0) / a
}

// Reciprocal with an additional single Newton-Raphson refinement
#[inline] pub fn rcp_nr1(a:f32x4)->f32x4 {
  // f(x) = 1/x - a
  // f'(x) = -1/x^2
  // x_{n+1} = x_n - f(x)/f'(x)
  //         = 2x_n - a x_n^2 = x_n (2 - a x_n)
  let xn = rcp(a);
  let axn = a * xn;
  xn * (f32x4::splat(2.0) - axn)
}

pub fn hi_dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_yyww(out);

  let sum  = hi + out;
  out = sum + shuffle_xxyy(out);
  out = shuffle_zwzw(out);
  swizzle!(out, f32x4::splat(0.0), [First(0),Second(1),Second(2),Second(3)]) // TODO make faster???
}

pub fn hi_dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_yyww(out);

  let sum  = hi + out;
  out = sum + shuffle_xxyy(out);
  shuffle_zzzz(out)
}

pub fn hi_dp_ss(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_yyww(out);
  let sum = hi + out;
  out = sum + shuffle_xxyy(out);
  shuffle_zwzw(out)
}

pub fn dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_yyww(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out[0] += b2b3a2a3(hi,out)[0];
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_yyww(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out = add_ss(out, b2b3a2a3(hi, out));
  shuffle_xxxx(out)
}

#[inline] pub fn zero_first(a:f32x4)->f32x4 { swizzle!(a, f32x4::splat(0.0), [Second(0), First(1), First(2), First(3)]) }

#[inline] pub fn f32x4_xor(a:f32x4,b:f32x4)->f32x4 { f32x4::from_bits(a.to_bits() ^ b.to_bits()) }
#[inline] pub fn f32x4_and(a:f32x4,b:f32x4)->f32x4 { f32x4::from_bits(a.to_bits() & b.to_bits()) }

#[inline] pub fn f32x4_andnot(a:f32x4,b:f32x4)->f32x4 { f32x4::from_bits(!a.to_bits() & b.to_bits()) }

// Is this faster then f32x4::abs, which is implemented in rust?
#[inline] pub fn f32x4_abs(a:f32x4)->f32x4 { f32x4_andnot(f32x4::splat(-0.0), a) }

#[inline] pub fn flip_signs(x:f32x4, mask:mask32x4)->f32x4 { mask.select(-x, x) }

#[inline] pub fn add_ss(a:f32x4,b:f32x4)->f32x4 { swizzle!(a + b, a, [First(0), Second(1), Second(2), Second(3)]) }
#[inline] pub fn sub_ss(a:f32x4,b:f32x4)->f32x4 { swizzle!(a - b, a, [First(0), Second(1), Second(2), Second(3)]) }
#[inline] pub fn mul_ss(a:f32x4,b:f32x4)->f32x4 { swizzle!(a * b, a, [First(0), Second(1), Second(2), Second(3)]) }

#[inline] pub fn b2b3a2a3(a:f32x4,b:f32x4)->f32x4 { swizzle!(a, b, [Second(2),Second(3),First(2),First(3)]) }
#[inline] pub fn b0a1a2a3(a:f32x4,b:f32x4)->f32x4 { swizzle!(a, b, [Second(0),First(1),First(2),First(3)]) }

#[inline] pub fn shuffle_xxxx(a:f32x4)->f32x4 { swizzle!(a, [0,0,0,0]) }
#[inline] pub fn shuffle_xyxy(a:f32x4)->f32x4 { swizzle!(a, [0,1,0,1]) }
#[inline] pub fn shuffle_xxyy(a:f32x4)->f32x4 { swizzle!(a, [0,0,1,1]) }
#[inline] pub fn shuffle_xxzz(a:f32x4)->f32x4 { swizzle!(a, [0,0,2,2]) }
#[inline] pub fn shuffle_xzxx(a:f32x4)->f32x4 { swizzle!(a, [0,2,0,0]) }
#[inline] pub fn shuffle_xzwy(a:f32x4)->f32x4 { swizzle!(a, [0,2,3,1]) }
#[inline] pub fn shuffle_xwyz(a:f32x4)->f32x4 { swizzle!(a, [0,3,1,2]) }
#[inline] pub fn shuffle_xwzy(a:f32x4)->f32x4 { swizzle!(a, [0,3,2,1]) }

#[inline] pub fn shuffle_yxxx(a:f32x4)->f32x4 { swizzle!(a, [1,0,0,0]) }
#[inline] pub fn shuffle_yxwx(a:f32x4)->f32x4 { swizzle!(a, [1,0,3,0]) }
#[inline] pub fn shuffle_yyyy(a:f32x4)->f32x4 { swizzle!(a, [1,1,1,1]) }
#[inline] pub fn shuffle_yyzw(a:f32x4)->f32x4 { swizzle!(a, [1,1,2,3]) }
#[inline] pub fn shuffle_yywz(a:f32x4)->f32x4 { swizzle!(a, [1,1,3,2]) }
#[inline] pub fn shuffle_yyww(a:f32x4)->f32x4 { swizzle!(a, [1,1,3,3]) }
#[inline] pub fn shuffle_yzxx(a:f32x4)->f32x4 { swizzle!(a, [1,2,0,0]) }
#[inline] pub fn shuffle_yzyz(a:f32x4)->f32x4 { swizzle!(a, [1,2,1,2]) }
#[inline] pub fn shuffle_yzyw(a:f32x4)->f32x4 { swizzle!(a, [1,2,1,3]) }
#[inline] pub fn shuffle_yzwx(a:f32x4)->f32x4 { swizzle!(a, [1,2,3,0]) }
#[inline] pub fn shuffle_yzwy(a:f32x4)->f32x4 { swizzle!(a, [1,2,3,1]) }
#[inline] pub fn shuffle_ywyx(a:f32x4)->f32x4 { swizzle!(a, [1,3,1,0]) }
#[inline] pub fn shuffle_ywyz(a:f32x4)->f32x4 { swizzle!(a, [1,3,1,2]) }

#[inline] pub fn shuffle_zxxx(a:f32x4)->f32x4 { swizzle!(a, [2,0,0,0]) }
#[inline] pub fn shuffle_zxzx(a:f32x4)->f32x4 { swizzle!(a, [2,0,2,0]) }
#[inline] pub fn shuffle_zyzw(a:f32x4)->f32x4 { swizzle!(a, [2,1,2,3]) }
#[inline] pub fn shuffle_zzzz(a:f32x4)->f32x4 { swizzle!(a, [2,2,2,2]) }
#[inline] pub fn shuffle_zzwx(a:f32x4)->f32x4 { swizzle!(a, [2,2,3,0]) }
#[inline] pub fn shuffle_zzwy(a:f32x4)->f32x4 { swizzle!(a, [2,2,3,1]) }
#[inline] pub fn shuffle_zzww(a:f32x4)->f32x4 { swizzle!(a, [2,2,3,3]) }
#[inline] pub fn shuffle_zwxx(a:f32x4)->f32x4 { swizzle!(a, [2,3,0,0]) }
#[inline] pub fn shuffle_zwxy(a:f32x4)->f32x4 { swizzle!(a, [2,3,0,1]) }
#[inline] pub fn shuffle_zwyx(a:f32x4)->f32x4 { swizzle!(a, [2,3,1,0]) }
#[inline] pub fn shuffle_zwyz(a:f32x4)->f32x4 { swizzle!(a, [2,3,1,2]) }
#[inline] pub fn shuffle_zwzw(a:f32x4)->f32x4 { swizzle!(a, [2,3,2,3]) }
#[inline] pub fn shuffle_zwwy(a:f32x4)->f32x4 { swizzle!(a, [2,3,3,1]) }

#[inline] pub fn shuffle_wxxx(a:f32x4)->f32x4 { swizzle!(a, [3,0,0,0]) }
#[inline] pub fn shuffle_wyzx(a:f32x4)->f32x4 { swizzle!(a, [3,1,2,0]) }
#[inline] pub fn shuffle_wyzw(a:f32x4)->f32x4 { swizzle!(a, [3,1,2,3]) }
#[inline] pub fn shuffle_wywx(a:f32x4)->f32x4 { swizzle!(a, [3,1,3,0]) }
#[inline] pub fn shuffle_wywz(a:f32x4)->f32x4 { swizzle!(a, [3,1,3,2]) }
#[inline] pub fn shuffle_wzyz(a:f32x4)->f32x4 { swizzle!(a, [3,2,1,2]) }
#[inline] pub fn shuffle_wzyw(a:f32x4)->f32x4 { swizzle!(a, [3,2,1,3]) }
#[inline] pub fn shuffle_wzwy(a:f32x4)->f32x4 { swizzle!(a, [3,2,3,1]) }
#[inline] pub fn shuffle_wwxx(a:f32x4)->f32x4 { swizzle!(a, [3,3,0,0]) }
#[inline] pub fn shuffle_wwyz(a:f32x4)->f32x4 { swizzle!(a, [3,3,1,2]) }
#[inline] pub fn shuffle_wwww(a:f32x4)->f32x4 { swizzle!(a, [3,3,3,3]) }

#[inline] pub fn bits_wwww(a:u32x4)->u32x4 { swizzle!(a, [0,0,0,0]) }

#[cfg(test)]
mod tests {
  use super::*;
  use core_simd::{f32x4};

  #[test] fn dp_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
    let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
    assert_eq!(dp(a, b), f32x4::from([-21.0, 0.0, 0.0, 0.0]));
  }

  #[test] fn hi_dp_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
    let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
    assert_eq!(hi_dp(a, b), f32x4::from([-17.0, 0.0, 0.0, 0.0]));
  }

  #[test] fn hi_dp_bc_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
    let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
    assert_eq!(hi_dp_bc(a, b), f32x4::from([-17.0, -17.0, -17.0, -17.0]));
  }

  #[test] fn dp_bc_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
    let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
    assert_eq!(dp_bc(a, b), f32x4::from([-21.0, -21.0, -21.0, -21.0]));
  }

  #[test] fn hi_dp_ss_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
    let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
    assert_eq!(hi_dp_ss(a, b), f32x4::from([-17.0, -16.0, -17.0, -16.0]));
  }

  #[test] fn add_first() {
    let a = f32x4::from([2.0, 2.0, 3.0, 4.0]);
    assert_eq!(add_ss(a, a), f32x4::from([4.0, 2.0, 3.0, 4.0]));
  }

  #[test] fn multiply_first() {
    let a = f32x4::from([2.0, 2.0, 3.0, 4.0]);
    assert_eq!(mul_ss(a, a), f32x4::from([4.0, 2.0, 3.0, 4.0]));
  }

  #[test] fn inverse_sqrt() {
    let a = f32x4::from([4.0, 9.0, 16.0, 25.0]);
    assert_eq!(a.sqrt(), f32x4::from([2.0, 3.0, 4.0, 5.0]));
    assert_eq!(f32x4::splat(1.0)/a.sqrt(), f32x4::from([1.0/2.0, 1.0/3.0, 1.0/4.0, 1.0/5.0]));
  }

  #[test] fn rcp_nr1_test() {
    let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
    let b = rcp_nr1(a);
    assert_eq!(b[0], 1.0);
    assert_eq!(b[1], 0.5);
    assert_eq!(b[2], 1.0/3.0);
    assert_eq!(b[3], 0.25);
  }

  #[test] fn f32x4_abs_test() {
    assert_eq!(f32x4_abs(f32x4::from([1.0, 0.0, -1.0, -0.0])), f32x4::from([1.0, 0.0, 1.0, 0.0]));
  }

  #[test] #[ignore] fn rsqrt_nr1_test() {}
}
