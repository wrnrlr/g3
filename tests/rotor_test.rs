#[cfg(test)]
mod tests {
  use std::f32::consts::PI;
  use g3::{Point, Rotor};

  #[test] fn rotor_sandwich_point() {
    // Rotate point 90 degrees
    let r = Rotor::new(-PI/2.0, 0.0, 0.0, 1.0);
    let a = Point::new(2.0, 0.0, 0.0);
    assert_eq!(r(a).normalized(), Point::new(0.0, 2.0, 0.0));
  }
}
