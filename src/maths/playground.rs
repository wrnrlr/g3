// use std_float::StdFloat as _;
// use core_simd::{f32x4,mask32x4,u32x4,simd_swizzle as swizzle};
// use core_simd::simd::Which::{First,Second};
//
// use std::arch::x86_64::*;
//
// unsafe fn into_m128(a:f32x4)->__m128 {
//   _mm_set_ps(a[3], a[2], a[1], a[0])
// }
//
// unsafe fn into_f32x4(a:__m128)->f32x4 {
//   f32x4::from(std::mem::transmute::<__m128, [f32; 4]>(a))
// }

// pub fn sse_hi_dp_ss(a:f32x4, b:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let b = into_m128(b);
//     let mut out = _mm_mul_ps(a, b);
//     let hi = _mm_movehdup_ps(out);
//     let sum = _mm_add_ps(hi, out);
//     out = _mm_add_ps(sum, _mm_unpacklo_ps(out, out));
//     into_f32x4(_mm_movehl_ps(out, out))
//   }
// }
//
// pub fn sse_dp_bc(a:f32x4, b:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let b = into_m128(b);
//     let mut out = _mm_mul_ps(a, b);
//     let hi = _mm_movehdup_ps(out);
//     out = _mm_add_ps(hi, out);
//     out = _mm_add_ps(out, _mm_movehl_ps(hi, out));
//     const index: i32 = _MM_SHUFFLE(0, 0, 0, 0);
//     into_f32x4(_mm_shuffle_ps::<index>(out, out))
//   }
// }
//
// pub fn sse_hi_dp_bc(a:f32x4, b:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let b = into_m128(b);
//     let mut out = _mm_mul_ps(a, b);
//     let hi = _mm_movehdup_ps(out);
//     let sum = _mm_add_ps(hi, out);
//     out = _mm_add_ps(sum, _mm_unpacklo_ps(out, out));
//     const index: i32 = _MM_SHUFFLE(2, 2, 2, 2);
//     into_f32x4(_mm_shuffle_ps::<index>(out, out))
//   }
// }
//
// pub fn sse_dp(a:f32x4, b:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let b = into_m128(b);
//     let mut out = _mm_mul_ps(a, b);
//     let hi = _mm_movehdup_ps(out);
//     out = _mm_add_ps(hi, out);
//     out = _mm_add_ss(out, _mm_movehl_ps(hi, out));
//     into_f32x4(_mm_and_ps(out, _mm_castsi128_ps(_mm_set_epi32(0, 0, 0, -1))))
//   }
// }
//
// pub fn sse_hi_dp(a:f32x4, b:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let b = into_m128(b);
//     let mut out = _mm_mul_ps(a, b);
//     let hi = _mm_movehdup_ps(out);
//     let sum = _mm_add_ps(hi, out);
//     out = _mm_add_ps(sum, _mm_unpacklo_ps(out, out));
//     out = _mm_movehl_ps(out, out);
//     into_f32x4(_mm_and_ps(out, _mm_castsi128_ps(_mm_set_epi32(0, 0, 0, -1))))
//   }
// }


// #[inline] pub fn rcp(a:f32x4)->f32x4 {
//   f32x4::splat(1.0) / a
// }
//
// fn normalize(a:f32x4)->f32x4 {
//   a * rcp_nr1(a)
// }
//
// pub fn sse_rcp(a:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     into_f32x4(_mm_rcp_ps(a))
//   }
// }
//
// pub fn rcp_nr1(a:f32x4)->f32x4 {
//   let xn = rcp(a);
//   let axn = a * xn;
//   // let axn2 = xn * xn * a;
//   // let xn3 = f32x4::splat(3.0) - axn2;
//   // f32x4::splat(0.5) * xn * xn3
//   xn * (f32x4::splat(2.0) - axn)
// }
//
// pub fn sse_rcp_nr1(a:f32x4)->f32x4 {
//   unsafe {
//     let a = into_m128(a);
//     let xn  = _mm_rcp_ps(a);
//     let axn = _mm_mul_ps(a, xn);
//     into_f32x4(_mm_mul_ps(xn, _mm_sub_ps(_mm_set1_ps(2.0), axn)))
//   }
// }
//
// fn sse_normalize(a:f32x4)->f32x4 {
//   a * sse_rcp_nr1(a)
// }

mod tests {
  // use super::*;
  // use core_simd::{f32x4};

  // #[test] fn sse_test() {
  //   let a = f32x4::from([1.0, 2.0, 3.0, 5.0]);
  //   let b = f32x4::from([-4.0, -3.0, -2.0, -1.0]);
  //   assert_eq!(unsafe{into_f32x4(into_m128(a))}, a);
  //   assert_eq!(dp(a, b), sse_dp(a,b));
  //   assert_eq!(hi_dp_ss(a, b), sse_hi_dp_ss(a,b));
  //   assert_eq!(hi_dp(a, b), sse_hi_dp(a,b));
  //   assert_eq!(hi_dp_bc(a, b), sse_hi_dp_bc(a,b));
  //   assert_eq!(dp_bc(a, b), sse_dp_bc(a,b));
  // }

  // #[test] fn sse_rcp_test() {
  //   let a = f32x4::from([1.0, 2.0, 3.0, 0.0]);
  //   println!("{:?}\n{:?}", sse_rcp(a), rcp(a));
  //   // assert_eq!(sse_rcp_nr1(a), rcp_nr1(a));
  //   assert_eq!(sse_normalize(a), normalize(a))
  // }
}