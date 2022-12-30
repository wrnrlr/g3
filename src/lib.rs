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
pub use std::f32::consts::PI;
/// τ
pub const TAU:f32 = PI*2.0;

pub use dual::{Dual,dual};
pub use point::{Point,point,Origin,E012,E023,E032,ORIGIN};
pub use line::{Line,line};
pub use branch::{Branch,branch};
pub use horizon::{Horizon,horizon};
pub use plane::{Plane,plane,E0,E1,E2,E3};
pub use motor::{Motor,motor};
pub use rotor::{EulerAngles,Rotor,rotor};
pub use direction::{Direction};
pub use translator::{Translator,translator};
pub mod maths;

#[cfg(feature = "glam")]
pub use {matrix::*};

#[cfg(feature = "glam")]
mod matrix;

pub mod prelude;

#[cfg(feature = "renderer")] mod render;
mod shaders;
