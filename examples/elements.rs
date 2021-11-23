// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use std::iter::Map;
use std::ops::Range;
use g3::{point, plane, Point, Translator, Motor, Mirror, Color};
// use baryon::{Color};

// const BLACK:u32 = 0xFF000000;
// const WHITE:u32 = 0xFFFFFFFF;
const PINK:Color = Color(0xFFCCBBFF);
const GREY:Color = Color(0xFF888888);
const CYAN:Color = Color(0xFF00FFFF);
const MAGENTA:Color = Color(0xFFFF00FF);
const YELLOW:Color = Color(0xFFFFFF00);
const ORANGE:Color = Color(0xFFF89F00);

fn align(p:Point, q:Point)->Motor {
  // sqrt(ab) = (1 + ab).Normalized
  (q.normalized() / p.normalized()).sqrt().into()
}

fn lerp(m:Motor, f:f32)->Motor {
  (m*(1.0-f) + m*f).normalized()
}

/// Return iterator of n equally-spaced numbers between 0.0 to 1.0
fn steps(n:u32)->impl Iterator<Item = f32> {
  (0..n).map(move |i| i as f32/(n as f32-1.0))
}

fn path(m:Motor, n:u32, x:Point)->Vec<Point> {
  steps(n).map(|f| lerp(m, f) (x)).collect()
}

fn main() {
  let mut mr = pollster::block_on(Mirror::new());

  let center = point(0.0, 0.0, 0.0);
  let a = point(0.0, 0.8, 0.0);
  let b = point(0.8, -1.0, -0.8); //

  // mr.vertex(a, Color::GREEN);
  // mr.vertex(b, Color::RED);

  let m = align(a,b);
  let t = (b.normalized() / a.normalized()).sqrt();
  let half_m = (m*0.5);
  let half_t = (t*0.5);
  // mr.vertex(half_t(a), CYAN);
  // mr.vertex(half_m(a), MAGENTA);
  // mr.vertex(center, Color::BLACK_OPAQUE);

  // path(align(a, b), 8, a).iter().for_each(|x| mr.vertex(*x, GREY));

  println!("m:      {}", m);
  println!("t:      {}", t);
  println!("half_m :{}", half_m);
  println!("half_t: {}", half_t);

  mr.run();
}

// m:      1 + 0e23 + 0e31 + 0e12 + -0.4e01 + 0.9e02 + 0.4e03 + 0e0123
// t:                               -0.4e01 + -0.4e02 + -0.4e03 + 1e0123
// half_m :0.5 + 0e23 + 0e31 + 0e12 + 0.3e01 + 0.95e02 + 0.7e03 + 0.5e0123
// half_t:                            0.3e01 + 0.3e02 + 0.3e03 + 1e0123
