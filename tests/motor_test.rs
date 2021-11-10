#[cfg(test)]
mod tests {
  use g3::{Motor};

  #[test] #[ignore] fn motor_normalized() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4).normalized();
    assert_eq!((m*m.reverse()).scalar(), 1.0, "for a normalized motor m*~m = 1")
  }

  #[test] fn multiply_motor_by_scalar() {
    let m = Motor::new(0.1,0.2,0.3,0.4,0.1,0.2,0.3,0.4)*2.0;
    assert_eq!(m, Motor::new(0.2,0.4,0.6,0.8,0.2,0.4,0.6,0.8));
  }
}
