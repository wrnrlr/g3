use rend3::util::output::OutputFrame;
use g3::prelude::*;

const SAMPLE_COUNT: SampleCount = SampleCount::One;

fn uv(x:f32,y:f32) -> Vec2 { Vec2::new(x,y) }

struct Ball {
  radius: f32,
  position: Vec3,
  velocity: Vec3,
  handle: ObjectHandle,
}

impl Ball {
  fn new(renderer: & Arc<Renderer>)->Self {
    let radius = 0.2;
    let position = vec3(0.0, 3.0, 0.0);
    let mesh = point_mesh(&point(0.0,0.0,0.0), radius);
    let mesh_handle = renderer.add_mesh(mesh);
    let material = PbrMaterial { albedo: AlbedoComponent::Value(vec4(1.0,0.0,0.0,1.0)), transparency: Transparency::Blend, ..PbrMaterial::default() };
    let material_handle = renderer.add_material(material);
    let object = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle, transform: Mat4::IDENTITY };
    let handle = renderer.add_object(object);
    Ball { radius, position, velocity: vec3(2.0, 5.0, 3.0), handle }
  }

  fn simulate(&mut self, renderer: &Arc<Renderer>, room:&Room) {
    let dt = 1.0 / 6.0; let gravity = vec3(0.0, -10.0, 0.0);
    self.velocity = self.velocity + dt * gravity;
    self.position = self.position + dt * self.velocity;

    if self.position.x < room.aabb.min.x { self.position.x = room.aabb.min.x; self.velocity.x = -self.velocity.x }
    else if self.position.x > room.aabb.max.x { self.position.x = room.aabb.max.x; self.velocity.x = -self.velocity.x }
    if self.position.z < room.aabb.min.z { self.position.z = room.aabb.min.z; self.velocity.z = -self.velocity.z }
    else if self.position.z > room.aabb.max.z { self.position.z = room.aabb.max.z; self.velocity.z = -self.velocity.z }
    if self.position.y < room.aabb.min.y+self.radius { self.position.y = room.aabb.min.y+self.radius; self.velocity.y = -self.velocity.y }

    let mut dc = renderer.data_core.lock();
    let raw = self.handle.get_raw();
    dc.object_manager.set_object_transform(raw, Mat4::from_translation(self.position));
    drop(dc);
  }
}

struct Room {
  aabb: AABB,
  floor: ObjectHandle,
  back: ObjectHandle,
  right: ObjectHandle,
  front: ObjectHandle,
  left: ObjectHandle,
}

impl Room {
  fn new(renderer: &Arc<Renderer>) -> Self {
    let width = 5.0; let height = 4.0; let depth = 3.0;
    let a = vec3(-width, 0.0, depth); let b = vec3(-width, 0.0, -depth); let c = vec3(width, 0.0, -depth); let d = vec3(width, 0.0, depth);
    let e = vec3(-width, height, depth); let f = vec3(-width, height, -depth); let g = vec3(width, height, -depth); let h = vec3(width, height, depth);

    let uvs = vec!(uv(0.0, 0.0), uv(1.0, 0.0), uv(1.0, 1.0), uv(0.0, 1.0));
    let floor_mesh = MeshBuilder::new(vec!(a,b,c,d), Handedness::Left).with_indices(vec!(0u32,2,1,3,2,0)).with_vertex_uv0(uvs).build().unwrap();
    let mesh_handle = renderer.add_mesh(floor_mesh);
    let material = PbrMaterial { albedo: AlbedoComponent::Value(vec4(0.0,1.0,0.0,1.0)), ..PbrMaterial::default() };
    let material_handle = renderer.add_material(material);
    let floor_object = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle, transform: Mat4::IDENTITY };
    let floor = renderer.add_object(floor_object);

    let wall_material = PbrMaterial { albedo: AlbedoComponent::Value(vec4(0.0, 0.0, 1.0, 1.0)), ..PbrMaterial::default() };
    let material_handle = renderer.add_material(wall_material);

    let mesh = MeshBuilder::new(vec!(a,d,h,e), Handedness::Left).with_indices(vec!(0u32,3,1,1,3,2)).build().unwrap();
    let mesh_handle = renderer.add_mesh(mesh);
    let obj = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle.clone(), transform: Mat4::IDENTITY };
    let back = renderer.add_object(obj);

    let mesh = MeshBuilder::new(vec!(c,g,h,d), Handedness::Left).with_indices(vec!(0u32,3,1,1,3,2)).build().unwrap();
    let mesh_handle = renderer.add_mesh(mesh);
    let obj = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle.clone(), transform: Mat4::IDENTITY };
    let right = renderer.add_object(obj);

    let mesh = MeshBuilder::new(vec!(g,c,b,f), Handedness::Left).with_indices(vec!(0u32,3,1,1,3,2)).build().unwrap();
    let mesh_handle = renderer.add_mesh(mesh);
    let obj = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle.clone(), transform: Mat4::IDENTITY };
    let front = renderer.add_object(obj);

    let mesh = MeshBuilder::new(vec!(b,a,e,f), Handedness::Left).with_indices(vec!(0u32,3,1,1,3,2)).build().unwrap();
    let mesh_handle = renderer.add_mesh(mesh);
    let obj = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle.clone(), transform: Mat4::IDENTITY };
    let left = renderer.add_object(obj);

    Room { aabb:AABB{min:vec3(-width,0.0,-depth),max:vec3(width, height, depth)}, floor, back, right, front, left }
  }
}

#[derive(Default)]
struct Game {
  camera: CameraControl,
  light: Option<DirectionalLightHandle>,
  ball: Option<Ball>,
  room: Option<Room>,
  aabb: AABB,
}

impl Game {
  fn new() -> Self {
    Self {..Game::default()}
  }

  fn simulate(&mut self, renderer: &Arc<Renderer>) {
    let room = &(self.room.as_ref().unwrap());
    self.ball.iter_mut().for_each(|mut b| { b.simulate(renderer, room) });
  }
}

impl App for Game {
  const HANDEDNESS: Handedness = Handedness::Left;
  fn sample_count(&self) -> SampleCount { SAMPLE_COUNT }
  fn setup(&mut self, window: &Window, renderer: &Arc<Renderer>, _routines: &Arc<DefaultRoutines>, _surface_format: TextureFormat) {
    let size = window.inner_size();
    self.light = Some(renderer.add_directional_light(DirectionalLight { color: Vec3::ONE, intensity: 10.0, direction: vec3(-1.0, -4.0, 2.0), distance: 400.0 }));
    self.room = Some(Room::new(renderer));
    self.ball = Some(Ball::new(renderer));
    self.camera.resize(size.width as f32, size.height as f32);
    self.camera.fit_verts(AABB{min:vec3(-1.0,-1.0,-1.0),max:vec3(1.0,1.0,1.0)});
  }

  fn handle_event(&mut self, w: &Window, r: &Arc<Renderer>, rs: &Arc<DefaultRoutines>, base_rendergraph: &BaseRenderGraph, s: Option<&Arc<Surface>>, res: UVec2, event: Event<'_, ()>, control_flow: impl FnOnce(ControlFlow)) {
    match event {
      Event::MainEventsCleared => w.request_redraw(),
      Event::WindowEvent { event: CloseRequested, .. } => control_flow(ControlFlow::Exit),
      Event::WindowEvent { event: CursorMoved { position, .. }, .. } => self.camera.mouse_move(position.x as f32, position.y as f32),
      Event::WindowEvent { event: MouseInput { button, state: Pressed, .. }, .. } => self.camera.mouse_pressed(button),
      Event::WindowEvent { event: MouseInput { button, state: Released, .. }, .. } => self.camera.mouse_released(button),
      Event::WindowEvent { event: TrackPad { delta, .. }, .. } => if let LineDelta(_, verti) = delta { self.camera.mouse_scroll(verti * 10.0); },
      Event::WindowEvent { event: ScaleFactorChanged { new_inner_size: size, .. }, .. } => self.camera.resize(size.width as f32, size.height as f32),
      Event::WindowEvent { event: Resized(size), .. } => self.camera.resize(size.width as f32, size.height as f32),
      Event::DeviceEvent { event: MouseWheel { delta: PixelDelta(p), .. }, .. } => self.camera.mouse_scroll(p.y as f32),
      Event::RedrawRequested(_) => {
        let frame = OutputFrame::Surface{surface:Arc::clone(s.unwrap())};
        let (cmd_bufs, ready) = r.ready();
        self.simulate(r);
        r.set_camera_data(self.camera.camera());
        let pbr_routine = rend3_framework::lock(&rs.pbr);
        let tonemapping_routine = rend3_framework::lock(&rs.tonemapping);
        let mut graph = RenderGraph::new();
        base_rendergraph.add_to_graph(&mut graph, &ready, &pbr_routine, None, &tonemapping_routine, res, self.sample_count(), Vec4::ZERO, Vec4::new(0.5, 0.5, 0.5, 1.0));
        graph.execute(r, frame, cmd_bufs, &ready);
      }
      _ => {}
    }
  }
}

pub fn main() {
  start(Game::new(), WindowBuilder::new().with_title("Ball"));
}