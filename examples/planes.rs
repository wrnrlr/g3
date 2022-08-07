use g3::prelude::*;

const SAMPLE_COUNT:SampleCount = SampleCount::One;
const DT:f32 = 1.0 / 6.0;

#[derive(Default)]
struct Planes {
  position:Point,
  velocity:Point,
  radius:f32,
  camera:CameraControl,

  directional_light_handle:Option<DirectionalLightHandle>,

  planes: Vec<ObjectHandle>,
  aabb:AABB,
}

impl Planes {
  fn new()->Self {
    Self {
      ..Default::default()
    }
  }

  fn simulate(&mut self) {
    let g:Point = point(0.0,-10.0,0.0);
    self.velocity += g * DT;
    self.position += self.velocity * DT;
    // if self.position.x() <
  }

  fn add_plane(&mut self, renderer: &Arc<rend3::Renderer>, p: Plane, color: Vec4) {
    let mesh = plane_mesh(&p);
    let aabb: AABB = (&mesh).into();
    self.aabb.add(aabb);
    let mesh_handle = renderer.add_mesh(mesh);
    let material = PbrMaterial { albedo: AlbedoComponent::Value(color), unlit: true, ..PbrMaterial::default() };
    let material_handle = renderer.add_material(material);
    let object = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle, transform: Mat4::IDENTITY };
    self.planes.push(renderer.add_object(object));
  }

  fn add_point(&mut self, renderer: &Arc<rend3::Renderer>, p: Point, color: Vec4) {
    let mesh = point_mesh(&p, 0.1);
    let aabb: AABB = (&mesh).into();
    self.aabb.add(aabb);
    let mesh_handle = renderer.add_mesh(mesh);
    let material = PbrMaterial { albedo: AlbedoComponent::Value(color), unlit: true, ..PbrMaterial::default() };
    let material_handle = renderer.add_material(material);
    let object = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle, transform: Mat4::IDENTITY };
    self.planes.push(renderer.add_object(object));
  }
}

impl App for Planes {
  const HANDEDNESS: Handedness = Handedness::Left;
  fn sample_count(&self) -> SampleCount { SAMPLE_COUNT }
  fn setup(&mut self, window: &Window, renderer: &Arc<Renderer>, _routines: &Arc<DefaultRoutines>, _surface_format: TextureFormat) {

    self.add_plane(renderer, E1, Vec4::new(1.0, 0.0, 0.0, 0.9));
    self.add_plane(renderer, E2, Vec4::new(0.0, 1.0, 0.0, 0.9));
    self.add_plane(renderer, E3, Vec4::new(0.0, 0.0, 1.0, 0.9));
    // self.add_point(renderer, point(0.0,0.0,0.0), Vec4::new(0.0, 0.0, 0.0, 1.0));

    // let room_mesh = cloth::room_mesh();
    // let aabb:&AABB = &(&room_mesh).into();
    // let mesh_handle = renderer.add_mesh(room_mesh);
    // let material = PbrMaterial { albedo: AlbedoComponent::Value(Vec4::new(1.0, 0.0, 0.0, 1.0)), unlit:true, ..PbrMaterial::default() };
    // let material_handle = renderer.add_material(material);
    // let floor = Object { mesh_kind: ObjectMeshKind::Static(mesh_handle), material: material_handle, transform: Mat4::IDENTITY };
    // self.floor_mesh = Some(renderer.add_object(floor));

    let size = window.inner_size();
    self.camera.resize(size.width as f32, size.height as f32);
    self.camera.fit_verts(self.aabb);
    self.directional_light_handle = Some(renderer.add_directional_light(DirectionalLight{color:Vec3::ONE,intensity: 10.0,direction:vec3(-1.0, -4.0, 2.0),distance: 400.0}));
  }

  fn handle_event(&mut self, w: &Window, r: &Arc<Renderer>, rs: &Arc<DefaultRoutines>, base_rendergraph: &BaseRenderGraph, s: Option<&Arc<Surface>>, res:UVec2, event: Event<'_,()>, control_flow: impl FnOnce(ControlFlow)) {
    match event {
      Event::MainEventsCleared => w.request_redraw(),
      Event::WindowEvent{event:CloseRequested, ..} => control_flow(ControlFlow::Exit),
      Event::WindowEvent{event:CursorMoved{position,..},..} => self.camera.mouse_move(position.x as f32, position.y as f32),
      Event::WindowEvent{event:MouseInput{button,state:Pressed,..},..} => self.camera.mouse_pressed(button),
      Event::WindowEvent{event:MouseInput{button,state:Released,..},..} => self.camera.mouse_released(button),
      Event::WindowEvent{event:TrackPad{delta,..},..} => if let LineDelta(_, verti) = delta { self.camera.mouse_scroll(verti * 10.0); },
      Event::WindowEvent{event:ScaleFactorChanged{new_inner_size:size,..},..} => self.camera.resize(size.width as f32, size.height as f32),
      Event::WindowEvent{event:Resized(size),..} => self.camera.resize(size.width as f32, size.height as f32),
      Event::DeviceEvent{event:MouseWheel{delta:PixelDelta(p),..},..} => self.camera.mouse_scroll(p.y as f32),
      Event::RedrawRequested(_) => { self.camera.render(r, rs, base_rendergraph, s, res) },
      _ => {}
    }
  }
}

pub fn main() {
  start(Planes::new(), WindowBuilder::new().with_title("Planes"));
}