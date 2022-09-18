#![feature(portable_simd)] #[cfg(test)]
mod tests {
  use std::simd::{f32x4};
  use g3::maths::{f32x4_xor, flip_signs, f32x4_andnot, f32x4_abs};

  #[test] fn test_f32_sign_flipping() {
    let v1 = f32x4::from_array([1.0,2.0,3.0,4.0]);
    let expected = f32x4::from_array([-1.0,-2.0,-3.0,4.0]);
    assert_eq!(f32x4_xor(&v1, &f32x4::from_array([-0.0,-0.0,-0.0,0.0])), expected);
    assert_eq!(flip_signs(&v1, [true,true,true,false].into()), expected);
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
}
