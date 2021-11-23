// use baryon::{window::{Event,Button}};
// use crate::{Point,Plane,Motor};
// use mint::{Vector2,Vector3,ColumnMatrix4};

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

// fn align(p:Point,q:Point)->Motor {
//   (Motor::one() + q.normalized() / p.normalized()).normalized()
// }


// fn look_at(position:Point, target:Point, pole:Point) {
//   const e0:plane = Plane::new(0.0,0.0,0.0,1.0);
//   const ne3:plane = Plane::new(0.0,0.0,-1.0,0.0);
//   const e2:plane = Plane::new(0.0,1.0,0.0,0.0);
//   align3([!e0,!ne3,!e2], [position,target,pole]);
// }

// https://github.com/Jam3/orbit-controls
// https://catlikecoding.com/unity/tutorials/movement/orbit-camera/
// https://github.com/Formlabs/foxtrot/blob/master/gui/src/camera.rs
// https://github.com/dmnsgn/cameras

// enum MouseState {
//   Unknown,
//   Free(Vector2<f32>),
//   Rotate(Vector2<f32>),
// }

/// Camera Control
// pub struct Orbit {
//   scene:baryon::Scene,
//   cid:baryon::NodeRef,
//
//   width:f32,
//   height:f32,
//
//   pitch:f32,
//   yaw:f32,
//
//   scale:f32,
//
//   center:Vector3<f32>,
//
//   mouse:MouseState
// }

// impl Orbit {
//   pub fn new(scene:baryon::Scene, cid:baryon::NodeRef, width:f32, height:f32)->Self {
//     Self{ scene, cid, width, height, pitch:0.0, yaw:0.0, scale:0.0, center: Vector3([0f32, 0, 0]), mouse:MouseState::Unknown }
//   }

  // pub fn event(&mut self, event:&Event) {
  //   match event {
  //     Event::Resize { width, height } => self.set_size(width as f32, height as f32),
  //     Event::Pointer { position } => self.mouse_move(*position),
  //     Event::Click { button, pressed:true } => self.mouse_pressed(*button),
  //     Event::Click { button, pressed:false } => self.mouse_released(*button),
  //     Event::Scroll { delta } => self.mouse_scroll(delta.y),
  //     _ => {}
  //   }
  // }
  //
  // fn set_size(&mut self, width: f32, height: f32) {
  //   self.width = width;
  //   self.height = height;
  // }
  //
  // fn mouse_pressed(&mut self, button:Button) {
  //   // If we were previously free, then switch to panning or rotating
  //   if let MouseState::Free(pos) = &self.mouse {
  //     match button {
  //       Button::Left => Some(MouseState::Rotate(*pos)),
  //       Button::Right => Some(MouseState::Pan(*pos, self.mouse_pos(*pos))),
  //       _ => None,
  //     }.map(|m| self.mouse = m);
  //   }
  // }
  //
  // fn mouse_released(&mut self, button:Button) {
  //   match &self.mouse {
  //     MouseState::Rotate(pos) if button == Button::Left => Some(MouseState::Free(*pos)),
  //     MouseState::Pan(pos, ..) if button == Button::Right => Some(MouseState::Free(*pos)),
  //     _ => None,
  //   }.map(|m| self.mouse = m);
  // }
  //
  // fn mouse_move(&mut self, pos:Vector2<f32>) {
  //   let x_norm =  2.0 * (pos.x / self.width - 0.5);
  //   let y_norm = -2.0 * (pos.y / self.height - 0.5);
  //   let new_pos = Vector2(x_norm, y_norm);
  //
  //   match &self.mouse {
  //     MouseState::Rotate(pos) => {
  //       let delta = new_pos - *pos;
  //       self.spin(delta.x * 3.0, -delta.y * 3.0 * self.height / self.width);
  //     },
  //     _ => (),
  //   }
  //
  //   // Store new mouse position
  //   match &mut self.mouse {
  //     MouseState::Free(pos)
  //     | MouseState::Pan(pos, ..)
  //     | MouseState::Rotate(pos) => *pos = new_pos,
  //     MouseState::Unknown => self.mouse = MouseState::Free(new_pos),
  //   }
  // }
  //
  // fn mouse_scroll(&mut self, delta: f32) {
  //   if let MouseState::Free(pos) = self.mouse {
  //     self.scale(1.0 + delta / 200.0, pos);
  //   }
  // }
  //
  // fn spin(&mut self, dx: f32, dy: f32) {
  //   self.pitch += dx;
  //   self.yaw += dy;
  // }
  //
  // pub fn scale(&mut self, value: f32, pos: Vecor2<f32>){
  //   let start_pos = self.mouse_pos(pos);
  //   self.scale *= value;
  //   let end_pos = self.mouse_pos(pos);
  //
  //   let delta = start_pos - end_pos;
  //   let mut delta_mouse = (self.mat() * delta.to_homogeneous()).xyz();
  //   delta_mouse.z = 0.0;
  //
  //   self.center += (self.mat_i() * delta_mouse.to_homogeneous()).xyz();
  // }
// }
