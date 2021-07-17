#[cfg(test)]
mod tests {
  use g3::{Plane,plane,Point,point};

  #[test] fn plane_constructor() {
    assert_eq!(Plane::new(1.0,2.0,3.0,4.0), plane(1.0,2.0,3.0,4.0))
  }
  #[test] fn plane_eq() {
    assert_eq!(plane(1.0,2.0,3.0,4.0), plane(1.0,2.0,3.0,4.0));
    assert_ne!(plane(1.0,2.0,3.0,4.0), plane(4.0,3.0,2.0,1.0));
  }
  #[test] fn plane_getters() {
    let p = plane(4.0,2.0,3.0,1.0);
    assert_eq!(p.x(), 4.0);
    assert_eq!(p.y(), 2.0);
    assert_eq!(p.z(), 3.0);
    assert_eq!(p.d(), 1.0);
    assert_eq!(p.e1(), 4.0);
    assert_eq!(p.e2(), 2.0);
    assert_eq!(p.e3(), 3.0);
    assert_eq!(p.e0(), 1.0); 
  }
  #[test] fn plane_reflect_plane() {
    let plane1 = Plane::new(1.0,2.0,3.0,4.0);
    let plane2 = plane1(Plane::new(1.0,2.0,3.0,4.0));
    println!("reflected plane: {plane2:?}", plane2=plane2);
    todo!();
  }
  #[test] fn plane_reflect_line() { todo!(); }
  #[test] fn plane_reflect_point() {
    let plane1 = Plane::new(1.0,2.0,3.0,4.0);
    let point1 = plane1(Point::new(1.0,2.0,3.0));
    println!("reflected point: {point1:?}", point1=point1);
    todo!();
  }
  #[test] fn plane_add() {
    assert_plane(plane(1.0,2.0,3.0,4.0)+plane(1.0,2.0,3.0,4.0), 2.0,4.0,6.0,8.0)
  }
  #[test] fn plane_add_assign() {
    let mut p = plane(1.0,2.0,3.0,4.0);
    p += plane(1.0,2.0,3.0,4.0);
    assert_plane(p, 2.0,4.0,6.0,8.0)
  }
  #[test] fn plane_sub() {
    assert_plane(plane(2.0,4.0,6.0,8.0)-plane(1.0,2.,3.,4.0), 1.0,2.0,3.0,4.0)
  }
  #[test] fn plane_sub_assign() {
    let mut p = plane(2.0,4.0,6.0,8.0);
    p -= plane(1.0,2.0,3.0,4.0);
    assert_plane(p, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_mul_scalar() {
    assert_plane(plane(1.0,2.0,3.0,4.0)*2.0, 2.0,4.0,6.0,8.0);
  }
  #[test] fn plane_mul_assign_scalar() {
    let mut p = plane(1.0,2.0,3.0,4.0);
    p *= 2.0;
    assert_plane(p, 2.0,4.0,6.0,8.0);
  }
  #[test] fn plane_div_scalar() {
    assert_plane(plane(2.0,4.0,6.0,8.0)/2.0, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_div_assign_scalar() {
    let mut p = plane(2.0,4.0,6.0,8.0);
    p /= 2.0;
    assert_plane(p, 1.0,2.0,3.0,4.0);
  }
  #[test] fn plane_negative() {
    let p = plane(1.0,2.0,3.0,4.0);
    assert_plane(-p, -1.0,-2.0,-3.0,4.0);
  }
  #[test] fn plane_normalized() { todo!(); }
  #[test] fn plane_invserse() { todo!(); }

  #[test] fn plane_inner_product_plane() {
    // p1 | p2
    todo!();
  }
  #[test] fn plane_inner_product_line() {
    // p1 | l1
    todo!();
  }
  #[test] fn plane_inner_product_point() {
    // p1 | a
    todo!();
  }
  #[test] fn plane_geometric_product_plane() {
    // p1 * p2
    todo!();
  }
  #[test] fn plane_geometric_product_line() {
    // p1 * l1
    todo!();
  }
  #[test] fn plane_geometric_product_point() {
    // p1 * a
    todo!();
  }
  #[test] fn plane_meet_plane() {
    // p1 ^ p2
    todo!();
  }
  #[test] fn plane_meet_line() {
    // p1 ^ l
    todo!();
  }
  #[test] fn plane_meet_point() {
    // p1 ^ a
    todo!();
  }
  #[test] fn plane_join_point() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let p2 = p1 & a;
    todo!();
  }
  fn assert_plane(p:Plane,x:f32,y:f32,z:f32,d:f32) {
    assert_eq!(p.x(), x);
    assert_eq!(p.y(), y);
    assert_eq!(p.z(), z);
    assert_eq!(p.d(), d);
  }
}
