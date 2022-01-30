// Inner product (Dot product)
// Inner product is commutative a|b = b|a
// Inner product is not-associative (a|b)|c != a|(b|c)


#[cfg(test)]
mod tests {
  use g3::*;

  #[test] fn inner_plane_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let p2 = plane(2.0, 3.0, -1.0, -2.0);
    let f = p1 | p2;
    assert_eq!(f, 5.0);
  }

  #[test] fn inner_plane_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(0.0, 0.0, 1.0, 4.0, 1.0, -2.0);
    let q = p | l;
    assert_eq!([q.e0(), q.e1(), q.e2(), q.e3()], [-3.0, 7.0, -14.0, 7.0]);
  }

  #[test] fn inner_ideal_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let i = ideal_line(-2.0, 1.0, 4.0);
    let a = p | i;
    assert_eq!(a.e0(), -12.0);
  }

  #[test] fn inner_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(-2.0, 1.0, 4.0);
    let l = p | a;
    assert_eq!([l.e01(), l.e02(), l.e03(), l.e12(), l.e31(), l.e23()], [-5.0, 10.0, -5.0, 3.0, 2.0, 1.0]);
  }

  #[test] fn inner_line_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(0.0, 0.0, 1.0, 4.0, 1.0, -2.0);
    let p2 = l | p1;
    assert_eq!([p2.e0(), p2.e1(), p2.e2(), p2.e3()], [3.0, -7.0, 14.0, -7.0]);
  }

  #[test] fn inner_line_line() {
    let l1 = line(1.0, 0.0, 0.0, 3.0, 2.0, 1.0);
    let l2 = line(0.0, 1.0, 0.0, 4.0, 1.0, -2.0);
    let f = l1 | l2;
    assert_eq!(f, -12.0);
  }

  #[test] fn inner_product_line_line() {
    todo!();
  }
  #[test] fn inner_product_line_point() {
    todo!();
  }
  #[test] fn inner_product_plane_plane() {
    // p1 | p2
    todo!();
  }
  #[test] fn inner_product_plane_line() {
    // p1 | l1
    todo!();
  }
  #[test] fn inner_product_plane_point() {
    // p1 | a
    todo!();
  }
  #[test] fn inner_product_point_plane() {
    let _ = point(1.0, 2.0, 3.0) | plane(1.0, 2.0, 3.0, 4.0);
    todo!();
  }
  #[test] fn inner_product_point_line() {
    let _ = point(1.0, 2.0, 3.0) | line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    todo!();
  }
  #[test] fn inner_product_point_point() {
    let _ = point(1.0, 2.0, 3.0) | point(1.0, 2.0, 3.0);
    todo!();
  }
}
