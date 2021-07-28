#[cfg(test)]
mod tests {
  use g3::{plane,point};
  #[test] fn geometric_product_plane_plane() {
    let p1 = plane(1.0,2.0,3.0,4.0);
    let p2 = plane(1.0,2.0,3.0,4.0);
    let _ = p1 * p2;
    todo!();
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
    todo!();
  }
  #[test] fn geometric_product_point_plane() {
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
