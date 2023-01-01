// G3 is an oriented algebra where a plane has two sides,
// and reflecting a plane with itself result in switching those sides.
// a(a) = -1

// A plane `b` perpendicular to a mirror a reflects to itself:
// -ab^(-a) = b
#![feature(portable_simd)] #[cfg(test)]
mod tests {
  use g3::*;
  use std::simd::f32x4;

  // TODO motor_to_matrix_3x4
}
