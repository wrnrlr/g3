use std::sync::Arc;
use glam::Vec4;
// use baryon::{Color, geometry::{Geometry, Streams}, window::{Event, Key, Window}};
use rend3::{Renderer,types::{ObjectHandle,DirectionalLightHandle,SampleCount,Surface,TextureFormat}};
use rend3_framework::{App,DefaultRoutines};
use crate::{Point};

mod cylinder;
mod triangle;
mod orbit;
mod cube;
mod sphere;

// use triangle::triangle;
// use cylinder::cylinder;
// use orbit::Orbit;

const POINT_RADIUS:f32 = 0.1;

#[derive(Default)]
pub struct Mirror {
  object_handle:Option<ObjectHandle>,
  directional_light_handle:Option<DirectionalLightHandle>,
}

impl App for Mirror {
  const DEFAULT_SAMPLE_COUNT: SampleCount = SampleCount::One;
  fn setup(&mut self, _window: &winit::window::Window, renderer: &Arc<Renderer>, _routines: &Arc<DefaultRoutines>, _surface: &Arc<Surface>, _surface_format: TextureFormat) {
    // Create mesh and calculate smooth normals based on vertices
    // let mesh = cube::create_mesh();
    let mesh = sphere::sphere();

    // Add mesh to renderer's world.
    //
    // All handles are refcounted, so we only need to hang onto the handle until we make an object.
    let mesh_handle = renderer.add_mesh(mesh);

    // Add PBR material with all defaults except a single color.
    let material = rend3_routine::material::PbrMaterial {
      albedo: rend3_routine::material::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
      ..rend3_routine::material::PbrMaterial::default()
    };
    let material_handle = renderer.add_material(material);

    // Combine the mesh and the material with a location to give an object.
    let object = rend3::types::Object {
      mesh: mesh_handle,
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
    surface: &Arc<rend3::types::Surface>,
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
          surface: Arc::clone(surface),
        };
        // Ready up the renderer
        let (cmd_bufs, ready) = renderer.ready();

        // Lock the routines
        let pbr_routine = rend3_framework::lock(&routines.pbr);
        let tonemapping_routine = rend3_framework::lock(&routines.tonemapping);

        // Build a rendergraph
        let mut graph = rend3::RenderGraph::new();

        // Add the default rendergraph without a skybox
        rend3_routine::add_default_rendergraph(
          &mut graph,
          &ready,
          &pbr_routine,
          None,
          &tonemapping_routine,
          Self::DEFAULT_SAMPLE_COUNT,
        );

        // Dispatch a render using the built up rendergraph!
        graph.execute(renderer, frame, cmd_bufs, &ready);
      }
      // Other events we don't care about
      _ => {}
    }
  }
}

impl Mirror {
  pub fn vertex(&mut self, p:Point, col:Color) {
  }

  pub fn edge(&mut self, _e:[Point;2], col:&Color) {
  }

  pub fn face(&mut self, f:[Point;3], col:&Color) {
    // TODO: Instead of adding the same triangle in both the clockwise and counter-clockwise direction, instead use indexes.
    // let triangle_prototype1 = triangle(f).bake(&mut self.context);
    // let triangle_prototype2 = triangle([f[2],f[1],f[0]]).bake(&mut self.context);
    // self.scene.add_entity(&triangle_prototype1).component(col).component(flat).build();
    // self.scene.add_entity(&triangle_prototype2).component(col).component(flat).build();
  }

  pub fn run(self) {
    // let app = Mirror::default();
    rend3_framework::start(
      self,
      winit::window::WindowBuilder::new()
        .with_title("Mirror")
        .with_maximized(true),
    );
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Color(pub u32);

impl Color {
  pub const BLACK: Self = Self(0x000000FF);
  pub const WHITE: Self = Self(0xFFFFFFFF);
  pub const RED: Self = Self(0xFF0000FF);
  pub const GREEN: Self = Self(0x00FF00FF);
  pub const BLUE: Self = Self(0x0000FFFF);
  pub const YELLOW: Self = Self(0xFFFF00FF);
  pub const CYAN: Self = Self(0x00FFFFFF);
  pub const MAGENTA: Self = Self(0x0000FFFF);

  pub fn red(&self)->f32 { ((self.0 >> 24) & 0xff) as f32 / 255.0 }
  pub fn green(&self)->f32 { ((self.0 >> 16) & 0xff) as f32 / 255.0 }
  pub fn blue(&self)->f32 { ((self.0 >> 8) & 0xff) as f32 / 255.0 }
  pub fn alpha(&self)->f32 { ((self.0) & 0xff) as f32 / 255.0 }
}

impl Into<glam::Vec4> for Color {
  fn into(self) -> Vec4 { Vec4::new(self.red(), self.blue(), self.green(), self.alpha()) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn color() {
    assert_eq!(Color::RED.red(), 1.0);
    assert_eq!(Color::RED.green(), 0.0);
    assert_eq!(Color::RED.blue(), 0.0);
    assert_eq!(Color::RED.alpha(), 1.0);
    assert_eq!(Color::CYAN.red(), 0.0);
    assert_eq!(Color::CYAN.green(), 1.0);
    assert_eq!(Color::CYAN.blue(), 1.0);
    assert_eq!(Color::CYAN.alpha(), 1.0);
  }
}
