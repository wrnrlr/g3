// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use g3::{Translator, point, Point, Origin};

  fn approx_eq(result: [f32; 4], expected: [f32; 4]) {
    const EPSILON: f32 = 0.02;
    assert_eq!(result.len(), expected.len());
    for (i, a) in result.iter().enumerate() {
      let b = expected[i];
      assert!((a - b).abs() < EPSILON, "{:?} ≉ {:?}, at index {:}", result, expected, i);
    }
  }

  #[test] fn translator_from_points() {
    let a = point(1.0, 1.0, 1.0);
    let b = point(-1.0, -1.0, -1.0);
    let o = Origin::to_point();
    let a_to_b = (b.normalized() / a.normalized()).sqrt();

    // assert_eq!(a_to_b);

    // translate a to b
    let c:Point = a_to_b(a);
    approx_eq([c.e013(), c.e021(), c.e032(), c.e123()], [b.e013(), b.e021(), b.e032(), b.e123()]);
    //translate halfway between a and b (origin)
    let d:Point = (a_to_b*0.5)(a);
    approx_eq([d.e013(), d.e021(), d.e032(), d.e123()], [o.e013(), o.e021(), o.e032(), o.e123()]);

    let t = Translator::new(4.0,1.0,0.0,1.0);
    let e:Point = t(o);
    let f = point(8f32.sqrt(), 0.0, 8f32.sqrt());
    approx_eq([e.e013(), e.e021(), e.e032(), e.e123()], [f.e013(), f.e021(), f.e032(), f.e123()]);
  }
}
