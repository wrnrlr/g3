#![allow(unused_imports)]
#[allow(unused_variables)]

use core_simd::{f32x4,Mask32,mask32x4};

pub fn refined_reciprocal(s:f32)->f32x4 {
  rcp_nr1(f32x4::splat(s))
}

pub fn rcp_nr1(_a:f32x4)->f32x4 {
  todo!()
}

// Equivalent to _mm_dp_ps(a, b, 0b11100001);

pub fn hi_dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);
  
  let sum  = hi + out;
  out = sum + shuffle_low(out);
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn hi_dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);
  
  let sum  = hi + out;
  out = sum + shuffle_low(out);
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn dp(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out[0] += b2b3a2a3(hi,out)[0];
  mask32x4::from_array([true, false, false, false]).select(out, f32x4::splat(0.0))
}

pub fn dp_bc(a:f32x4, b:f32x4)->f32x4 {
  let mut out = a * b;
  let hi = shuffle_odd(out);

  // (a1 b1, a2 b2, a3 b3, 0) + (a2 b2, a2 b2, 0, 0)
  // = (a1 b1 + a2 b2, _, a3 b3, 0)
  out = hi + out;
  out[0] += b2b3a2a3(hi,out)[0];
  shuffle_first(out);
  out
}

pub fn dot_product(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn f32x4_xor(a:f32x4,b:f32x4)->f32x4 {
  f32x4::from_bits(a.to_bits() ^ b.to_bits())
}

pub fn f32x4_flip_signs(x:f32x4, mask:Mask32<4>)->f32x4 {
  mask.select(-x, x)
}

#[inline] pub fn add_ss(a:f32x4,b:f32x4)->f32x4 {
  let tmp = a + b;
  tmp.shuffle::<{[0,5,6,7]}>(a) 
}

#[inline] pub fn mul_ss(a:f32x4,b:f32x4)->f32x4 {
  let tmp = a * b;
  tmp.shuffle::<{[0,5,6,7]}>(a) 
}

#[inline] pub fn shuffle_first(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }
#[inline] pub fn shuffle_low(a:f32x4)->f32x4 { a.shuffle::<{[0,0,1,1]}>(a) }

#[inline] pub fn b2b3a2a3(a:f32x4,b:f32x4)->f32x4 { a.shuffle::<{[6,7,2,3]}>(b) } // b2b3a2a3

#[inline] pub fn shuffle_odd(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) }
#[inline] pub fn shuffle_even(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,2]}>(a) }

// #[inline] fn shuffle_odd(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) }

#[inline] pub fn shuffle_zzwy(a:f32x4)->f32x4 { a.shuffle::<{[3,3,0,1]}>(a) }
#[inline] pub fn shuffle_wwyz(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,3]}>(a) }
// #[inline] fn shuffle_wwyz(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,3]}>(a) }

#[inline] pub fn shuffle_yyzz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,3,3]}>(a) } // ???

#[inline] pub fn shuffle_yyzw(a:f32x4)->f32x4 { a.shuffle::<{[2,2,3,0]}>(a) }

#[inline] pub fn shuffle_zwyz(a:f32x4)->f32x4 { a.shuffle::<{[3,0,2,3]}>(a) }
#[inline] pub fn shuffle_yzwy(a:f32x4)->f32x4 { a.shuffle::<{[2,3,0,2]}>(a) }

#[inline] pub fn shuffle_zwyx(a:f32x4)->f32x4 { a.shuffle::<{[3,0,2,1]}>(a) }
#[inline] pub fn shuffle_yzwx(a:f32x4)->f32x4 { a.shuffle::<{[2,3,0,1]}>(a) }

#[inline] pub fn shuffle_wyzw(a:f32x4)->f32x4 { a.shuffle::<{[0,2,3,0]}>(a) }

#[inline] pub fn shuffle_wwww(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }
#[inline] pub fn shuffle_dddd(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }

#[inline] pub fn shuffle_scalar(a:f32x4)->f32x4 { a.shuffle::<{[0,0,0,0]}>(a) }

#[inline] pub fn shuffle_yzxy(a:f32x4)->f32x4 { a.shuffle::<{[2,3,1,2]}>(a) }
#[inline] pub fn shuffle_yyzx(a:f32x4)->f32x4 { a.shuffle::<{[2,2,3,1]}>(a) }
#[inline] pub fn shuffle_xyzx(a:f32x4)->f32x4 { a.shuffle::<{[1,2,3,1]}>(a) }
#[inline] pub fn shuffle_xzxy(a:f32x4)->f32x4 { a.shuffle::<{[1,3,1,2]}>(a) }
#[inline] pub fn shuffle_wzxy(a:f32x4)->f32x4 { a.shuffle::<{[0,3,1,2]}>(a) }
#[inline] pub fn shuffle_wyzx(a:f32x4)->f32x4 { a.shuffle::<{[0,2,3,1]}>(a) }
#[inline] pub fn shuffle_zyzx(a:f32x4)->f32x4 { a.shuffle::<{[3,2,3,1]}>(a) }
#[inline] pub fn shuffle_zzxy(a:f32x4)->f32x4 { a.shuffle::<{[3,3,1,2]}>(a) }
#[inline] pub fn shuffle_yxyz(a:f32x4)->f32x4 { a.shuffle::<{[2,1,2,3]}>(a) }
#[inline] pub fn shuffle_zwww(a:f32x4)->f32x4 { a.shuffle::<{[3,0,0,0]}>(a) }
#[inline] pub fn shuffle_wwyy(a:f32x4)->f32x4 { a.shuffle::<{[0,0,2,2]}>(a) }
#[inline] pub fn shuffle_ywww(a:f32x4)->f32x4 { a.shuffle::<{[2,0,0,0]}>(a) }
#[inline] pub fn shuffle_xwww(a:f32x4)->f32x4 { a.shuffle::<{[1,0,0,0]}>(a) }
#[inline] pub fn shuffle_xxyz(a:f32x4)->f32x4 { a.shuffle::<{[1,1,2,3]}>(a) }

// a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
/* 
let res: f64 = x
        .par_chunks(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.par_chunks(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
*/
// a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()

