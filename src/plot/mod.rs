mod plugin;

pub mod camera;
pub mod mesh;

// pub use plugin::*;

use rend3::types::Mesh;
use glam::Vec3;
use itertools::Itertools;

#[derive(Clone,Copy,Debug)]
pub struct AABB {
  pub min:Vec3,
  pub max:Vec3
}

impl Default for AABB {
  fn default() -> Self {
    Self{min: Vec3::new(f32::MAX, f32::MAX, f32::MAX), max: Vec3::new(f32::MIN, f32::MIN, f32::MIN) }
  }
}

impl AABB {
  pub fn add(&mut self, aabb: AABB) {
    self.min = self.min.min(aabb.min);
    self.max = self.max.max(aabb.max);
  }
}

impl From<&Mesh> for AABB {
  fn from(mesh:&Mesh) -> Self {
    let xb = mesh.vertex_positions.iter().map(|v| v.x).minmax().into_option().unwrap();
    let yb = mesh.vertex_positions.iter().map(|v| v.y).minmax().into_option().unwrap();
    let zb = mesh.vertex_positions.iter().map(|v| v.z).minmax().into_option().unwrap();
    Self{min: Vec3::new(xb.0, yb.0, zb.0), max: Vec3::new(xb.1, yb.1, zb.1)}
  }
}

impl From<Mesh> for AABB {
  fn from(mesh:Mesh) -> Self {
    let xb = mesh.vertex_positions.iter().map(|v| v.x).minmax().into_option().unwrap();
    let yb = mesh.vertex_positions.iter().map(|v| v.y).minmax().into_option().unwrap();
    let zb = mesh.vertex_positions.iter().map(|v| v.z).minmax().into_option().unwrap();
    Self{min: Vec3::new(xb.0, yb.0, zb.0), max: Vec3::new(xb.1, yb.1, zb.1)}
  }
}

// TODO this is an bounding box not a axis aligned bounding box, dummy

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Color(pub u32);

impl Color {
  pub const BLACK: Self = Self(0x000000FF);
  pub const WHITE: Self = Self(0xFFFFFFFF);
  pub const GREY: Self = Self(0xFF888888);
  pub const RED: Self = Self(0xFF0000FF);
  pub const GREEN: Self = Self(0x00FF00FF);
  pub const BLUE: Self = Self(0x0000FFFF);
  pub const YELLOW: Self = Self(0xFF00FFFF);
  pub const CYAN: Self = Self(0xFFFF00FF);
  pub const MAGENTA: Self = Self(0x00FFFFFF);

  pub fn red(&self)->f32 { ((self.0 >> 24) & 0xff) as f32 / 255.0 }
  pub fn green(&self)->f32 { ((self.0 >> 16) & 0xff) as f32 / 255.0 }
  pub fn blue(&self)->f32 { ((self.0 >> 8) & 0xff) as f32 / 255.0 }
  pub fn alpha(&self)->f32 { ((self.0) & 0xff) as f32 / 255.0 }
}

impl Into<[f32;4]> for Color {
  fn into(self) -> [f32;4] { [self.red(), self.green(), self.blue(), self.alpha()] }
}

impl Into<[f32;4]> for &Color {
  fn into(self) -> [f32;4] { [self.red(), self.green(), self.blue(), self.alpha()] }
}

// impl Into<glam::Vec4> for Rgba {
//   fn into(self) -> glam::Vec4 { glam::Vec4::new(self.red(), self.blue(), self.green(), self.alpha()) }
// }
//
// impl Into<glam::Vec4> for &Rgba {
//   fn into(self) -> glam::Vec4 { glam::Vec4::new(self.red(), self.blue(), self.green(), self.alpha()) }
// }
//
// impl Into<glam::Vec3> for Rgba {
//   fn into(self) -> glam::Vec3 { glam::Vec3::new(self.red(), self.blue(), self.green()) }
// }

#[cfg(test)]
mod tests {
  use rend3::types::{Handedness, MeshBuilder};
  use super::*;

  #[test] fn aabb() {
    let vertices = vec!([-1.0,-1.0,-1.0].into(), [0.0,0.0,0.0].into(), [1.0,1.0,1.0].into());
    let mesh = &MeshBuilder::new(vertices, Handedness::Left).build().unwrap();
    let aabb:AABB = mesh.into();
    assert_eq!(aabb.min, [-1.0,-1.0,-1.0].into());
    assert_eq!(aabb.max, [1.0,1.0,1.0].into());
  }

  #[test] fn color() {
    assert_eq!([Color::RED.red(), Color::RED.green(), Color::RED.blue(), Color::RED.alpha()], [1.0, 0.0, 0.0, 1.0]);
    assert_eq!([Color::CYAN.red(), Color::CYAN.green(), Color::CYAN.blue(), Color::CYAN.alpha()], [0.0, 1.0, 1.0, 1.0]);
  }
}