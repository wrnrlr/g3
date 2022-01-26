// Based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_icosahedron

use g3::{*};
use baryon::{Color};

fn main() {
  let mut mr = Mirror::new();

  let r = &mut Rotor::new(PI/2.5, 0.0, 1.0, 0.0);
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

  let lightpink = Color(0xFFCCCCFF);
  for _ in 0..5 {
    let b2 = r(b);
    mr.face([a,b,b2], lightpink);
    mr.face([b,b2,c], lightpink);
    b = b2;
    mr.face([c,b,r(c)], lightpink);
    let c2 = r(c);
    mr.face([c,E2(a),c2], lightpink);
    c = c2;
  }

  mr.run();
}
