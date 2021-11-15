#[cfg(test)]
mod tests {
  use std::f32::consts::PI;
  use g3::{Motor, Rotor, Translator, Point};

  #[test] #[ignore] fn motor_normalized() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4).normalized();
    assert_eq!((m*m.reverse()).scalar(), 1.0, "for a normalized motor m*~m = 1")
  }

  #[test] fn motor_by_scalar() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4)*2.0;
    assert_eq!(m, Motor::new(0.2,0.4,0.6,0.8,0.2,0.4,0.6,0.8));
  }

  #[test] fn motor_from_translator() {
    let a = Point::new(2.0,0.0,0.0);
    let m = Motor::from(Translator::new(2.0,1.0,0.0,0.0));
    assert_eq!(m(a), Point::new(4.0, 0.0, 0.0));
  }

  #[test] fn motor_from_rotor() {
    // Rotate point 90 degrees
    let a = Point::new(2.0,0.0,0.0);
    let m = Motor::from(Rotor::new(-PI/2.0,0.0,0.0,1.0));
    assert_eq!(m(a).normalized(), Point::new(0.0,2.0,0.0));
  }

  #[test] fn motor_product_of_rotor_translator() {
    // FIXME
    let r = Rotor::new(-PI/2.0,0.0,0.0,1.0);
    let t = Translator::new(2.0,1.0,0.0,0.0);
    let a = Point::new(2.0,2.0,0.0);
    let m = r*t;
    assert_ne!((r*t)(a), (t*r)(a));
    assert_eq!(m(a), Point::new(2.0,2.0,0.0));
    // assert_eq!(m(a), r(t(a)));
    // assert_eq!(m(a), t(r(a)));
  }
}
