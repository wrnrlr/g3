#[cfg(test)]
mod tests {
  use std::f32::consts::PI;
  use g3::{Motor, Rotor, Translator, point};

  #[test] #[ignore] fn motor_normalized() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4).normalized();
    assert_eq!((m*m.reverse()).scalar(), 1.0, "for a normalized motor m*~m = 1")
  }

  #[test] fn motor_by_scalar() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4)*2.0;
    assert_eq!(m, Motor::new(0.2,0.4,0.6,0.8,0.2,0.4,0.6,0.8));
  }

  #[test] fn motor_from_translator() {
    let a = point(2.0,0.0,0.0);
    let m = Motor::from(Translator::new(2.0,1.0,0.0,0.0));
    assert_eq!(m(a), point(4.0, 0.0, 0.0));
  }

  #[test] fn motor_from_rotor() {
    // Rotate point 90 degrees
    let a = point(2.0,0.0,0.0);
    let m:Motor = Rotor::new(-PI/2.0,0.0,0.0,1.0).into();
    assert_eq!(m(a).normalized(), point(0.0,2.0,0.0));
  }

  #[test] fn motor_constrained() {
    let m1 = Motor::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    let m2 = m1.constrained();
    assert_eq!(m1, m2);
    let m3 = -m1;
    let m4 = m1.constrained();
    assert_eq!(m3, -m4);
  }
}
