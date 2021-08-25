#![feature(portable_simd)]
#[cfg(test)]
mod tests {
  use core_simd::{f32x4,mask32x4};
  use core::arch::x86_64::{_mm_mul_ps, __m128, _mm_set_ps, _mm_extract_ps, _mm_movelh_ps,
    _mm_xor_ps, _mm_set_ss, _mm_movehdup_ps, _mm_unpacklo_ps, _mm_movehl_ps, _mm_sub_ps,
    _mm_and_ps};
  use g3::util::{f32x4_xor,flip_signs, f32x4_andnot, f32x4_abs, f32x4_and};

  unsafe fn m128(a:f32,b:f32,c:f32,d:f32)->__m128 {
    _mm_set_ps(d,c,b,a)
  }

  unsafe fn assert_m128(a:__m128,b:__m128) {
    assert_eq!(_mm_extract_ps(a,0),_mm_extract_ps(b,0), "w");
    assert_eq!(_mm_extract_ps(a,1),_mm_extract_ps(b,1), "x");
    assert_eq!(_mm_extract_ps(a,2),_mm_extract_ps(b,2), "y");
    assert_eq!(_mm_extract_ps(a,3),_mm_extract_ps(b,3), "z");
  }

  unsafe fn printm128(a:__m128) {
    println!("{} {} {} {}",
    _mm_extract_ps(a,0),
    _mm_extract_ps(a,1),
    _mm_extract_ps(a,2),
    _mm_extract_ps(a,3));
  }

  fn printv(v:f32x4) {
    println!("{} {} {} {}", v[0], v[1], v[2], v[3]);
  }

  #[test] fn test_f32_sign_flipping() {
    let v1 = f32x4::from_array([1.0,2.0,3.0,4.0]);
    let expected = f32x4::from_array([-1.0,-2.0,-3.0,4.0]);
    assert_eq!(f32x4_xor(v1, f32x4::from_array([-0.0,-0.0,-0.0,0.0])), expected);
    assert_eq!(flip_signs(v1, mask32x4::from_array([true,true,true,false])), expected);
  }

  #[test] fn test_swizzle() {
    let a = f32x4::from_array([0.0,1.0,2.0,3.0]);
    let zzwy = a.shuffle::<{[3,3,0,1]}>(a);
    assert_eq!(zzwy[0],3.0);
  }

  #[test] fn test_mul() {
    let a = unsafe { _mm_set_ps(1.0,2.0,3.0,4.0) };
    let b = unsafe { _mm_set_ps(1.0,2.0,3.0,4.0) };
    unsafe { assert_m128(a,b) };
    let c = unsafe { _mm_mul_ps(a,b) };
    let d = unsafe { _mm_set_ps(1.0,4.0,9.0,16.0) };
    unsafe { assert_m128(c,d) };
  }

  // https://stackoverflow.com/questions/27485959/sse-intrinsics-masking-a-float-and-using-bitwise-and
  // -1 selects lane otherwise 0
  #[test] fn test_and() {
    // let v1 = f32x4::from_array([1.0, 99.0, 127.0, 1.0]);
    // let v = mask32x4::from_array([true, false, true, false]).select(v1, f32x4::splat(0.0));
    // printv(v);
    // unsafe { printm128(_mm_set_ps(0.0,0.0,0.0,-1.0)) };
    // unsafe { printm128(_mm_and_ps(_mm_set_ps(2.0,0.0,0.0,-1.0), _mm_set_ps(0.0,0.0,0.0,-0.0))) };
    // unsafe { printm128(_mm_and_ps(_mm_set_ps(0.0,0.0,0.0,1.0), _mm_set_ps(0.0,0.0,0.0,-0.0))) };
    // unsafe { printm128(_mm_and_ps(_mm_set_ps(0.0,0.0,0.0,2.0), _mm_set_ps(0.0,0.0,0.0,-0.0))) };
    // unsafe { printm128(_mm_and_ps(_mm_set_ps(0.0,0.0,0.0,-2.0), _mm_set_ps(0.0,0.0,0.0,-0.0))) };

    printv(f32x4_and(f32x4::from_array([-1.0, 2.0, 3.0, 4.0]), f32x4::from_array([-1.0,0.0,0.0,0.0])));
    printv(f32x4_and(f32x4::from_array([-1.0, 2.0, 3.0, 4.0]), f32x4::from_array([0.0,0.0,0.0,0.0])));
    printv(f32x4_and(f32x4::from_array([-1.0, 2.0, 3.0, 4.0]), f32x4::from_array([1.0,0.0,0.0,0.0])));
    printv(f32x4_and(f32x4::from_array([1.0, 2.0, 3.0, 4.0]), f32x4::from_array([1.0,0.0,0.0,0.0])));
  }

  #[test] fn test_xor() {
    let a = unsafe { _mm_set_ps(1.0,2.0,3.0,4.0) };
    let b = unsafe { _mm_xor_ps(a, _mm_set_ss(-0.0)) };
    unsafe { printm128(b) };
    unsafe { printm128(_mm_xor_ps(_mm_set_ss(-0.0), a)) };
    unsafe { printm128(_mm_xor_ps(a, _mm_set_ss(-0.0))) };
    let c = unsafe { _mm_set_ps(-2.0,-1.0,2.0,1.0) };
    unsafe { printm128(c) };
    unsafe { printm128(_mm_xor_ps(c, _mm_set_ss(-0.0))) };
    unsafe { printm128(_mm_sub_ps(c, _mm_xor_ps(c, _mm_set_ss(-0.0)))) };
  }

  // _mm_movelh_ps
  #[test] fn test_movehl() {
    unsafe {
      let a = _mm_set_ps(0.0, 0.0, 0.0, 0.0);
      let b = _mm_set_ps(1.0, 1.0, 1.0, 1.0);
      let v = _mm_movelh_ps(a,b);
      printm128(v);
      assert_m128(v, _mm_set_ps(1.0, 1.0, 0.0, 0.0));
    };
  }

  #[test] fn test_mm_movehdup_ps() {
    unsafe {
      let a = m128(0.0, 1.0, 2.0, 3.0);
      // Duplicate odd-indexed single-precision (32-bit) floating-point elements from a
      let b = _mm_movehdup_ps(a);
      printm128(b);
      assert_m128(b, m128(1.0, 1.0, 3.0, 3.0));
    }
  }
  
  #[test] fn test_mm_unpacklo_ps() {
    unsafe {
      let a = m128(0.0, 1.0, 2.0, 3.0);
      // Unpacks and interleave single-precision (32-bit) floating-point elements from the lower half of a and b
      let b = _mm_unpacklo_ps(a, a);
      printm128(b);
      assert_m128(b, m128(0.0, 0.0, 1.0, 1.0));
    }
  }

  #[test] fn test_mm_movehl_ps() {
    unsafe {
      let a = m128(0.0, 1.0, 2.0, 3.0);
      // Unpacks and interleave single-precision (32-bit) floating-point elements from the lower half of a and b
      let b = _mm_movehl_ps(a, a);
      printm128(b);
      assert_m128(b, m128(0.0, 0.0, 1.0, 1.0));
    }
  }

  #[test] fn test_f32x4_abs() {
    assert_eq!(f32x4_abs(f32x4::from_array([1.0, -1.0, 0.0, -0.0])), f32x4::from_array([1.0, 1.0, 0.0, 0.0]));
  }

  #[test] fn test_approx_eq() {
    let a = f32x4::from_array([1.0, -1.0, 2.0, 0.0]);
    let b = f32x4::from_array([0.9, -0.9, 1.9, -0.1]);
    let diff  = a - b;
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("diff: {:?}", diff);
    println!("and_not_diff: {:?}", f32x4_andnot(f32x4::splat(-0.0), a - b));
    println!("approx_eq: {:?}", f32x4_abs(a - b) < f32x4::splat(0.2));
    assert_eq!(f32x4_abs(a - b) < f32x4::splat(0.2), true);
    assert_eq!(f32x4_abs(a - b) < f32x4::splat(0.11), true);
    assert_eq!(f32x4_abs(a - b) < f32x4::splat(0.1), false);
  }

  // _mm_xor_ps(_mm_mul_ps(a, KLN_SWIZZLE(c, 0, 0, 0, 0)), _mm_set_ss(-0.f))
  // _mm_movehdup_ps shuffle_xxzz
}
