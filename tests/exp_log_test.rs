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

  #[test] fn motor_exp_log_sqrt() { todo!() }

  #[test] fn motor_slerp() { todo!() }

  #[test] fn motor_blend() { todo!() }

  #[test] fn translator_motor_log() { todo!() }

  #[test] fn ideal_motor_step() { todo!() }
}