#[cfg(test)]
mod tests {
  use g3::{Dual,dual};

  #[test] fn dual_getters() {
    assert_eq!(dual(1.0, 2.0).scalar(), 1.0);
    assert_eq!(dual(1.0, 2.0).e0123(), 2.0)
  }
  #[test] fn dual_constructor() {
    assert_eq!(Dual::new(1.0, 2.0), dual(1.0, 2.0))
  }
  #[test] fn dual_eq() {
    assert_eq!(dual(1.0, 2.0), dual(1.0, 2.0));
    assert_ne!(dual(1.0, 2.0), dual(2.0, 4.0))
  }
  #[test] fn dual_add() {
    assert_eq!(dual(1.0, 2.0) + dual(1.0,2.0), dual(2.0, 4.0))
  }
  #[test] fn dual_add_assign() {
    let mut d = dual(1.0, 2.0);
    d += dual(1.0, 2.0);
    assert_eq!(d, dual(2.0, 4.0))
  }
  #[test] fn dual_sub() {
    assert_eq!(dual(2.0, 4.0) - dual(1.0,2.0), dual(1.0,2.0))
  }
  #[test] fn dual_sub_assign() {
    let mut d = dual(2.0, 4.0);
    d -= dual(1.0, 2.0);
    assert_eq!(d, dual(1.0, 2.0))
  }
  #[test] fn dual_mul() {
    assert_eq!(dual(1.0, 2.0) * 2.0, dual(2.0, 4.0))
  }
  #[test] fn dual_mul_assign() {
    let mut d1 = dual(1.0, 2.0);
    d1 *= 2.0;
    assert_eq!(d1, dual(2.0, 4.0));
  }
  #[test] fn dual_div() {
    assert_eq!(dual(2.0, 4.0) / 2.0, dual(1.0, 2.0))
  }
  #[test] fn dual_div_assign() {
    let mut d = dual(2.0, 4.0);
    d /= 2.0;
    assert_eq!(d, dual(1.0, 2.0))
  }
}
