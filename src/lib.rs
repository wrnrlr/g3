#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(portable_simd)]

mod direction;
mod dual;
mod line;
mod motor;
mod plane;
mod point;
mod rotor;
mod translator;

pub mod exterior;
pub mod geometric;
pub mod inner;
pub mod sandwich;
pub mod sqrt;
pub mod util;

pub use direction::Direction;
pub use dual::{dual, Dual};
pub use line::{branch, ideal_line, line, Branch, IdealLine, Line};
pub use motor::Motor;
pub use plane::{plane, Plane};
pub use point::{point, Origin, Point};
pub use rotor::Rotor;
pub use translator::Translator;
