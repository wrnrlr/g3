// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use std::iter::Map;
use std::ops::Range;
use g3::{point, plane, Point, Translator, Motor, Mirror};
use baryon::{Color};

// const BLACK:u32 = 0xFF000000;
// const WHITE:u32 = 0xFFFFFFFF;
const PINK:Color = Color(0xFFCCBBFF);
const GREY:Color = Color(0xFF888888);
const CYAN:Color = Color(0xFFFFFF00);
const MAGENTA:Color = Color(0xFFFF00FF);

fn align(p:Point, q:Point)->Translator {
  // sqrt(ab) = (1 + ab).Normalized
  (q.normalized() / p.normalized()).sqrt()
}

fn lerp(m:Translator, f:f32)->Translator {
  (m*f + (1.0-f)) //.normalized()
}

/// Return iterator of n equally-spaced numbers between 0.0 to 1.0
fn steps(n:u32)->impl Iterator<Item = f32> {
  (0..n).map(move |i| i as f32/(n as f32-1.0))
}

fn path(m:Translator, n:u32, x:Point)->Vec<Point> {
  steps(n).map(|f| lerp(m, f) (x)).collect()
}

fn main() {
  let mut mr = Mirror::new();

  let a = point(0.0, 0.0, 0.0);
  let b = point(1.0, 1.0, 1.0);
  let c = point(1.0, -1.0, 2.0);
  let d = point(2.0, 0.0, 0.0);
  let e = point(-2.0, 0.0, 0.0);

  let f = point(1.0, 0.0, 0.0);
  let e1 = plane(1.0,0.0,0.0,0.0);
  let f2 = e1(f);

  path(align(f, f2), 8, f).iter().for_each(|x|  mr.vertex(*x, Color::BLUE));

  mr.vertex(a, Color::BLACK_OPAQUE);
  // mr.vertex(b, Color(0xffff0000));
  // mr.vertex(c, Color::GREEN);
  // mr.vertex(d, Color::BLUE);

  // mr.vertex(point(-1.0,0.0,0.0), GREY);

  mr.vertex(f, Color::GREEN);
  mr.vertex(f2, Color::RED);

  let t = align(f,f2);
  let af = t(f);
  mr.vertex(af, Color::BLUE);

  println!("f: {}", f);
  println!("e1: {}", e1);
  println!("f2: {}", f2);
  println!("t: {:?}", t);
  println!("af :{}", af);


  // mr.face([a, b, c], PINK);

  // mr.vertex(d, MAGENTA);
  // mr.vertex(e, MAGENTA);
  // mr.edge([d, e], CYAN);

  mr.run();
}
