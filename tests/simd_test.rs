#[cfg(test)]
mod tests {

  unsafe fn assert_m128(a:__m128,b:__m128) {
    assert_eq!(_mm_extract_ps(a,0),_mm_extract_ps(b,0));
    assert_eq!(_mm_extract_ps(a,1),_mm_extract_ps(b,1));
    assert_eq!(_mm_extract_ps(a,2),_mm_extract_ps(b,2));
    assert_eq!(_mm_extract_ps(a,3),_mm_extract_ps(b,3));
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

  use core_simd::{f32x4,mask32x4,Mask32};
  use core::arch::x86_64::{_mm_mul_ps,__m128,_mm_set_ps,_mm_extract_ps,_mm_movelh_ps,_mm_xor_ps, _mm_set_ss};
  use g3::util::{f32x4_xor,f32x4_flip_signs};

  #[test] fn test_f32_sign_flipping() {
    let v1 = f32x4::from_array([1.0,2.0,3.0,4.0]);
    let expected = f32x4::from_array([-1.0,-2.0,-3.0,4.0]);
    assert_eq!(f32x4_xor(v1, f32x4::from_array([-0.0,-0.0,-0.0,0.0])), expected);
    assert_eq!(f32x4_flip_signs(v1, Mask32::from_array([true,true,true,false])), expected);
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
    unsafe { assert_m128(a,b) };
  }

  // https://stackoverflow.com/questions/27485959/sse-intrinsics-masking-a-float-and-using-bitwise-and
  // -1 selects lane otherwise 0
  #[test] fn test_and() {
    let v1 = f32x4::from_array([1.0, 99.0, 127.0, 1.0]);
    let v = mask32x4::from_array([true, false, true, false]).select(v1, f32x4::splat(0.0));
    printv(v);
  }

  #[test] fn test_xor() {
    let a = unsafe { _mm_set_ps(1.0,2.0,3.0,4.0) };
    let b = unsafe { _mm_xor_ps(a, _mm_set_ss(-0.0)) };
    unsafe { printm128(b) };
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

  // _mm_xor_ps(_mm_mul_ps(a, KLN_SWIZZLE(c, 0, 0, 0, 0)), _mm_set_ss(-0.f))

}

