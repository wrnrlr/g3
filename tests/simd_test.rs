#[cfg(test)]
mod tests {
  use core_simd::{f32x4,Mask32};
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
}

