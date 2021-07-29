#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod dual;
mod point;
mod line;
mod plane;
mod motor;
mod rotor;
mod direction;
mod translator;

pub mod util;
pub mod sqrt;
pub mod inner;
pub mod geometric;
pub mod exterior;
pub mod sandwich;

pub use dual::{Dual,dual};
pub use point::{Point,point};
pub use line::{Line,line,IdealLine,ideal_line,Branch,branch};
pub use plane::{Plane,plane};
pub use motor::{Motor};
pub use rotor::{Rotor};
pub use direction::{Direction};
pub use translator::{Translator};
