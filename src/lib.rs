#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(adt_const_params)]
#![feature(portable_simd)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]
#![allow(non_upper_case_globals)]

mod dual;
mod point;
mod line;
mod branch;
mod horizon;
mod plane;
mod motor;
mod rotor;
mod direction;
mod translator;

/// π
pub const pi:f32 =  std::f32::consts::PI;
/// τ = 2π
pub const tau:f32 = pi*2.0;

use std::ops::{Add, Div, Mul, Sub};
pub use dual::{Dual, dual};
pub use point::*;
pub use line::{Line,line};
pub use branch::{Branch,branch};
pub use horizon::{Horizon,horizon};
pub use plane::*;
pub use motor::{Motor,motor};
pub use rotor::{EulerAngles,Rotor,rotor};
pub use direction::{Direction};
pub use translator::{Translator,translator};
pub(crate) mod maths;

#[cfg(feature = "renderer")] mod render;
#[cfg(feature = "renderer")] pub use render::{Renderer,Color};

/// !a
pub trait PoincareDual {}
/// a * b = a|b + a^b
pub trait GeometricProduct {}
/// a & b
pub trait JoinProduct {}
/// a ^ b
pub trait MeetProduct {}
/// a(b) = -aba⁻¹
pub trait SandwichProduct {}
/// a|b
pub trait InnerProduct {}
/// k*a
pub trait ScalarProduct: Mul + Div + Sized {
  type Output;
  fn mul(self,k:f32)->Self;
  fn div(self,k:f32)->Self;
  // fn mul_assign(&mut self, s:f32);
  // fn div_assign(&mut self, s:f32);
}

pub trait SumProduct: Add + Sub + Sized {
  type Output=Self;
  fn add(self,k:Self)->Self;
  // fn sub(self,k:Self)->Self;
}
