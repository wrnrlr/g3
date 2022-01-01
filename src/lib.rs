#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(portable_simd)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

mod dual;
mod point;
mod line;
mod plane;
mod motor;
mod rotor;
mod direction;
mod translator;

pub mod util;
pub mod inner;
pub mod geometric;
pub mod exterior;
pub mod sandwich;

pub use dual::{Dual, dual};
pub use point::*;
pub use line::{Branch, branch, ideal_line, IdealLine, Line, line};
pub use plane::*;
pub use motor::{Motor,motor};
pub use rotor::{Rotor,rotor};
pub use direction::Direction;
pub use translator::{Translator,translator};

pub use std::f32::consts::PI;

#[cfg(feature = "mirror")] mod mirror;
#[cfg(feature = "mirror")] pub use mirror::{*};
