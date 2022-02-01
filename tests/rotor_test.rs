#[cfg(test)]
mod tests {
  use g3::*;

  #[test] fn rotor_constrained() {
    let r1 = Rotor::new(1.0, 2.0, 3.0, 4.0);
    let r2 = r1.constrained();
    assert_eq!(r1, r2);
    let r3 = -r1;
    let r4 = r1.constrained();
    assert_eq!(r3, -r4);
  }
}
