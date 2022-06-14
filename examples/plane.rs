use std::sync::Arc;
use g3::{Plane,E1,E2,E3,mirror::create_plane_mesh};

const SAMPLE_COUNT: rend3::types::SampleCount = rend3::types::SampleCount::One;

#[derive(Default)]
struct CubeExample {
  object_handle: Option<rend3::types::ObjectHandle>,
  directional_light_handle: Option<rend3::types::DirectionalLightHandle>,
}

impl rend3_framework::App for CubeExample {
  const HANDEDNESS: rend3::types::Handedness = rend3::types::Handedness::Left;

  fn sample_count(&self) -> rend3::types::SampleCount {
    SAMPLE_COUNT
  }

  fn setup(
    &mut self,
    _window: &winit::window::Window,
    renderer: &Arc<rend3::Renderer>,
    _routines: &Arc<rend3_framework::DefaultRoutines>,
    _surface_format: rend3::types::TextureFormat,
  ) {

    // Create mesh and calculate smooth normals based on vertices
    let mesh = create_plane_mesh(E3);

    // Add mesh to renderer's world.
    //
    // All handles are refcounted, so we only need to hang onto the handle until we
    // make an object.
    let mesh_handle = renderer.add_mesh(mesh);

    // Add PBR material with all defaults except a single color.
    let material = rend3_routine::pbr::PbrMaterial {
      albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
      ..rend3_routine::pbr::PbrMaterial::default()
    };
    let material_handle = renderer.add_material(material);

    // Combine the mesh and the material with a location to give an object.
    let object = rend3::types::Object {
      mesh_kind: rend3::types::ObjectMeshKind::Static(mesh_handle),
      material: material_handle,
      transform: glam::Mat4::IDENTITY,
    };
    // Creating an object will hold onto both the mesh and the material
    // even if they are deleted.
    //
    // We need to keep the object handle alive.
    self.object_handle = Some(renderer.add_object(object));

    let view_location = glam::Vec3::new(3.0, 3.0, -5.0);
    let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, -0.55, 0.5, 0.0);
    let view = view * glam::Mat4::from_translation(-view_location);

    // Set camera's location
    renderer.set_camera_data(rend3::types::Camera {
      projection: rend3::types::CameraProjection::Perspective { vfov: 60.0, near: 0.1 },
      view,
    });

    // Create a single directional light
    //
    // We need to keep the directional light handle alive.
    self.directional_light_handle = Some(renderer.add_directional_light(rend3::types::DirectionalLight {
      color: glam::Vec3::ONE,
      intensity: 10.0,
      // Direction will be normalized
      direction: glam::Vec3::new(-1.0, -4.0, 2.0),
      distance: 400.0,
    }));
  }

  fn handle_event(
    &mut self,
    window: &winit::window::Window,
    renderer: &Arc<rend3::Renderer>,
    routines: &Arc<rend3_framework::DefaultRoutines>,
    base_rendergraph: &rend3_routine::base::BaseRenderGraph,
    surface: Option<&Arc<rend3::types::Surface>>,
    resolution: glam::UVec2,
    event: rend3_framework::Event<'_, ()>,
    control_flow: impl FnOnce(winit::event_loop::ControlFlow),
  ) {
    match event {
      // Close button was clicked, we should close.
      rend3_framework::Event::WindowEvent {
        event: winit::event::WindowEvent::CloseRequested,
        ..
      } => {
        control_flow(winit::event_loop::ControlFlow::Exit);
      }
      rend3_framework::Event::MainEventsCleared => {
        window.request_redraw();
      }
      // Render!
      rend3_framework::Event::RedrawRequested(_) => {
        // Get a frame
        let frame = rend3::util::output::OutputFrame::Surface {
          surface: Arc::clone(surface.unwrap()),
        };
        // Ready up the renderer
        let (cmd_bufs, ready) = renderer.ready();

        // Lock the routines
        let pbr_routine = rend3_framework::lock(&routines.pbr);
        let tonemapping_routine = rend3_framework::lock(&routines.tonemapping);

        // Build a rendergraph
        let mut graph = rend3::graph::RenderGraph::new();

        // Add the default rendergraph without a skybox
        base_rendergraph.add_to_graph(
          &mut graph,
          &ready,
          &pbr_routine,
          None,
          &tonemapping_routine,
          resolution,
          SAMPLE_COUNT,
          glam::Vec4::new(0.10, 0.05, 0.10, 1.0), // Nice scene-referred purple
        );

        // Dispatch a render using the built up rendergraph!
        graph.execute(renderer, frame, cmd_bufs, &ready);
      }
      // Other events we don't care about
      _ => {}
    }
  }
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug")))]
fn main() {
  let app = CubeExample::default();
  rend3_framework::start(
    app,
    winit::window::WindowBuilder::new()
      .with_title("cube-example")
      .with_maximized(true),
  );
}
