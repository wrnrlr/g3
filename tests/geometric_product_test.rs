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

  fn approx_eq1(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}", a, b);
  }

  #[test] fn mul_plane_plane() {
    let p1 = plane(1.0,2.0,3.0,4.0);
    let p2 = plane(2.0,3.0,-1.0,-2.0);
    let m = p1 * p2;
    approx_eq([m.scalar(), m.e12(), m.e31(), m.e23()], [5.0, -1.0, 7.0, -11.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [10.0, 16.0, 2.0, 0.0]);
    let p3:Plane = (p1/p2).sqrt()(p2);
    assert!(p3.approx_eq(p1, 0.001));
    let p1 = p1.normalized();
    let m = p1 * p2;
    approx_eq1(m.scalar(), 1.0);
  }

  #[test] fn div_plane_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let m = p1 / p1;
    approx_eq([m.scalar(), m.e12(), m.e31(), m.e23()], [1.0, 0.0, 0.0, 0.0]);
    approx_eq([m.e01(), m.e02(), m.e02(), m.e0123()], [0.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn div_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(-2.0, 1.0, 4.0);
    let m = p * a;
    approx_eq([m.scalar(), m.e01(), m.e02(), m.e03()], [0.0, -5.0, 10.0, -5.0]);
    approx_eq([m.e12(), m.e31(), m.e23(), m.e0123()], [3.0, 2.0, 1.0, 16.0]);
  }

  #[test] fn line_normalization() {
    let l = line(1.0, 2.0, 3.0, 3.0, 2.0, 1.0);
    let l = l.normalized();
    let m = l * l.reverse();
    approx_eq([m.scalar(), m.e23(), m.e31(), m.e12()], [1.0, 0.0, 0.0, 0.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [0.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn mul_branch_branch() {
    let b1 = branch(2.0, 1.0, 3.0);
    let b2 = branch(1.0, -2.0, -3.0);
    let r = b1 * b2;
    approx_eq([r.scalar(), r.e23(), r.e31(), r.e12()], [9.0, 3.0, 9.0, -5.0]);

    let b1 = b1.normalized();
    let b2 = b2.normalized();
    let b3 = (b2 * b1).sqrt().inverse()(b1); // TODO maybe reverse?
    approx_eq([b3.x(), b3.y(), b3.z(), 0.0], [b3.x(), b3.y(), b3.z(), 0.0])
  }

  #[test] fn div_branch_branch() {
    let b = branch(2.0, 1.0, 3.0);
    let r = b / b;
    approx_eq([r.scalar(), r.e23(), r.e31(), r.e12()], [1.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn mul_line_line() {
    let l1 = line(1.0, 0.0, 0.0, 3.0, 2.0, 1.0);
    let l2 = line(0.0, 1.0, 0.0, 4.0, 1.0, -2.0);
    let m = l1 * l2;
    approx_eq([m.scalar(), m.e12(), m.e31(), m.e23()], [-12.0, 5.0, -10.0, 5.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [1.0, -2.0, -4.0, 6.0]);
    let l1 = l1.normalized();
    let l2 = l2.normalized();
    let l3:Line = (l1 * l2).sqrt()(l2);
    assert(l3.approx_eq(-l1, 0.001));
  }

  #[test] fn div_line_line() {
    let l = line(1.0, -2.0, 2.0, -3.0, 3.0, -4.0);
    let m = l / l;
    approx_eq([m.scalar(), m.e12(), m.e31(), m.e23()], [1.0, 0.0, 0.0, 0.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [0.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn mul_point_plane() {
    let a = point(-2.0, 1.0, 4.0);
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let m = a * p;
    approx_eq([m.scalar(), m.e12(), m.e31(), m.e23()], [0.0, -5.0, 10.0, -5.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [3.0, 2.0, 1.0, -16.0]);
  }

  #[test] fn mul_point_point() {
    let a = point(1.0, 2.0, 3.0);
    let b = point(-2.0, 1.0, 4.0);
    let t = a * b;
    approx_eq([m.e01(), m.e02(), m.e03(), 0.0], [-3.0, -1.0, 1.0, 0.0]);
    let c = t.sqrt()(b);
    approx_eq([m.x(), m.y(), m.z(), 0.0], [1.0, 2.0, 3.0, 0.0]);
  }

  #[test] fn div_point_point() {
    let a = point(1.0, 2.0, 3.0);
    let t = a / a;
    approx_eq([m.e01(), m.e02(), m.e03(), 0.0], [0.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn div_translator_translator() {
    let t1 = translator(3.0, 1.0, -2.0, 3.0);
    let t2 = t1 / t1;
    approx_eq([m.e01(), m.e02(), m.e03(), 0.0], [0.0, 0.0, 0.0, 0.0]);
  }

  #[test] fn mul_rotor_translator() {}

  #[test] fn mul_translator_rotor() {}

  #[test] fn mul_motor_rotor() {}

  #[test] fn mul_rotor_motor() {}

  #[test] fn mul_motor_translator() {}

  #[test] fn mul_translator_motor() {}

  #[test] fn mul_motor_motor() {}

  #[test] fn div_motor_motor() {}



  // Does not exist in klein
  // #[test] fn geometric_product_plane_line() {
  //   let p1 = plane(1.0,2.0,3.0,4.0);
  //   let l1 = line(1.0,2.0,3.0,4.0, 5.0, 6.0);
  //   let _ = p1 * l1;
  //   todo!();
  // }

  #[test] fn geometric_product_plane_point() {
    let p1 = plane(1.0,2.0,3.0,4.0);
    let a1 = point(1.0, 2.0, 3.0);
    let _ = p1 * a1;
    let _ = a1 * p1;
    todo!();
  }

  // #[test] fn geometric_product_point_line() {
  //   todo!(); Does not exist
  // }
}
