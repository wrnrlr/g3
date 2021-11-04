use baryon::{window::{Event, Button}};
use crate::{Point,Plane,Motor};

// https://observablehq.com/@enkimute/glu-lookat-in-3d-pga#11
// fn align3(a:[Point;3], b:[Point;3])->Motor {
//   let mut m = Motor::new_scalar();
//
//   let p1:Point = m(a[0]);
//   let q1 = b[0];
//   let m = (1 + q1.normalized()/p1.normalized()).normalized() * m;
//
//   let p2 = q1 & m(a[1]);
//   let q2 = q1 & b[1];
//   let m = (1 + q2.normalized()/p2.normalized()).normalized() * m;
//
//   let p3 = q2 & m(a[2]);
//   let q3 = q2 & b[2];
//   let m = (1 + q3.normalized()/p3.normalized()).normalized() * m;
//
//   return m
// }

// fn align2(a:[Point;2], b:[Point;2])->Motor {
//   let m = align(a[0], b[0]);
//   let p2 = q1 & m(a[1]);
//   let q2 = q1 & b[1];
//   return (1 + q2.normalized()/p2.normalized()).normalized() * m;
// }

fn align(p:Point,q:Point)->Motor {
  (Motor::one() + q.normalized() / p.normalized()).normalized()
}


// fn look_at(position:Point, target:Point, pole:Point) {
//   const e0:plane = Plane::new(0.0,0.0,0.0,1.0);
//   const ne3:plane = Plane::new(0.0,0.0,-1.0,0.0);
//   const e2:plane = Plane::new(0.0,1.0,0.0,0.0);
//   align3([!e0,!ne3,!e2], [position,target,pole]);
// }

// https://github.com/Jam3/orbit-controls
// https://catlikecoding.com/unity/tutorials/movement/orbit-camera/
pub struct Orbit {
  dragging:bool,
  dragging_start:Option<mint::Vector2<f32>>

}

impl Default for Orbit {
  fn default() -> Self {
    Orbit{dragging: false, dragging_start: None}
  }
}

impl Orbit {
  pub fn event(&mut self, event:&Event, scene:&mut baryon::Scene, camera:&baryon::Camera) {
    match event {
      Event::Pointer { position } => {
        if !self.dragging { return; }
        if self.dragging_start == None {
          self.dragging_start = Some(*position);
        }
        println!("pointer: {:?}", position);
        let v = mint::Vector3{x:1.0, y:-1.0, z:0.0};
        scene[camera.node].post_rotate(v, 1.0);
      }
      Event::Click { button:Button::Left, pressed:true } => {
        self.dragging = true;
      }
      Event::Click { button:Button::Left, pressed:false } => {
        self.dragging = false;
        self.dragging_start = None;
      }
      Event::Scroll { delta } => {
        println!("scroll: {:?}", delta);
        let v = mint::Vector3{x:0.0, y:0.0, z:delta.y};
        scene[camera.node].post_move(v);
      },
      _ => {}
    }
  }
}
