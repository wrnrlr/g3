/// Can be specified as 0xAARRGGBB
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Color(pub u32);

impl Color {
  pub const TRANSPARENT: Self = Self(0x0);
  pub const BLACK: Self = Self(0xFF000000);
  pub const WHITE: Self = Self(0xFFFFFFFF);
  pub const RED: Self = Self(0xFFFF0000);
  pub const GREEN: Self = Self(0xFF00FF00);
  pub const BLUE: Self = Self(0xFF0000FF);
  pub const GREY:Self = Self(0xFF888888);
  pub const CYAN:Self = Self(0xFF00FFFF);
  pub const MAGENTA:Self = Self(0xFFFF00FF);
  pub const YELLOW:Self = Self(0xFFFFFF00);
  pub const ORANGE:Self = Self(0xFFF89F00);
  pub const PINK:Self = Self(0xFFCCBBFF);
}

