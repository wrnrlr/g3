use core_simd::{f32x4};
use crate::util::{f32x4_xor,shuffle_wwww,shuffle_zzwy,shuffle_wwyz,shuffle_yyzz,shuffle_zwyz,shuffle_yzwy,shuffle_wyzw,shuffle_dddd,shuffle_zwyx,shuffle_yyzw};

// p3: (w,    x,    y,    z)
// p3: (e123, e032, e013, e021)

// p0: (e0, e1, e2, e3)
// d, x, y, z

// Partition memory layouts
//     LSB --> MSB
// p0: (e0, e1, e2, e3)
// p1: (1, e23, e31, e12)
// p2: (e0123, e01, e02, e03)
// p3: (e123, e032, e013, e021)

// Reflect a plane through another plane
// b * a * b
pub fn sw00(a:f32x4,b:f32x4)->f32x4 {
  // (2a0(a2 b2 + a3 b3 + a1 b1) - b0(a1^2 + a2^2 + a3^2)) e0 +
  // (2a1(a2 b2 + a3 b3)         + b1(a1^2 - a2^2 - a3^2)) e1 +
  // (2a2(a3 b3 + a1 b1)         + b2(a2^2 - a3^2 - a1^2)) e2 +
  // (2a3(a1 b1 + a2 b2)         + b3(a3^2 - a1^2 - a2^2)) e3
  let a_zzwy = shuffle_zzwy(a);
  let a_wwyz = shuffle_wwyz(a);

  // Left block
  let mut tmp = a_zzwy * shuffle_zzwy(b);
  tmp = tmp + a_wwyz * shuffle_wwyz(b);

  let a1 = shuffle_yyzz(a);
  let b1 = shuffle_yyzz(b);
  tmp = tmp + a1 * b1;
  tmp = tmp * (a + a);

  // Right block
  let a_yyzw = shuffle_yyzw(a);
  let mut tmp2 = f32x4_xor(a_yyzw * a_yyzw, f32x4::splat(-0.0));
  tmp2 = tmp2 - a_zzwy * a_zzwy;
  tmp2 = tmp2 - a_wwyz * a_wwyz;
  tmp2 = tmp2 * b;

  return tmp + tmp2
}

// reflect point through plane
pub fn sw30(p0:f32x4,p3:f32x4)->f32x4 {
  //                                b0(a1^2 + a2^2 + a3^2)  e123 +
  // (-2a1(a0 b0 + a3 b3 + a2 b2) + b1(a2^2 + a3^2 - a1^2)) e032 +
  // (-2a2(a0 b0 + a1 b1 + a3 b3) + b2(a3^2 + a1^2 - a2^2)) e013 +
  // (-2a3(a0 b0 + a2 b2 + a1 b1) + b3(a1^2 + a2^2 - a3^2)) e021

  let a_zwyz = shuffle_zwyz(p0);
  let a_yzwy = shuffle_yzwy(p3);

  let mut p3_out = shuffle_dddd(p0) * shuffle_wwww(p3);
  p3_out = p3_out + a_zwyz * shuffle_zwyx(p3);
  p3_out = p3_out + a_zwyz * shuffle_yzwy(p3);
  p3_out = p3_out * (p0 * f32x4::from_array([-2.0,-2.0,-2.0,-2.0]));

  let mut tmp = a_yzwy * a_yzwy;
  tmp = tmp + (a_zwyz * a_zwyz);
  let a_wyzw = shuffle_wyzw(p0);
  tmp = tmp - f32x4_xor(a_wyzw * a_wyzw, f32x4::from_array([-0.0,-0.0,-0.0,-0.0]));

  return p3_out + p3 * tmp
}

pub fn sw012<const N:bool,const F:bool>(_p0:f32x4,_p1:f32x4)->f32x4 {
  todo!()
}

pub fn swmm<const N:bool,const F:bool,const P:bool>(_a:f32x4,_b:f32x4,_c:Option<f32x4>)->(f32x4,f32x4) { // todo, c doesn't seem to be used add count argument
  todo!()
}

pub fn sw02(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn sw32(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn sw33(_a:f32x4,_b:f32x4)->f32x4 {
  todo!()
}

pub fn swl2(_a:f32x4,_b:f32x4,_c:f32x4)->f32x4 {
  todo!()
}

pub fn sw312<const N:bool,const F:bool>(_a:f32x4,_b:f32x4,_c:f32x4)->f32x4 { // todo count param
  todo!()
}
