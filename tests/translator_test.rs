// Two translations are additive
// (1-t2e/2) (1-t1e/2)       =
// 1 - (t2+t1)e/2 + t2et1e/4 = , because e^2 = 0
// 1 - (t2+t1)e/2

#[cfg(test)]
mod tests {
  use g3::{Point, Translator};

  #[test] fn translator_from_points() {
    let a = Point::new(1.0,1.0,1.0);
    let b = Point::new(-1.0,-1.0,-1.0);
    let t = (b.normalized() / a.normalized()).sqrt();
    assert_eq!(t(a), b);
  }

  #[test] fn translator_new() {
    let origin = Point::new(0.0,0.0,0.0);
    let t = Translator::new(4.0,1.0,0.0,1.0);
    assert_eq!(t(origin), Point::new(8f32.sqrt(), 0.0, 8f32.sqrt()));
  }
}
