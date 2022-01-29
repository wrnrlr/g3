// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use g3::{Translator, point, Origin};

  #[test] fn translator_from_points() {
    let a = point(1.0, 1.0, 1.0);
    let b = point(-1.0, -1.0, -1.0);
    let o = Origin::to_point();
    let a_to_b = (b.normalized() / a.normalized()).sqrt();

    // assert_eq!(a_to_b);

    assert_eq!(a_to_b(a), b, "translate a to b");
    assert_eq!((a_to_b*0.5)(a), o, "translate halfway between a and b (origin)");

    let t = Translator::new(4.0,1.0,0.0,1.0);
    assert_eq!(t(o), point(8f32.sqrt(), 0.0, 8f32.sqrt()));
  }
}
