// Based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_icosahedron

use g3::{*};
use baryon::{Color};

fn main() {
  let mut mr = Mirror::new();

  let mut r = &mut Rotor::new(PI/2.5, 0.0, 1.0, 0.0);
  let a = point(0.0,1.0,0.0);
  let mut b = point((1.0 - 0.5f32.atan().powf(2.0)).sqrt(), 0.5f32.atan().atan(), 0.0);
  let mut c = Rotor::new(PI/5.0, 0.0, 1.0, 0.0)(E2(b));

  mr.vertex(a, Color::BLUE);

  for _ in 0..5 {
    b = r(b);
    mr.vertex(b, Color::BLUE);
    c = r(c);
    mr.vertex(c, Color::BLUE);
    mr.vertex(E2(a), Color::BLUE);
  }

  mr.run();
}
