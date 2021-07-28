#[cfg(test)]
mod tests {
  use g3::{plane,point};
  #[test] fn join_plane_point() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let _p2 = p1 & a;
    todo!();
  }
  #[test] fn join_point_point() {
    todo!();
  }
  #[test] fn join_line_point() {
    todo!();
  }
}
