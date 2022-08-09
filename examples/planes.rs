use g3::prelude::*;

const SAMPLE_COUNT:SampleCount = SampleCount::One;

#[derive(Default)]
struct Planes {
  camera:CameraControl,
  directional_light_handle:Option<DirectionalLightHandle>,
  planes: Vec<ObjectHandle>,
  aabb:AABB,
}

impl Planes {
  fn new()->Self { Self { ..Default::default() } }

  fn add_plane(&mut self, renderer: &Arc<Renderer>, p: Plane, color: Vec4) {
    let mesh = plane_mesh(&p);
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