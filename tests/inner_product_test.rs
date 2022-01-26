// Inner product (Dot product)
// Inner product is commutative a|b = b|a
// Inner product is not-associative (a|b)|c != a|(b|c)


#[cfg(test)]
mod tests {
  use g3::{plane,point,line};
  #[test] fn inner_product_line_plane() {
    todo!();
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
