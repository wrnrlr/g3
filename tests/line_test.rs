#![allow(unused_imports)]
#[allow(unused_variables)]

#[cfg(test)]
mod tests {
  use g3::{Plane,Line,Point,plane,line};

  #[test] fn line_constructor() {
    let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    assert_eq!(l,line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0));
  }
  #[test] fn line_eq() {
    let l1 = line(1.,2.,3.,4.,5.,6.);
    let l2 = line(6.,5.,4.,3.,2.,1.);
    assert_eq!(l1,l1);
    assert_ne!(l1,l2)
  }
  #[test] fn line_getters() {
    let l1 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
  }
  #[test] fn line_add() {
    todo!()
  }
  #[test] fn line_add_assign() {
    todo!()
  }
  #[test] fn line_sub() {
    todo!()
  }
  #[test] fn line_sub_assign() {
    todo!()
  }
  #[test] fn line_mul_scalar() {
    todo!()
  }
  #[test] fn line_mul_assign_scalar() {
    todo!()
  }
  #[test] fn line_div_scalar() {
    todo!()
  }
  #[test] fn line_div_assign_scalar() {
    todo!()
  }
  #[test] fn line_dual() {
    assert_eq!(!line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0), line(4.0, 5.0, 6.0, 1.0, 2.0, 3.0));
  }
  #[test] fn line_reverse() {
    assert_eq!(line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).reverse(), line(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0))
  }
}
