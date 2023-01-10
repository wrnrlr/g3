use std::simd::{*, simd_swizzle as swizzle, Which::{First, Second}, StdFloat as _};

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
  let mut ab = a*b;
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
#[inline] pub fn f32x4_andnot(a:f32x4,b:f32x4)->f32x4 { f32x4::from_bits(!a.to_bits() & b.to_bits()) }
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
