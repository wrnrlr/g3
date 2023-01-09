use std::f32::consts::PI;
use glow::{HasContext, NativeBuffer, NativeVertexArray};
// use winit::event::MouseButton;
use crate::*;

pub struct Renderer {
  world: hecs::World,
  point: Program,
  line: Program,
  plane: Program,
  uniforms: UniformBuffer,
  camera: Camera,
  run: Option<fn(&mut hecs::World)>
}

impl Renderer {
  pub fn new(gl:&glow::Context, world: hecs::World, run: Option<fn(&mut hecs::World)>)->Self {
    unsafe {
      gl.enable(glow::BLEND);
      gl.disable(glow::CULL_FACE);
      gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_CONSTANT_ALPHA);
    }
    Self {
      world,
      plane: unsafe { Program::new(gl, COLOR_VERTEX_SHADER, COLOR_FRAGMENT_SHADER) },
      line: unsafe { Program::new(gl, COLOR_VERTEX_SHADER, COLOR_FRAGMENT_SHADER) },
      point: unsafe { Program::new(gl, POINT_VERTEX_SHADER, POINT_FRAGMENT_SHADER) },
      uniforms: UniformBuffer::new(),
      camera: Camera::new(1000.0, 1000.0),
      run
    }
  }
  pub fn paint(&mut self, gl: &glow::Context) {
    if let Some(run) = self.run { run(&mut self.world); }
    self.draw_planes(gl);
    self.draw_lines(gl);
    self.draw_points(gl);
  }
  fn draw_planes(&mut self, gl:&glow::Context) {
    let mut planes = vec![];
    let mut colors = vec![];
    // for f in self.world.query_mut::<(&Box<dyn Fn()->Plane + Send + Sync>,)>() {
    //
    // }
    for (_id, (p,color)) in self.world.query_mut::<(&Plane, &Color)>() {
      let m = (p.normalized()* e2).sqrt();
      let a = m(point(-1.0,0.0,-1.0));
      let b = m(point(-1.0,0.0,1.0));
      let c = m(point(1.0,0.0,1.0));
      let d = m(point(1.0,0.0,-1.0));
      planes.extend_from_slice(&[a, b, c, c, d, a]);
      colors.extend_from_slice(&[*color,*color,*color,*color,*color,*color]);
    };
    unsafe {
      gl.use_program(Some(self.plane.raw));
      gl.bind_vertex_array(self.plane.vao);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.plane.vbo);
      gl.enable_vertex_attrib_array(0);
      gl.enable_vertex_attrib_array(1);
      gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, 0 as i32, 0);
      gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0 as i32, (std::mem::size_of::<f32>()*4*planes.len()) as i32);
      self.plane.load(gl, &self.uniforms);
      let mut l = vec![];
      for p in &planes { l.push([p.x(), p.y(), p.z(), p.w()]) }
      for c in &colors { l.push([c.red(), c.green(), c.blue(), c.alpha()]) }
      let buffer = bytemuck::cast_slice(&l);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.plane.vbo);
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer, glow::DYNAMIC_DRAW);
      gl.bind_vertex_array(self.plane.vao);
      gl.draw_arrays(glow::TRIANGLES, 0, planes.len() as i32);
    }
  }
  fn draw_lines(&mut self, gl:&glow::Context) {
    let size = 1.0;
    let eps = 0.001;
    let cube = &[plane(size, 0.0, 0.0, 1.0), plane(size, 0.0, 0.0, -1.0), plane(0.0, size, 0.0, 1.0), plane(0.0, size, 0.0, -1.0), plane(0.0, 0.0, size, 1.0), plane(0.0, 0.0, size, -1.0)];
    let mut points = vec![];
    let mut colors = vec![];
    for (_id, (l,c)) in self.world.query_mut::<(&Line, &Color)>() {
      cube.iter().map(|p|(*l^*p).normalized())
        .filter(|p|p.x()<=size&&p.y()<=size&&p.z()<=size)
        .for_each(|p|{points.push(p); colors.push(*c);});
    };
    unsafe {
      gl.use_program(Some(self.line.raw));
      gl.bind_vertex_array(self.line.vao);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.line.vbo);
      gl.enable_vertex_attrib_array(0);
      gl.enable_vertex_attrib_array(1);
      gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, 0 as i32, 0);
      gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0 as i32, (std::mem::size_of::<f32>()*4*points.len()) as i32);
      self.line.load(gl, &self.uniforms);
      let mut l = vec![];
      for p in &points { l.push([p.x(), p.y(), p.z(), p.w()]) }
      for c in &colors { l.push([c.red(), c.green(), c.blue(), c.alpha()]) }
      let buffer = bytemuck::cast_slice(&l);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.line.vbo);
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer, glow::DYNAMIC_DRAW);
      gl.enable(glow::PROGRAM_POINT_SIZE);
      gl.bind_vertex_array(self.line.vao);
      gl.draw_arrays(glow::LINES, 0, points.len() as i32);
    }
  }
  fn draw_points(&mut self, gl:&glow::Context) {
    let mut points = vec![];
    let mut colors = vec![];
    for (_id, (p,c)) in self.world.query_mut::<(&Point, &Color)>() {
      points.push(*p);
      colors.push(*c);
    };
    unsafe {
      gl.use_program(Some(self.point.raw));
      gl.bind_vertex_array(self.point.vao);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.point.vbo);
      gl.enable_vertex_attrib_array(0);
      gl.enable_vertex_attrib_array(1);
      gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, 0 as i32, 0);
      gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0 as i32, (std::mem::size_of::<f32>()*4*points.len()) as i32);
      self.point.load(gl, &self.uniforms);
      let mut l = vec![];
      for p in &points { l.push([p.x(), p.y(), p.z(), p.w()]) }
      for c in &colors { l.push([c.red(), c.green(), c.blue(), c.alpha()]) }
      let buffer = bytemuck::cast_slice(&l);
      gl.bind_buffer(glow::ARRAY_BUFFER, self.point.vbo);
      gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer, glow::DYNAMIC_DRAW);
      gl.enable(glow::PROGRAM_POINT_SIZE);
      gl.bind_vertex_array(self.point.vao);
      gl.draw_arrays(glow::POINTS, 0, points.len() as i32);
    }
  }
}

struct UniformBuffer {
  model: [f32;16],
  view: [f32;16],
  projection: [f32;16],
}

impl UniformBuffer {
  fn new()->Self {
    // let model = [
    //   1.0, 0.0, 0.0, 0.0,
    //   0.0, 1.0, 0.0, 0.0,
    //   0.0, 0.0, 1.0, 0.0,
    //   0.0, 0.0, 0.0, 1.0]; // move model around
    let model = <[f32;16]>::from(Rotor::new(PI/4.0, 0.0, 1.0, 0.0));
    let view = [
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, -2.0, 1.0]; // move camera around
    let projection = [
      0.6, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, -1.0, -1.0,
      0.0, 0.0, -2.0, 0.0];
    Self{
      model: model,
      view,
      projection: projection
    }
  }
}

struct Locations {
  model: Option<glow::UniformLocation>,
  view: Option<glow::UniformLocation>,
  projection: Option<glow::UniformLocation>,
}

struct Program {
  raw:glow::Program,
  locations:Locations,
  vao:Option<glow::VertexArray>,
  vbo:Option<glow::Buffer>,
}

impl Program {
  unsafe fn new(gl:&glow::Context, vertex:&str, fragment:&str)->Self {
    let raw = gl.create_program().expect("Cannot create program");
    create_shader(gl, raw, glow::VERTEX_SHADER, vertex);
    create_shader(gl, raw, glow::FRAGMENT_SHADER, fragment);
    let vao = Some(gl.create_vertex_array().unwrap());
    let vbo = Some(gl.create_buffer().unwrap());
    gl.link_program(raw);
    let model = gl.get_uniform_location(raw, "model");
    let view = gl.get_uniform_location(raw, "view");
    let projection = gl.get_uniform_location(raw, "projection");
    Self{raw, locations: Locations{ model, view, projection }, vao, vbo}
  }

  unsafe fn load(&self, gl: &glow::Context, uniforms:&UniformBuffer) {
    // gl.use_program(Some(self.raw));
    gl.uniform_matrix_4_f32_slice(self.locations.model.as_ref(), false, &uniforms.model);
    gl.uniform_matrix_4_f32_slice(self.locations.view.as_ref(), false, &uniforms.view);
    gl.uniform_matrix_4_f32_slice(self.locations.projection.as_ref(), false, &uniforms.projection);
    // gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
  }
}

struct Mesh {
  positions: Vec<Point>
}

impl Mesh {
  unsafe fn vertex_attribute(&self, gl: &glow::Context, mode: u32, vao: Option<NativeVertexArray>, vbo: Option<NativeBuffer>) {
    let mut l = vec![];
    for p in &self.positions { l.push([p.x(), p.y(), p.z(), p.w()]) }
    let buffer = bytemuck::cast_slice(&l);
    gl.bind_buffer(glow::ARRAY_BUFFER, vbo);
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer, glow::DYNAMIC_DRAW);
    // unsafe { gl.vertex_attrib_4_f32(0, 0.0, 0.0, 0.0, 1.0); }
    gl.enable(glow::PROGRAM_POINT_SIZE);
    gl.bind_vertex_array(vao);
    gl.draw_arrays(mode, 0, self.positions.len() as i32);
  }
  // fn element_attribute(&self, gl: &glow::Context) {
  //   let buffer = bytemuck::cast_slice(&self.indices);
  //   unsafe {gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buffer, glow::DYself.positionsNAMIC_DRAW)};
  // }
}

unsafe fn create_shader(gl: &glow::Context, program: glow::Program, shader_type:u32, source:&str) {
  let shader_version = if cfg!(target_arch = "wasm32") { "#version 300 es" } else { "#version 330" };
  let shader = gl.create_shader(shader_type).expect("Cannot create shader");
  gl.shader_source(shader, &format!("{}\n{}", shader_version, source));
  gl.compile_shader(shader);
  if !gl.get_shader_compile_status(shader) { panic!("Failed to compile shader: {}", gl.get_shader_info_log(shader)); }
  gl.attach_shader(program, shader);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MouseButton {
  Left(Point),
  Right(Point),
}

#[derive(Copy, Clone, Debug)]
enum MouseState {
  Unknown,
  Free(Point),
  Rotate(Point),
  Pan(Point, Point),
}

struct Camera {
  width: f32,
  height: f32,
  pitch: f32,
  yaw: f32,
  scale: f32,
  center: Point,
  mouse: MouseState,
}

impl Camera {
  pub fn new(width: f32, height: f32) -> Self {
    Camera {
      width, height,
      pitch: 0.0,
      yaw: 0.0,
      scale: 1.0,
      center: point(0.0,0.0,0.0),
      mouse: MouseState::Unknown,
    }
  }

  pub fn mouse_pressed(&mut self, button: MouseButton) {
    // If we were previously free, then switch to panning or rotating
    if let MouseState::Free(pos) = &self.mouse {
      match button {
        MouseButton::Left(pos) => Some(MouseState::Rotate(pos)),
        MouseButton::Right(pos) => Some(MouseState::Pan(pos, self.mouse_pos(pos))),
        _ => None,
      }.map(|m| self.mouse = m);
    }
  }
  pub fn mouse_released(&mut self, button: MouseButton) {
    // match &self.mouse {
    //   MouseState::Rotate(pos) => { if button == MouseButton::Left { Some(MouseState::Free(*pos)) } else { None } },
    //   MouseState::Pan(pos, ..) => { if button == MouseButton::Right { Some(MouseState::Free(*pos)) } else { None } },
    //   _ => None,
    // }.map(|m| self.mouse = m);
  }

  pub fn mat(&self) -> Motor {
    self.view() * self.model()
  }

  pub fn mat_i(&self) -> Motor {
    // (self.view_matrix() * self.model_matrix())
    //   .try_inverse()
    //   .expect("Failed to invert mouse matrix")
    self.view() * self.model()
  }

  /// Converts a normalized mouse position into 3D
  pub fn mouse_pos(&self, pos_norm: Point) -> Point {
    self.mat_i()(pos_norm)
  }

  pub fn mouse_move(&mut self, new_pos: Point) {
    let x_norm =  2.0 * (new_pos.x() / self.width - 0.5);
    let y_norm = -2.0 * (new_pos.y() / self.height - 0.5);
    let new_pos = Point::new(x_norm, y_norm, 0.0);

    // Pan or rotate depending on current mouse state
    match &self.mouse {
      MouseState::Pan(_pos, orig) => {
        let current_pos = self.mouse_pos(new_pos);
        let delta_pos = *orig - current_pos;
        self.center += delta_pos;
      },
      MouseState::Rotate(pos) => {
        let delta = new_pos - *pos;
        self.spin(delta.x() * 3.0, -delta.y() * 3.0 * self.height / self.width);
      },
      _ => (),
    }

    // Store new mouse position
    match &mut self.mouse {
      MouseState::Free(pos)
      | MouseState::Pan(pos, ..)
      | MouseState::Rotate(pos) => *pos = new_pos,
      MouseState::Unknown => self.mouse = MouseState::Free(new_pos),
    }
  }

  pub fn mouse_scroll(&mut self, delta: f32) {
    if let MouseState::Free(pos) = self.mouse {
      self.scale(1.0 + delta / 200.0, pos);
    }
  }

  pub fn fit_verts(&mut self, verts: &[Point;3]) {
    let xb = verts.iter().map(|p|(p.x(),p.x())).reduce(|a,b|(a.0.min(b.0), a.1.max(b.1))).unwrap();
    let yb = verts.iter().map(|p|(p.y(),p.y())).reduce(|a,b|(a.0.min(b.0), a.1.max(b.1))).unwrap();
    let zb = verts.iter().map(|p|(p.z(),p.z())).reduce(|a,b|(a.0.min(b.0), a.1.max(b.1))).unwrap();
    let dx = xb.1 - xb.0;
    let dy = yb.1 - yb.0;
    let dz = zb.1 - zb.0;
    self.scale = (1.0 / dx.max(dy).max(dz)) as f32;
    self.center = Point::new((xb.0 + xb.1) as f32 / 2.0,
                            (yb.0 + yb.1) as f32 / 2.0,
                            (zb.0 + zb.1) as f32 / 2.0);
  }

  pub fn set_size(&mut self, width: f32, height: f32) {
    self.width = width;
    self.height = height;
  }

  pub fn model(&self) -> Motor {
    // let i = Motor::identity();
    // // The transforms below are applied bottom-to-top when thinking about
    // // the model, i.e. it's translated, then scaled, then rotated, etc.
    //
    // // Scale to compensate for model size
    // glm::scale(&i, &Vec3::new(self.scale, self.scale, self.scale)) *
    //
    //   // Rotation!
    //   glm::rotate_x(&i, self.yaw) *
    //   glm::rotate_y(&i, self.pitch) *
    //
    //   // Recenter model
    //   glm::translate(&i, &-self.center)
    Motor::one()
  }

  /// Returns a matrix which compensates for window aspect ratio and clipping
  pub fn view(&self) -> Motor {
    // let i = Motor::identity();
    // The Z clipping range is 0-1, so push forward
    // glm::translate(&i, &Vec3::new(0.0, 0.0, 0.5)) *
      // Scale to compensate for aspect ratio and reduce Z scale to improve
      // clipping
      // glm::scale(&i, &Vec3::new(1.0, self.width / self.height, 0.1))
    Motor::one()
  }

  pub fn spin(&mut self, dx: f32, dy: f32) {
    self.pitch += dx;
    self.yaw += dy;
  }

  pub fn scale(&mut self, value: f32, pos: Point){
    let start_pos = self.mouse_pos(pos);
    self.scale *= value;
    let end_pos = self.mouse_pos(pos);

    let delta = start_pos - end_pos;
    // let mut delta_mouse = (self.mat() * delta.to_homogeneous()).xyz();
    // delta_mouse.z = 0.0;

    // self.center += (self.mat_i() * delta_mouse.to_homogeneous()).xyz();
  }
}


const COLOR_VERTEX_SHADER:&str = r#"
  layout(location=0) in vec4 in_position;
  layout(location=1) in vec4 color;
  uniform mat4 model;
  uniform mat4 view;
  uniform mat4 projection;
  out vec4 f_color;
  void main() {
      gl_Position = projection * view * model * in_position;
      f_color = color;
  }
"#;
const COLOR_FRAGMENT_SHADER:&str = r#"
  precision mediump float;
  in vec4 f_color;
  out vec4 out_color;
  void main() {
		out_color = f_color;
  }
"#;
const POINT_VERTEX_SHADER:&str = r#"
  layout(location=0) in vec4 in_position;
  layout(location=1) in vec4 color;
  uniform mat4 model;
  uniform mat4 view;
  uniform mat4 projection;
  out vec4 f_color;
  void main() {
      gl_PointSize = 20.0;
      gl_Position = projection * view * model * in_position;
      f_color = color;
  }
"#;
const POINT_FRAGMENT_SHADER:&str = r#"
  precision mediump float;
  in vec4 f_color;
  out vec4 out_color;
  void main() {
    if (dot(gl_PointCoord-0.5,gl_PointCoord-0.5)>0.25)
			discard;
		else
			out_color = f_color;
  }
"#;

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Color(pub u32);

impl Color {
  pub const BLACK: Self = Self(0x000000FF);
  pub const WHITE: Self = Self(0xFFFFFFFF);
  pub const GREY: Self = Self(0xFF888888);
  pub const RED: Self = Self(0xFF0000FF);
  pub const GREEN: Self = Self(0x00FF00FF);
  pub const BLUE: Self = Self(0x0000FFFF);
  pub const YELLOW: Self = Self(0xFFFF00FF);
  pub const CYAN: Self = Self(0x00FFFFFF);
  pub const MAGENTA: Self = Self(0xFF00FFFF);

  pub fn red(&self)->f32 { ((self.0 >> 24) & 0xff) as f32 / 255.0 }
  pub fn green(&self)->f32 { ((self.0 >> 16) & 0xff) as f32 / 255.0 }
  pub fn blue(&self)->f32 { ((self.0 >> 8) & 0xff) as f32 / 255.0 }
  pub fn alpha(&self)->f32 { ((self.0) & 0xff) as f32 / 255.0 }
}

impl Into<[f32;4]> for Color {
  fn into(self) -> [f32;4] { [self.red(), self.green(), self.blue(), self.alpha()] }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn color() {
    assert_eq!([Color::RED.red(), Color::RED.green(), Color::RED.blue(), Color::RED.alpha()], [1.0, 0.0, 0.0, 1.0]);
    assert_eq!([Color::GREEN.red(), Color::GREEN.green(), Color::GREEN.blue(), Color::GREEN.alpha()], [0.0, 1.0, 0.0, 1.0]);
    assert_eq!([Color::BLUE.red(), Color::BLUE.green(), Color::BLUE.blue(), Color::BLUE.alpha()], [0.0, 0.0, 1.0, 1.0]);
  }
}
