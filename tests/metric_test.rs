#[cfg(test)]
mod tests {
  use g3::*;

  const EPSILON: f32 = 0.02;

  fn approx_eq(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}", a, b);
  }

  #[test] fn measure_point_to_point() {
    let a = point(1.0, 0.0, 0.0);
    let b = point(0.0, 1.0, 0.0);
    let l = a & b;
    assert_eq!(l.squared_norm(), 2.0);
  }

  #[test] fn measure_point_to_plane() {
    //    Plane p2
    //    /
    //   / \ line perpendicular to
    //  /   \ p2 through p1
    // 0------x--------->
    //        p1
    let a = point(2.0, 0.0, 0.0);
    let p = plane(1.0, -1.0, 0.0, 0.0).normalized();
    // Distance from point p1 to plane p2
    let root_two = 2f32.sqrt();
    approx_eq((a & p).scalar().abs(), root_two);
    approx_eq((a ^ p).e0123().abs(), root_two);
  }

  #[test] fn measure_point_to_line() {
    let l = line(0.0, 1.0, 0.0, 1.0, 0.0, 0.0);
    let a = point(0.0, 1.0, 2.0);
    let d = (l & a).norm();
    approx_eq(d, 2f32.sqrt());
  }

  #[test] fn euler_angles() {
    let r1 = rotor(1.0, 1.0, 0.0, 0.0) * rotor(1.0, 0.0, 1.0, 0.0) * rotor(1.0, 0.0, 0.0, 1.0);
    let ea = EulerAngles::from(r1);
    approx_eq(ea.roll, 1.0);
    approx_eq(ea.pitch, 1.0);
    approx_eq(ea.yaw, 1.0);
    let r2:Rotor = ea.into();
    approx_eq(r1.scalar(), r2.scalar());
    approx_eq(r1.e12(), r2.e12());
    approx_eq(r1.e31(), r2.e31());
    approx_eq(r1.e23(), r2.e23());
  }

  #[test] fn euler_angles_precision() {
    let ea1 = EulerAngles{roll: 0.2*PI, pitch: 0.2*PI, yaw: 0.0};
    let r:Rotor = ea1.into();
    let ea2:EulerAngles = r.into();
    approx_eq(ea1.roll, ea2.roll);
    approx_eq(ea1.pitch, ea2.pitch);
    approx_eq(ea1.yaw, ea2.yaw);
  }
}
