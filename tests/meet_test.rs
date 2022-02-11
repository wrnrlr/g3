// The wedge product ^ (also known as the meet, exterior or outer) is
// bilinear, anti-symmetric, and extended to be associative.

#[cfg(test)]
mod tests {
  use g3::*;

  // TODO
  // * meet_plane_branch
  // * meet_point_line
  // anti_commute, a^b = -b^a
  // associative (a^b)^c = a^(b^c)
  // outer product with itself is 0, squares_to_zero, a ^ a = 0

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

  #[test] fn meet_plane_horizon() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = horizon(-2.0, 1.0, 4.0);
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

  #[test] fn meet_line_horizon() {
    let l = line(0.0, 0.0, 1.0, 3.0, 2.0, 1.0);
    let i = horizon(-2.0, 1.0, 4.0);
    let a = l ^ i;
    assert_eq!([a.e0123(), a.scalar()], [0.0, 0.0]);
  }

  #[test] fn meet_horizon_plane() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let i = horizon(-2.0, 1.0, 4.0);
    let a = i ^ p;
    assert_eq!([a.e021(), a.e013(), a.e032(), a.e123()], [5.0, -10.0, 5.0, 0.0]);
  }

  #[test] fn meet_point_plane() {
    let a = point(-2.0, 1.0, 4.0);
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let d = a^p;
    assert_eq!(d.e0123(), -16.0);
  }
}
