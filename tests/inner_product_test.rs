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

  #[test] fn inner_horizon() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let i = horizon(-2.0, 1.0, 4.0);
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

  #[test] fn inner_line_point() {
    let l = line(0.0, 0.0, 1.0, 3.0, 2.0, 1.0);
    let a = point(-2.0, 1.0, 4.0);
    let p = l | a;
    assert_eq!([p.e0(), p.e1(), p.e2(), p.e3()], [0.0, -3.0, -2.0, -1.0]);
  }

  #[test] fn inner_point_plane() {
    let a = point(-2.0, 1.0, 4.0);
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = a | p;
    assert_eq!([l.e01(), l.e02(), l.e03(), l.e12(), l.e31(), l.e23()], [-5.0, 10.0, -5.0, 3.0, 2.0, 1.0]);
  }

  #[test] fn inner_point_line() {
    let l = line(0.0, 0.0, 1.0, 3.0, 2.0, 1.0);
    let a = point(-2.0, 1.0, 4.0);
    let p = l | a;
    assert_eq!([p.e0(), p.e1(), p.e2(), p.e3()], [0.0, -3.0, -2.0, -1.0]);
  }

  #[test] fn inner_point_point() {
    let a = point(1.0, 2.0, 3.0);
    let b = point(-2.0, 1.0, 4.0);
    let f = a | b;
    assert_eq!(f, -1.0)
  }

  #[test] fn project_point_to_line() {
    let a = point(2.0, 2.0, 0.0);
    let b = point(0.0, 0.0, 0.0);
    let c = point(1.0, 0.0, 0.0);
    let l = b & c;
    let mut d = (l | a) ^ l;
    d = d.normalized();
    assert_eq!([d.e123(), d.x(), d.y(), d.z()], [1.0, 2.0, 0.0, 0.0]);
  }
}
