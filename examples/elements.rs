// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use std::iter::Map;
use std::ops::Range;
use g3::{point, plane, Point, Translator, Motor, Rotor, Mirror, PI, E032, E023, E012};
use baryon::{Color};

// const BLACK:u32 = 0xFF000000;
// const WHITE:u32 = 0xFFFFFFFF;
const RED:Color = Color(0xFFFF0000);
const GREEN:Color = Color(0xFF00FF00);
const BLUE:Color = Color(0xFF0000FF);
const PINK:Color = Color(0xFFCCBBFF);
const GREY:Color = Color(0xFF888888);
const CYAN:Color = Color(0xFF00FFFF);
const MAGENTA:Color = Color(0xFFFF00FF);
const YELLOW:Color = Color(0xFFFF00FF);

fn align(p:Point, q:Point)->Translator {
  // sqrt(ab) = (1 + ab).Normalized
  (q/p).sqrt()
}

fn lerp(m:Translator, f:f32)->Translator {
  m*f //.normalized()
}

/// Return iterator of n equally-spaced numbers between 0.0 to 1.0
fn steps(n:u32)->impl Iterator<Item = f32> {
  (0..n).map(move |i| i as f32/(n as f32-1.0))
}

fn path(m:Translator, n:u32, x:Point)->Vec<Point> {
  steps(n).map(|f| lerp(m, f) (x)).collect()
}

fn align2(p:Point, q:Point)->Motor {
  // sqrt(ab) = (1 + ab).Normalized
  (q.normalized() / p.normalized()).sqrt().into()
}

fn lerp2(m:Motor, f:f32)->Motor {
  (f*m)
}

fn path2(m:Motor, n:u32, x:Point)->Vec<Point> {
  steps(n).map(|f| lerp2(m, f) (x)).collect()
}

fn path3(n:u32, x:Point)->Vec<Point> {
  steps(n).map(|f| Rotor::new(-PI*2.0*f,0.0,0.0,1.0) (x)).collect()
}

fn main() {
  let mut mr = Mirror::new();

  let a = point(0.0, 1.0, 0.0);
  // let e1 = plane(1.0,-1.0,1.0,0.0);
  let b = point(1.0,-1.0,-1.0);

  // path(align(a, b), 8, a).iter().for_each(|x|  mr.vertex(*x, RED)); // Translator
  path2(align2(a, b), 12, a).iter().for_each(|x|  mr.vertex(*x, CYAN)); // Motor

  // path3(8, a).iter().for_each(|x|  mr.vertex(*x, GREY)); // Rotor

  // mr.vertex(point(0.0, 0.0, 0.0), Color::BLACK_OPAQUE); // center

  mr.vertex(((b/a).sqrt()*0.5)(a), YELLOW);
  let t = (b/a).sqrt();
  steps(4).for_each(|f| mr.vertex((t*f)(a), GREY));
  mr.vertex(a, GREEN);
  mr.vertex(b, BLUE);

  let a_to_b = align(a,b);
  let at = a_to_b(a);
  // mr.vertex(at, BLUE);

  println!("a: {}", a);
  // println!("e1: {}", e1);
  println!("b: {}", b);
  println!("a_to_b: {:?}", a_to_b);
  println!("a transformed :{}", at);
  println!("true??? :{:?}", a_to_b(a) == b);

  // mr.face([E032,E023,E012], MAGENTA);

  mr.run();
}
