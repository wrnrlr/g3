// G3 is an oriented algebra where a plane has two sides,
// and reflecting a plane with itself result in switching those sides.
// a(a) = -1

// A plane `b` perpendicular to a mirror a reflects to itself:
// -ab^(-a) = b

#![feature(portable_simd)]
#[cfg(test)]
mod tests {
  use g3::*;
  use core_simd::f32x4;
  use g3::maths;

  fn approx_eq(result:[f32; 3], expected:[f32; 3]) {
    const EPSILON:f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a-b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn simd_sandwich() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([-4.0, -3.0, -2.0, -1.0]);
    let c = maths::sw02(a, b);
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
    assert_eq!([l2.e01(), l2.e02(), l2.e03(), l2.e12(), l2.e31(), l2.e23()],
               [28.0, -72.0, 32.0, 104.0, 26.0, 60.0]);
  }

  #[test] fn reflect_point() {
    let p = plane(3.0, 2.0, 1.0, -1.0);
    let a = point(4.0, -2.0, -1.0);
    let b = p(a);
    assert_eq!([b.e021(), b.e013(), b.e032(), b.e123()], [-26.0, -52.0, 20.0, 14.0]);
  }

  #[test] fn rotor_line() {
    let r = Rotor::load_normalized([1.0, 4.0, -3.0, 2.0]);
    let l = line(-1.0, 2.0, -3.0, -6.0, 5.0, 4.0);
    let k = r(l);
    approx_eq([k.e01(), k.e02(), k.e03()], [-110.0, 20.0, 10.0]);
    approx_eq([k.e12(), k.e31(), k.e23()], [-240.0, 102.0, -36.0]);
  }

  #[test] fn rotor_point() {
    let r = rotor(PI*0.5, 0.0, 0.0, 1.0);
    let a = point(1.0, 0.0, 0.0);
    let b:Point = r(a);
    approx_eq([b.x(), b.y(), b.z()], [0f32, -1.0, 0.0]);
  }

  #[test] fn translator_point() {
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let a = point(1.0, 0.0, 0.0);
    let b = t(a);
    assert_eq!([b.x(), b.y(), b.z()], [1.0, 0.0, 1.0]);
  }

  #[test] fn translator_line() {
    let data = [0.0, -5.0, -2.0, 2.0];
    let t = Translator::load_normalized(data);
    let l = line(-1.0, 2.0, -3.0, -6.0, 5.0, 4.0);
    let k = t(l);
    assert_eq!([k.e01(),k.e02(),k.e03(),k.e12(),k.e31(),k.e23()],
               [35.0, -14.0, 71.0, 4.0, 5.0, -6.0])
  }

  #[test] fn construct_motor() {
    let r = rotor(PI * 0.5, 0.0, 0.0, 1.0);
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let m = r * t;
    let a = point(1.0, 0.0, 0.0);
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, -1.0, 1.0]);

    let m = t * r;
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, -1.0, 1.0]);

    let l = m.log();
    approx_eq([l.e23(), l.e12(), l.e31()], [0f32, 0.7854, 0.0]);
    approx_eq([l.e01(), l.e02(), l.e03()], [0f32, 0.0, -0.5]);
  }

  #[test] fn construct_motor_via_screw_axis() {
    let m = Motor::from_screw_axis(PI*0.5, 1.0, line(0.0,0.0,0.0,0.0,0.0,1.0));
    let a = point(1.0, 0.0, 0.0);
    let b = m(a);
    approx_eq([b.x(), b.y(), b.z()], [0.0, 1.0, 1.0]);
  }

  #[test] fn motor_plane() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0);
    let a = plane(3.0, 2.0, 1.0, -1.0);
    let b:Plane = m(a);
    assert_eq!([b.x(), b.y(), b.z(), b.d()], [78.0, 60.0, 54.0, 358.0]);
  }

  // #[test] fn motor_plane_variadic() {todo!()}

  #[test] fn motor_point() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0);
    let a = point(-1.0, 1.0, 2.0);
    let b = m(a);
    assert_eq!([b.x(), b.y(), b.z(), b.w()], [-12.0, -86.0, -86.0, 30.0]);
  }

  // #[test] fn motor_point_variadic() {todo!()}

  #[test] fn motor_line() {
    let m = motor(2.0, 4.0, 3.0, -1.0, -5.0, -2.0, 2.0, -3.0);
    let l = line(-1.0, 2.0, -3.0, -6.0, 5.0, 4.0);
    let k = m(l);
    approx_eq([k.e01(), k.e02(), k.e03()], [6.0, 522.0, 96.0]);
    approx_eq([k.e12(), k.e31(), k.e23()], [-214.0, -148.0, -40.0]);
  }

  // #[test] fn motor_line_variadic() {todo!()}

  #[test] fn motor_origin() {
    let r = rotor(PI * 0.5, 0.0, 0.0, 1.0);
    let t = translator(1.0, 0.0, 0.0, 1.0);
    let m = r * t;
    let p:Point = m(Origin{});
    approx_eq([p.x(), p.y(), p.z()], [0.0, 0.0, 1.0]);
  }

  // #[test] fn motor_to_matrix() {todo!()}

  // #[test] fn motor_to_matrix_3x4() {todo!()}

  #[test] fn normalize_motor() {
    let m = motor(1.0, 4.0, 3.0, 2.0, 5.0, 6.0, 7.0, 8.0).normalized();
    let norm = m * m.inverse();
    approx_eq([norm.scalar(), norm.e0123(), 0.0], [1.0, 0.0, 0.0]);
  }

  #[test] fn motor_sqrt() {
    let m = Motor::from_screw_axis(PI/2.0, 3.0, line(3.0, 1.0, 3.0, 4.0, -2.0, 1.0).normalized());
    let s = m.sqrt();
    let n = s * s;
    approx_eq([m.scalar(), m.e01(), m.e02()], [n.scalar(), n.e01(), n.e02()]);
    approx_eq([m.e03(), m.e23(), m.e31()], [n.e03(), n.e21(), n.e31()]);
    approx_eq([m.e12(), m.e0123(), 0.0], [n.e12(), n.e0123(), 0.0]);
  }

  #[test] fn rotor_sqrt() {
    let r = rotor(PI * 0.5, 1.0, 2.0, 3.0);
    let s = r.sqrt();
    let s = s * s;
    assert_eq!([s.scalar(), s.e23(), s.e31(), s.e12()], [r.scalar(), r.e23(), r.e31(), r.e12()]);
  }

  #[test] fn normalize_rotor() {
    let r = Rotor{p1: f32x4::from([4.0, -3.0, 3.0, 28.0])};
    r.normalized();
    let norm = r * r.inverse();
    approx_eq([norm.scalar(), 0.0, 0.0], [1.0, 0.0, 0.0]);
    approx_eq([norm.e12(), norm.e31(), norm.e23()], [0.0, 0.0, 0.0]);
  }
}
