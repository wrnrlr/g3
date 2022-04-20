use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Component)]
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

impl Into<glam::Vec4> for Color {
  fn into(self) -> glam::Vec4 { glam::Vec4::new(self.red(), self.blue(), self.green(), self.alpha()) }
}

impl Into<glam::Vec4> for &Color {
  fn into(self) -> glam::Vec4 { glam::Vec4::new(self.red(), self.blue(), self.green(), self.alpha()) }
}

impl Into<glam::Vec3> for Color {
  fn into(self) -> glam::Vec3 { glam::Vec3::new(self.red(), self.blue(), self.green()) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn color() {
    assert_eq!([Color::RED.red(), Color::RED.green(), Color::RED.blue(), Color::RED.alpha()], [1.0, 0.0, 0.0, 1.0]);
    assert_eq!([Color::CYAN.red(), Color::CYAN.green(), Color::CYAN.blue(), Color::CYAN.alpha()], [0.0, 1.0, 1.0, 1.0]);
  }
}
