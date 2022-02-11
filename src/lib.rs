#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(portable_simd)]
#![feature(adt_const_params)]
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

pub use dual::{Dual, dual};
pub use point::*;
pub use line::{Line, line};
pub use branch::{Branch, branch};
pub use horizon::{horizon, Horizon};
pub use plane::*;
pub use motor::{Motor, motor};
pub use rotor::{EulerAngles, Rotor, rotor};
pub use direction::Direction;
pub use translator::{Translator, translator};

pub use std::f32::consts::PI;

pub mod maths;

#[cfg(feature = "mirror")] mod mirror;
#[cfg(feature = "mirror")] pub use mirror::{*};
