#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(adt_const_params)]
#![feature(portable_simd)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

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

pub use dual::{Dual,dual};
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

#[cfg(feature = "glam")]
pub use {matrix::*};

#[cfg(feature = "glam")]
mod matrix;

pub mod prelude;

#[cfg(feature = "renderer")] mod render;
mod shaders;
