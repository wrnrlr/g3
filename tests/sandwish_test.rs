#![feature(portable_simd)]
#[cfg(test)]
mod tests {
  use g3::{sandwich,plane,line,point};
  use core_simd::{f32x4};
  // G3 is an oriented algebra where a plane has two sides,
  // and reflecting a plane with itself result in switching those sides.
  // a(a) = -1

  // A plane `b` perpendicular to a mirror a reflects to itself:
  // -ab^(-a) = b

  #[test] fn simd_sandwich() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([-4.0, -3.0, -2.0, -1.0]);
    let c = sandwich::sw02(a,b);
    assert_eq!([c[0], c[1], c[2], c[3]], [9.0, 2.0, 3.0, 4.0]);
  }

  #[test] fn reflect_pane() {
    let p1 = plane(3.0, 2.0, 1.0, -1.0);
    let p2 = plane(1.0, 2.0, -1.0, -3.0);
    let p3 = p1(p2);
    assert_eq!([p3.e0(), p3.e1(), p3.e2(), p3.e3()], [30.0, 22.0, -4.0, 26.0]);
  }

  #[test] fn reflect_line() {
    let p1 = plane(3.0, 2.0, 1.0, -1.0);
    let l1 = line(1.0, -2.0, 3.0, 6.0, 5.0, -4.0);
    let l2 = p1(l1);
    assert_eq!([l2.e01(), l2.e02(), l2.e03(), l2.e12(), l2.e31(), l2.e23()], [28.0, -72.0, 32.0, 104.0, 26.0, 60.0]);
  }

  #[test] fn reflect_point() {
    let p = plane(3.0, 2.0, 1.0, -1.0);
    let a = point(4.0, -2.0, -1.0);
    let b = p(a);
    assert_eq!([b.e021(), b.e013(), b.e032(), b.e123()], [-26.0, -52.0, 20.0, 14.0]);
  }

  #[test] fn rotor_line() {todo!()}

  #[test] fn rotor_point() {todo!()}

  #[test] fn translator_point() {todo!()}

  #[test] fn translator_line() {todo!()}

  #[test] fn construct_motor() {todo!()}

  #[test] fn construct_motor_via_screw_axis() {todo!()}

  #[test] fn motor_plane() {todo!()}

  #[test] fn motor_plane_variadic() {todo!()}

  #[test] fn motor_point() {todo!()}

  #[test] fn motor_point_variadic() {todo!()}

  #[test] fn motor_line() {todo!()}

  #[test] fn motor_line_variadic() {todo!()}

  #[test] fn motor_origin() {todo!()}

  #[test] fn motor_to_matrix() {todo!()}

  #[test] fn motor_to_matrix_3x4() {todo!()}

  #[test] fn normalize_motor() {todo!()}

  #[test] fn motor_sqrt() {todo!()}

  #[test] fn rotor_sqrt() {todo!()}

  #[test] fn normalize_rotor() {todo!()}
}
