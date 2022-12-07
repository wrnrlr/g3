use glow::{HasContext, NativeBuffer, NativeVertexArray};
use crate::{Point};

pub struct Renderer {
  world: hecs::World,
  point: Program,
}

impl Renderer {
  pub fn new(gl:&glow::Context, world: hecs::World)->Self {
    Self{
      world,
      point: unsafe { Program::new(gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE) }
    }
  }
  pub fn paint(&mut self, gl: &glow::Context) {
    let mut points = vec![];
    for (id, (p,&c)) in self.world.query_mut::<(&mut Point, &i32)>() {
      points.push(*p);
    };
    println!("points: {:?}", points);
    let mesh = Mesh{positions: points};
    unsafe {gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE)};
    unsafe {gl.use_program(Some(self.point.raw))};
    // unsafe {gl.draw_arrays(glow::TRIANGLES, 0, 3 as i32)};
    mesh.vertex_attribute(gl, glow::POINTS, self.point.vao, self.point.vbo);
    // mesh.vertex_attribute(gl, glow::TRIANGLES, self.point.vao, self.point.vbo);
    unsafe {gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL)};
  }
}

struct UniformBuffer {
  model: [f32;16],
  view: [f32;16],
  projection: [f32;16],
}

impl UniformBuffer {
  fn new()->Self {
    Self{
      model: [4.045084971874737, 0.0, 2.938926261462366, 0.0, 1.7274575140626314, 4.045084971874737, -2.377641290737884, 0.0, -2.377641290737884, 2.938926261462366, 3.2725424859373686, 0.0, 0.0, 0.0, 0.0, 1.0],
      view: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -17.45006258016331, 1.0],
      projection: [0.5913978494623657, 0.0, 0.0, 0.0, 0.0, 1.0000000000000002, 0.0, 0.0, 0.0, 0.0, -1.040816326530612, -1.0, 0.0, 0.0, -2.0408163265306123, 0.0]
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
    let model = gl.get_uniform_location(raw, "projection");
    let view = gl.get_uniform_location(raw, "view");
    let projection = gl.get_uniform_location(raw, "projection");
    let vao = Some(gl.create_vertex_array().unwrap());
    let vbo = Some(gl.create_buffer().unwrap());
    unsafe {gl.bind_vertex_array(vao)};
    unsafe {gl.bind_buffer(glow::ARRAY_BUFFER, vbo)};
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, (std::mem::size_of::<f32>()*4) as i32, 0);
    gl.link_program(raw);
    Self{raw, locations: Locations{ model, view, projection }, vao, vbo}
  }

  unsafe fn load(&self, gl: &glow::Context, uniforms:&UniformBuffer) {
    // gl.use_program(Some(self.raw));
    // gl.uniform_matrix_4_f32_slice(self.locations.model.as_ref(), false, &uniforms.model);
    // gl.uniform_matrix_4_f32_slice(self.locations.view.as_ref(), false, &uniforms.view);
    // gl.uniform_matrix_4_f32_slice(self.locations.projection.as_ref(), false, &uniforms.projection);
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
    println!("{:?}", l);
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

const VERTEX_SHADER_SOURCE:&str = r#"
  layout(location=0) in vec4 in_position;
  uniform mat4 model;
  uniform mat4 view;
  uniform mat4 projection;
  const vec4 vertices[3] = vec4[3](vec4(0,1,0,1), vec4(-1,-1,0,1), vec4(1,-1,0,1));
  void main() {
      // gl_Position = projection * view * model * vec4(in_position.xyz, 1.0);
      // gl_Position = vertices[gl_VertexID];
      gl_Position = in_position;
      gl_PointSize = 500.0;
  }
"#;
const FRAGMENT_SHADER_SOURCE:&str = r#"
  precision mediump float;
  out vec4 out_color;
  void main() {
    if (dot(gl_PointCoord-0.5,gl_PointCoord-0.5)>0.25)
			discard;
		else
			out_color = vec4(1, 0, 0, 1.0 );
      // out_color = vec4(1, 0, 0, 1);
  }
"#;

