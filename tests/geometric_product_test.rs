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
    let m = l * l.reversed();
    approx_eq([m.scalar(), m.e23(), m.e31(), m.e12()], [1.0, 0.0, 0.0, 0.0]);
    approx_eq([m.e01(), m.e02(), m.e03(), m.e0123()], [0.0, 0.0, 0.0, 0.0]);
  }

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

  #[test] fn point_geometric_product_point() {
    todo!();
  }
  #[test] fn point_inverse_geometric_product_point() {
    todo!();
  }
  #[test] fn geometric_product_line_plane() {
    todo!();
  }
  #[test] fn geometric_product_line_line() {
    todo!();
  }
  #[test] fn geometric_product_line_point() {
    todo!();
  }
}
