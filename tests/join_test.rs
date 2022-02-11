// Join Operation, Regressive Product, &
// a & b = !(!a ^ !b)
// the pseudoscalar  is the identity for the regressive product

#[cfg(test)]
mod tests {
  use g3::*;

  #[test] fn z_line() {
    let p1 = point(0.0, 0.0, 0.0);
    let p2 = point(0.0, 0.0, 1.0);
    let p3 = p1 & p2;
    assert_eq!(p3.e12(), 1.0);
  }

  #[test] fn y_line() {
    let p1 = point(0.0, -1.0, 0.0);
    let p2 = point(0.0, 0.0, 0.0);
    let p3 = p1 & p2;
    assert_eq!(p3.e31(), 1.0);
  }

  #[test] fn x_line() {
    let p1 = point(-2.0, 0.0, 0.0);
    let p2 = point(-1.0, 0.0, 0.0);
    let p3 = p1 & p2;
    assert_eq!(p3.e23(), 1.0);
  }

  #[test] fn plane_construction() {
    let a = point(1.0, 3.0, 2.0);
    let b = point(-1.0, 5.0, 2.0);
    let c = point(2.0, -1.0, -4.0);
    let p = a & b & c;
    assert_eq!(p.e1() + p.e2() * 3.0 + p.e3() * 2.0 + p.e0(), 0.0);
    assert_eq!(-p.e1() + p.e2() * 5.0 + p.e3() * 2.0 + p.e0(), 0.0);
    assert_eq!(p.e1() * 2.0 - p.e2() - p.e3() * 4.0 + p.e0(), 0.0);
  }

  // TODO
  // * join_point_branch
  // * join_point_horizon
  // * join_plane_point
}
