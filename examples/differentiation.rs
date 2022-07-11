// use g3::prelude::*;
//
// fn align(p:Point, q:Point)->Translator {
//   (q.normalized()/p.normalized()).sqrt()
// }
// fn steps(n:u32)->impl Iterator<Item = f32> {
//   (0..n).map(move|i| i as f32/(n as f32-1.0))
// }
// fn lerp(m:Translator, f:f32)->Translator {
//   (m*f) //.normalized(); TODO normalized is broken
// }
// fn path(m:Translator, n:u32, x:Point)->impl Iterator<Item = Point> {
//   steps(n).map(move|f|lerp(m, f)(x))
// }
//
// const N:u32 = 8;
//
// fn main() {
//   App::new()
//     .add_plugin(PlotPlugin)
//     .add_startup_system(|mut cmd: Cmd| {
//       let (a,b,d) = (point(-1.0,1.0,0.0), point(1.0,1.0,0.0), point(-1.0,-1.0,0.0));
//       let ad = align(a,d);
//       for i in 0..N {
//         let down = lerp(ad, i as f32/(7.0));
//         let p = down(a);
//         let q = down(b);
//         let pq = align(down(a), q);
//         path(pq, N, p).for_each(|p|{cmd.spawn_bundle((p, Color::CYAN));});
//       }})
//     .add_system(|time: Res<Time>, mut q:Q<(&mut Point, &Color)>|{
//       let t = time.seconds_since_startup() as f32;
//       for (mut p,_) in q.iter_mut() {
//         let x = p.x(); let y = p.y();
//         let z = 0.5*(t*5.0).sin()*x*x*x-0.5*t.cos()*y*y;
//         *p = point(x, y, z);
//       }
//     })
//     .run();
// }
fn main() {}