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

  #[test] fn meet_plane_branch() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let l = branch(1.0,2.0,3.0);
    let _a1 = p1 ^ l;
    let _a2 = l ^ p1;
    todo!();
  }

  #[test] fn meet_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let _d1 =  p ^ a;
    let _d2 =  a ^ p;
    todo!();
  }
  // Doesn't exist
  // #[test] fn meet_point_line() {
  //   let a = point(1.0, 2.0, 3.0);
  //   let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
  //   let _p1 = a ^ l;
  //   let _p2 = l ^ a;
  //   todo!();
  // }
  #[test] fn meet_line_line() {
    let l1 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let l2 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let _d = l1 ^ l2;
    todo!();
  }
  #[test] fn anti_commute() {
    let a = point(1.0, 2.0, 3.0);
    let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    // assert_eq!(a^l, b)
  }

  #[test] fn squares_to_zero() { todo!() }
}
