// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use g3::{point,Mirror};
use baryon::{Color};

// const BLACK:u32 = 0xFF000000;
// const WHITE:u32 = 0xFFFFFFFF;
const PINK:Color = Color(0xFFCCBBFF);

fn main() {
  let mut mr = Mirror::new();

  let a = point(0.0, 0.0, 0.0);
  let b = point(1.0, 1.0, 1.0);
  let c = point(1.0, -1.0, 2.0);
  let d = point(0.0, -1.0, 3.0);
  // let e = point(0.0, -1.0, 3.0);

  mr.vertex(a, Color::BLACK_OPAQUE);
  mr.vertex(b, Color::RED);
  mr.vertex(c, Color::GREEN);
  mr.vertex(d, Color::BLUE);

  mr.face([a, b, c], PINK);

  // mr.edge();

  mr.run();
}
