#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(portable_simd)]
#![feature(adt_const_params)]
#![feature(portable_simd)]
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

pub use std::f32::consts::PI;
pub const TAU:f32 = PI*2.0;

pub use dual::{Dual,dual};
pub use point::{Point,point,Origin,E012,E023,E032};
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

#[cfg(feature = "bevy")]
mod plot;

#[cfg(feature = "mirror")]
pub mod mirror;

#[cfg(feature = "bevy")]
pub use plot::*;

#[cfg(feature = "bevy")]
pub use bevy::prelude::{Plugin,Res,Time,App,Query,Without,Handle,Quat,Changed,Vec3,AlphaMode,Entity,Commands,PbrBundle,Assets,Mesh,StandardMaterial,PointLight,PointLightBundle,ResMut,Added,Transform};

#[cfg(feature = "bevy")] pub mod prelude {
  pub use bevy::prelude::{App,Commands as Cmd,Res,Time,Query as Q};
  pub use super::{PlotPlugin,Color,Point,Direction,Plane,Line,Horizon,Branch,Translator,Rotor,Motor,point,plane,line,horizon,branch,translator,rotor,motor};
}
