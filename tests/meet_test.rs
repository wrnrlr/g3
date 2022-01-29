// The wedge product ^ (also known as the meet or outer) is
// bilinear, anti-symmetric, and extended to be associative.
// Note that a ^ a = 0

// inner product with itself is the magnitude squared
// outer product with itself is 0
// inner commutative
// outer anti-commutative

// wedge product anti-commutes a^b = -b^a
// wedge product is associative (a^b)^c = a^(b^c)

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

  #[test] fn meet_plane_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let p2 = plane(2.0, 3.0, -1.0, -2.0);
    let l = p1 ^ p2;
    assert_eq!([l.e01(), l.e02(), l.e03()], [10.0, 16.0, 2.0]);
    assert_eq!([l.e12(), l.e31(), l.e23()], [-1.0, 7.0, -11.0]);
  }

  #[test] fn meet_plane_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(0.0, 0.0, 1.0, 4.0, 1.0, -2.0);
    let a = p ^ l;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [8.0, -5.0, -14.0, 0.0]);
  }

  #[test] fn meet_plane_ideal_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = ideal_line(-2.0, 1.0, 4.0);
    let a = p ^ l;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [5.0, -10.0, 5.0, 0.0]);
  }

  #[test] fn meet_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(-2.0, 1.0, 4.0);
    let d =  p ^ a;
    assert_eq!([d.scalar(), d.e0123()], [0.0, 16.0]);
  }

  #[test] fn meet_line_plane() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(0.0, 0.0, 1.0, 4.0, 1.0, -2.0);
    let a = l ^ p;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [8.0, -5.0, -14.0, 0.0]);
  }

  #[test] fn meet_line_line() {
    let l = line(1.0, 0.0, 0.0, 3.0, 2.0, 1.0);
    let k = line(0.0, 1.0, 0.0, 4.0, 1.0, -2.0);
    let a = l ^ k;
    assert_eq!(a.e0123(), 6.0);
  }

  #[test] fn meet_line_ideal_line() {
    let l = line(1.0, 0.0, 0.0, 3.0, 2.0, 1.0);
    let i = ideal_line(-2.0, 1.0, 4.0);
    let a = l ^ i;
    assert_eq!([a.e0123(), a.scalar()], [0.0, 0.0]);
  }

  #[test] fn meet_ideal_line_plane() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let i = ideal_line(-2.0, 1.0, 4.0);
    let a = i ^ p;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [5.0, -10.0, 5.0, 0.0]);
  }

  #[test] fn meet_point_plane() {
    let a = point(-2.0, 1.0, 4.0);
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let d = a^p;
    assert_eq!(d.e0123(), -16.0);
  }

  // TODO
  // * meet_plane_branch
  // * meet_point_line
  // anti_commute
  // squares_to_zero
}
