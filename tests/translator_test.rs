// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use g3::{Point, Translator, point};

  const A:Point = point(1.0, 1.0, 1.0);
  const B:Point = point(-1.0, -1.0, -1.0);
  const O:Point = point(0.0, 0.0, 0.0);

  #[test] fn translator_from_points() {
    assert_eq!((B / A).sqrt()(A), B);
  }

  #[test] fn translator_new() {
    let t = Translator::new(4.0,1.0,0.0,1.0);
    assert_eq!(t(O), point(8f32.sqrt(), 0.0, 8f32.sqrt()));
  }

  #[test] fn translator_multiply_by_scalar() {
    let t = (B / A).sqrt();
    assert_eq!(O, (t*0.5)(A));
  }
}
