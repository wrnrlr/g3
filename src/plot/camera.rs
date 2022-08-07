// https://github.com/Formlabs/foxtrot/blob/master/gui/src/camera.rs

use std::sync::Arc;
use glam::{Mat4, Quat, UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};
use rend3::types::{Camera, CameraProjection, SampleCount};
use winit::event::{MouseButton};
use rend3::graph::RenderGraph;
use rend3::Renderer;
use rend3::util::output::OutputFrame;
use rend3_framework::DefaultRoutines;
use rend3_routine::base::BaseRenderGraph;
use wgpu::Surface;
use crate::plot::AABB;

#[derive(Copy, Clone, Debug)]
pub enum MouseState {
  Unknown,
  Free(Vec2),
  Rotate(Vec2),
  Pan(Vec2, Vec3),
}

#[derive(Debug)]
pub struct CameraControl {
  pub target:Vec3,
  pub rotation:Quat,
  pub pitch:f32,
  pub yaw:f32,
  pub distance:f32,

  pub sample_count:SampleCount,

  state:MouseState,
  center:Vec3,
  scale:f32,
  width:f32,
  height:f32
}

impl Default for CameraControl {
  fn default() -> Self { Self{ target: Vec3::ZERO, scale:1.0, width: 0.0, center: Vec3::ZERO, rotation:Quat::IDENTITY, pitch: 0.0, yaw: 0.0, distance: 10.0, state:MouseState::Unknown, height: 0.0, sample_count: SampleCount::One } }
}

impl CameraControl {

  pub fn new(target:Vec3) ->Self {
    Self{target, rotation:Quat::IDENTITY, distance:10.0, ..Default::default()}
  }

  pub fn resize(&mut self, width:f32, height:f32) {
    self.width = width; self.height = height;
  }

  pub fn mouse_pressed(&mut self, button: MouseButton) {
    if let MouseState::Free(pos) = &self.state {
      match button {
        MouseButton::Left => Some(MouseState::Rotate(*pos)),
        MouseButton::Right => Some(MouseState::Pan(*pos, self.mouse_pos(*pos))),
        _ => None
      }.map(|m| self.state = m);
    }
  }

  pub fn mouse_released(&mut self, button: MouseButton) {
    match &self.state {
      MouseState::Rotate(pos) if button == MouseButton::Left => Some(MouseState::Free(*pos)),
      MouseState::Pan(pos, ..) if button == MouseButton::Right => Some(MouseState::Free(*pos)),
      _ => None,
    }.map(|m| self.state = m);
  }

  pub fn mouse_move(&mut self, x:f32, y:f32) {
    let x_norm =  2.0 * (x / self.width - 0.5);
    let y_norm = -2.0 * (y / self.height - 0.5);
    let new_pos = Vec2::new(x_norm, y_norm);
    match &self.state {
      MouseState::Pan(_pos, orig) => {
        let current_pos = self.mouse_pos(new_pos);
        let delta_pos = *orig - current_pos;
        self.center += delta_pos;
      },
      MouseState::Rotate(pos) => {
        let delta = new_pos - *pos;
        self.spin(delta.x * 3.0, -delta.y * 3.0 * self.height / self.width);
      }
      _ => {}
    }
    match &mut self.state {
      MouseState::Free(pos) | MouseState::Pan(pos, ..) | MouseState::Rotate(pos) => *pos = new_pos,
      MouseState::Unknown => self.state = MouseState::Free(new_pos),
    }
  }

  pub fn mouse_scroll(&mut self, delta: f32) {
    if let MouseState::Free(pos) = self.state {
      self.scale(1.0 + delta / 200.0, pos);
    }
  }

  pub fn spin(&mut self, dx:f32, dy:f32) {
    self.pitch += dx;
    self.yaw += dy;
  }

  pub fn scale(&mut self, value: f32, pos: Vec2) {
    let start_pos = self.mouse_pos(pos);
    self.scale *= value;
    let end_pos = self.mouse_pos(pos);

    let delta = start_pos - end_pos;
    let mut delta_mouse:Vec3 = (self.mat() * Vec4::new(delta.x, delta.y, delta.z, 0.0)).xyz();
    delta_mouse.z = 0.0;

    self.center += (self.mat_i() * Vec4::new(delta_mouse.x, delta_mouse.y, delta_mouse.z, 0.0)).xyz();
  }

  pub fn model_matrix(&self)->Mat4 {
    // The transforms below are applied bottom-to-top when thinking about
    // the model, i.e. it's translated, then scaled, then rotated, etc.
    Mat4::from_scale(Vec3::new(self.scale, self.scale, self.scale)) * Mat4::from_rotation_x(self.yaw) * Mat4::from_rotation_y(self.pitch) * Mat4::from_translation(-self.center)
  }

  /// Returns a matrix which compensates for window aspect ratio and clipping
  pub fn view_matrix(&self)->Mat4 {
    // The Z clipping range is 0-1, so push forward
    // Scale to compensate for aspect ratio and reduce Z scale to improve clipping
    Mat4::from_translation(Vec3::new(0.0, 0.0, 1.5)) * Mat4::from_scale(Vec3::new(1.0, self.width / self.height, 0.1))
  }

  pub fn mat(&self) -> Mat4 { self.view_matrix() * self.model_matrix() }

  pub fn mat_i(&self) -> Mat4 { self.mat().inverse() }

  pub fn mouse_pos(&self, pos_norm: Vec2) -> Vec3 {
    (self.mat_i() * Vec4::new(pos_norm.x, pos_norm.y, 0.0, 1.0)).xyz()
  }

  pub fn fit_verts(&mut self, AABB{min,max}:AABB) {
    let dx = max.x - min.x; let dy = max.y - min.y; let dz = max.z - min.z;
    self.scale = (1.0 / dx.max(dy).max(dz)) as f32;
    self.center = Vec3::new((min.x + max.x) as f32 / 2.0, (min.y + max.y) as f32 / 2.0, (min.z + max.z) as f32 / 2.0);
    println!("aabb: min {:?}, max {:?}", min, max);
    println!("scale {:?}, center {:?}", self.scale, self.center);
  }

  pub fn camera(&self) ->Camera {
    Camera { projection: CameraProjection::Perspective { vfov: 60.0, near: 0.1 }, view:self.mat() }
  }

  pub fn render(&self, r: &Arc<Renderer>, rs: &Arc<DefaultRoutines>, base_rendergraph: &BaseRenderGraph, s: Option<&Arc<Surface>>, res:UVec2) {
    let frame = OutputFrame::Surface{surface:Arc::clone(s.unwrap())};
    let (cmd_bufs, ready) = r.ready();
    r.set_camera_data(self.camera());
    let pbr_routine = rend3_framework::lock(&rs.pbr);
    let tonemapping_routine = rend3_framework::lock(&rs.tonemapping);
    let mut graph = RenderGraph::new();
    base_rendergraph.add_to_graph(&mut graph, &ready, &pbr_routine, None, &tonemapping_routine, res, self.sample_count, Vec4::ZERO, Vec4::new(0.5, 0.5, 0.5, 1.0));
    graph.execute(r, frame, cmd_bufs, &ready);
  }
}

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;
  use rend3::types::{Handedness, MeshBuilder};
  use super::*;

  #[test] fn translation_rotation_scale() {
    let rotation = (Quat::from_rotation_x(PI) * Quat::from_rotation_y(-PI)).normalize();
    let center = Vec3::new(1.0,2.0,-3.0);
    let scale = Vec3::new(2.0,2.0,2.0);
    let trs = Mat4::from_translation(center) * Mat4::from_quat(rotation) * Mat4::from_scale(scale);
    let srt2 = Mat4::from_scale(scale) * Mat4::from_quat(rotation) * Mat4::from_translation(center);
    let srt = Mat4::from_scale_rotation_translation(scale, rotation, center);
    assert_eq!(trs, srt);
    assert_eq!(trs, srt2);
  }
}
