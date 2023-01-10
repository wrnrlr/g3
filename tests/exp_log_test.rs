#![feature(portable_simd)]
#[cfg(test)]
mod tests {
  use g3::*;

  const EPSILON: f32 = 0.02;
  fn approx_eq(result: [f32; 4], expected: [f32; 4]) {
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn rotor_log_exp() {
    let r = rotor(pi * 0.5, 0.3, -3.0, 1.0);
    let b = r.log();
    let s = b.exp();
    assert!(r.approx_eq(s, EPSILON));
  }

  #[test] fn rotor_sqrt() {
    let r = rotor(pi/2.0, 0.3, -3.0, 1.0);
    let s = r.sqrt();
    let n = s * s;
    assert!(r.approx_eq(n, EPSILON));
  }

  #[test] fn motor_exp_log_sqrt() {
    let r = rotor(pi/2.0, 0.3, -3.0, 1.0);
    let t = translator(12.0, -2.0, 0.4, 1.0);
    let m1 = r * t;
    let l = m1.log();
    let m2 = l.exp();
    assert!(m1.approx_eq(m2, EPSILON));
    let m3 = m1.sqrt() * m1.sqrt();
    assert!(m1.approx_eq(m3, EPSILON));
  }

  #[test] fn motor_slerp() {
    // Construct a motor from a translator and rotor
    let r = rotor(pi/2.0, 0.3, -3.0, 1.0);
    let t = translator(12.0, -2.0, 0.4, 1.0);
    let m1 = r * t;
    let l = m1.log();
    // Divide the motor action into three equal steps
    let step = l / 3.0;
    let m_step = step.exp();
    let m2 = m_step * m_step * m_step;
    assert!(m1.approx_eq(m2, EPSILON));
  }

  #[test] fn motor_blend() {
    let r1 = rotor(pi/2.0, 0.0, 0.0, 1.0);
    let t1 = translator(1.0, 0.0, 0.0, 1.0);
    let m1 = r1 * t1;

    let r2 = rotor(pi/2.0, 0.3, -3.0, 1.0);
    let t2 = translator(12.0, -2.0, 0.4, 1.0);
    let m2 = r2 * t2;

    let motion = m2 * m1.reverse();
    let step = motion.log() / 4.0;
    let motor_step = step.exp();

    // Applying motor_step 0 times to m1 is m1.
    // Applying motor_step 4 times to m1 is m2 * ~m1;
    let r = motor_step * motor_step * motor_step * motor_step * m1;
    assert!(r.approx_eq(m2, EPSILON));
  }

  #[test] fn translator_motor_log() {
    let t = translator(1.0, 1.0, 2.0, 3.0);
    t.log();
    let m:Motor = t.into();
    let l = m.log();
    approx_eq([l.e01(), l.e02(), l.e03(), 0.0], [m.e01(), m.e02(), m.e03(), 0.0]);
  }

  #[test] fn ideal_motor_step() {
    let r1 = rotor(0.0, 0.0, 0.0, 1.0);
    let t1 = translator(1.0, 0.0, 0.0, 1.0);
    let m1 = r1 * t1;
    let step = m1.log() / 4.0;
    let motor_step = step.exp();
    // Applying motor_step 4 times should recover the translator t1 (embedded) in m1
    let m3 = motor_step * motor_step * motor_step * motor_step;
    assert!(m3.approx_eq(m1, EPSILON));
  }
}
