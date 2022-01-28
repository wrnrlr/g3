#![feature(portable_simd)]
#[cfg(test)]
mod tests {
  use g3::*;

  fn approx_eq(result: [f32; 4], expected: [f32; 4]) {
    const EPSILON: f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn rotor_log_exp() {
    let r = rotor(PI * 0.5, 0.3, -3.0, 1.0);
    let b = r.log();
    let s = b.exp();
    approx_eq([r.scalar(), r.e12(), r.e31(), r.e23()], [s.scalar(), s.e12(), s.e31(), s.e23()]);
  }

  #[test] fn rotor_sqrt() {
    let r = rotor(PI/2.0, 0.3, -3.0, 1.0);
    let s = r.sqrt();
    let n = s * s;
    approx_eq([r.scalar(), r.e12(), r.e31(), r.e23()], [n.scalar(), n.e12(), n.e31(), n.e23()]);
  }

  #[test] fn motor_exp_log_sqrt() {
    let r = rotor(PI/2.0, 0.3, -3.0, 1.0);
    let t = translator(12.0, -2.0, 0.4, 1.0);
    let m1 = r * t;
    let l = m1.log();
    let m2 = l.exp();
    approx_eq([m1.scalar(), m1.e12(), m1.e31(), m1.e23()], [m2.scalar(), m2.e12(), m2.e31(), m2.e23()]);
    approx_eq([m1.e01(), m1.e02(), m1.e03(), m1.e0123()], [m2.e01(), m2.e02(), m2.e03(), m2.e0123()]);
    let m3 = m1.sqrt() * m1.sqrt();
    approx_eq([m1.scalar(), m1.e12(), m1.e31(), m1.e23()], [m3.scalar(), m3.e12(), m3.e31(), m3.e23()]);
    approx_eq([m1.e01(), m1.e02(), m1.e03(), m1.e0123()], [m3.e01(), m3.e02(), m3.e03(), m3.e0123()]);
  }

  #[test] fn motor_slerp() { todo!() }

  #[test] fn motor_blend() {
    let r1 = rotor(PI/2.0, 0.0, 0.0, 1.0);
    let t1 = translator(1.0, 0.0, 0.0, 1.0);
    let m1 = r1 * t1;
    let r2 = rotor(PI/2.0, 0.3, -3.0, 1.0);
    let t2 = translator(12.0, -2.0, 0.4, 1.0);
    let m2 = r2 * t2;
    let motion = m2 * m1.reverse();
    let step = motion.log() / 4.0;
    let motor_step = step.exp();

    // Applying motor_step 0 times to m1 is m1.
    // Applying motor_step 4 times to m1 is m2 * ~m1;
    let r = motor_step * motor_step * motor_step * motor_step * m1;
    approx_eq([r.scalar(), r.e12(), r.e31(), r.e23()], [m2.scalar(), m2.e12(), m2.e31(), m2.e23()]);
    approx_eq([r.e01(), r.e02(), r.e03(), r.e0123()], [m2.e01(), m2.e02(), m2.e03(), m2.e0123()]);
  }

  #[test] fn translator_motor_log() {
    let t = translator(1.0, 1.0, 2.0, 3.0);
    let m:Motor = t.into();
    let l = m.log();
    approx_eq([l.e01(), l.e02(), l.e03(), 0.0], [m.e01(), m.e02(), m.e03(), 0.0]);
  }

  #[test] fn ideal_motor_step() { todo!() }
}
