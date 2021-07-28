#[cfg(test)]
mod tests {
  use g3::{Point,point};

  #[test] fn point_constructor() {
    assert_eq!(Point::new(1.0,2.0,3.0), point(1.0, 2.0, 3.0))
  }
  #[test] fn point_eq() {
    assert_eq!(point(1.0, 2.0, 3.0), point(1.0, 2.0, 3.0));
    assert_ne!(point(1.0, 2.0, 3.0), point(3.0, 2.0, 1.0));
  }
  #[test] fn point_getters() {
    let p = point(4.0, 2.0, 3.0);
    assert_eq!(p.x(), 4.0);
    assert_eq!(p.y(), 2.0);
    assert_eq!(p.z(), 3.0);
    assert_eq!(p.w(), 1.0);
    assert_eq!(p.e032(), 4.0);
    assert_eq!(p.e013(), 2.0);
    assert_eq!(p.e021(), 3.0);
    assert_eq!(p.e123(), 1.0);
  }
  #[test] fn point_add() {
    assert_point(point(1.0, 2.0, 3.0)+point(1.0, 2.0, 3.0), 2.0,4.0,6.0,2.0)
  }
  #[test] fn point_add_assign() {
    let mut p = point(1.0, 2.0, 3.0);
    p += point(1.0, 2.0, 3.0);
    assert_point(p, 2.0, 4.0, 6.0, 2.0)
  }
  #[test] fn point_sub() {
    assert_point(point(2.0,4.0,6.0)-point(1.0,2.,3.), 1.0,2.0,3.0,0.0)
  }
  #[test] fn point_sub_assign() {
    let mut p = point(2.0,4.0,6.0);
    p -= point(1.0,2.0,3.0);
    assert_point(p, 1.0,2.0,3.0,0.0);
  }
  #[test] fn point_mul() {
    assert_point(point(1.0, 2.0, 3.0)*2.0, 2.0,4.0,6.0,2.0);
  }
  #[test] fn point_mul_assign() {
    let mut p = point(1.0, 2.0, 3.0);
    p *= 2.0;
    assert_point(p, 2.0, 4.0, 6.0, 2.0);
  }
  #[test] fn point_div() {
    assert_point(point(2.0, 4.0, 6.0)/2.0, 1.0,2.0,3.0,0.5);
  }
  #[test] fn point_div_assign() {
    let mut p = point(2.0, 4.0, 6.0);
    p /= 2.0;
    assert_point(p, 1.0, 2.0, 3.0, 0.5);
  }
  #[test] fn point_negative() {
    let p = point(1.0, 2.0, 3.0);
    assert_point(-p, -1.0, -2.0, -3.0, 1.0);
  }
  // this is become the dual operation
  // #[test] fn point_not() {
  //   let p = point(1.0,2.0,3.0);
  //   assert_point(!p, -1.0,-2.0,-3.0,-1.0);
  // }
  #[test] #[ignore] fn point_normalized() {}
  #[test] #[ignore] fn point_invserse() {}

  fn assert_point(p:Point,x:f32,y:f32,z:f32,w:f32) {
    assert_eq!(p.x(),x);
    assert_eq!(p.y(),y);
    assert_eq!(p.z(),z);
    assert_eq!(p.w(),w);
  }
}
