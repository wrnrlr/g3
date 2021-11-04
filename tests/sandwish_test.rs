#[cfg(test)]
mod tests {
  use g3::{plane,point};
  // G3 is an oriented algebra where a plane has two sides,
  // and reflecting a plane with itself result in switching those sides.
  // a(a) = -1

  // A plane `b` perpendicular to a mirror a reflects to itself:
  // -ab^(-a) = b


  #[test] fn sandwich_point_between_plane() {
    assert_eq!(plane(1.0,0.0,0.0,0.0)(point(1.0,0.0,0.0)), point(-1.0,0.0,0.0))
  }
}
