use glm::Vec2;
use winit::{
  dpi::{PhysicalSize},
  event::{ElementState, ModifiersState, WindowEvent, DeviceEvent, VirtualKeyCode, MouseScrollDelta},
};

use crate::mirror::{mesh::Mesh, model::Model, backdrop::Backdrop, camera::Camera};

pub const DEPTH_FORMAT:wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub enum Reply {
  Continue,
  Redraw,
  Quit,
}

pub struct SurfaceContext {
  raw: wgpu::Surface,
  config: wgpu::SurfaceConfiguration,
}

pub struct Context {
  instance: wgpu::Instance,
  surface: Option<SurfaceContext>,
  device: wgpu::Device,
  queue: wgpu::Queue,

  model: Option<Model>,
  backdrop: Backdrop,
  camera: Camera,

  depth: (wgpu::Texture, wgpu::TextureView),
  size: PhysicalSize<u32>,

  modifiers: ModifiersState,
}

impl Context {
  pub async fn new(size: PhysicalSize<u32>, window:&winit::window::Window)->Self {

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

    let surface = unsafe { instance.create_surface(window) };

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      force_fallback_adapter: false,
      compatible_surface: Some(&surface),
    }).await.expect("Failed to find an appropriate adapter");

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).unwrap(),
      width: size.width, height: size.height,
      present_mode: wgpu::PresentMode::Mailbox};

    // Create the logical device and command queue
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default()},
        None
    ).await.expect("Failed to create device");

    surface.configure(&device, &config);

    let depth = Self::rebuild_depth_(size, &device);
    let backdrop = Backdrop::new(&device, config.format);

    let camera = Camera::new(size.width as f32, size.height as f32);

    Self {
      instance,
      surface: Some(SurfaceContext{raw: surface, config}),
      device, queue, depth, backdrop,
      model: None, camera, size,
      modifiers: ModifiersState::empty(),
    }
  }

  pub fn format(&self)->wgpu::TextureFormat {
    self.surface.as_ref().unwrap().config.format
  }

  pub fn device_event(&mut self, e: DeviceEvent) {
    if let DeviceEvent::MouseWheel { delta } = e {
      if let MouseScrollDelta::PixelDelta(p) = delta {
        self.camera.mouse_scroll(p.y as f32);
      }
    }
  }

  pub fn window_event(&mut self, e: WindowEvent) -> Reply {
    match e {
      WindowEvent::Resized(size) => {
        self.resize(size);
        Reply::Redraw
      },
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        self.resize(*new_inner_size);
        Reply::Redraw
      },
      WindowEvent::CloseRequested => Reply::Quit,
      WindowEvent::ModifiersChanged(m) => {
        self.modifiers = m;
        Reply::Continue
      },
      WindowEvent::KeyboardInput { input, .. } => {
        if self.modifiers.logo() && input.virtual_keycode == Some(VirtualKeyCode::Q) {
          Reply::Quit
        } else {
          Reply::Continue
        }
      },
      WindowEvent::MouseInput { button, state, .. } => {
        use ElementState::*;
        match state {
          Pressed => self.camera.mouse_pressed(button),
          Released => self.camera.mouse_released(button),
        }
        Reply::Continue
      }
      WindowEvent::CursorMoved { position, .. } => {
        self.camera.mouse_move(Vec2::new(position.x as f32, position.y as f32));
        Reply::Redraw
      },
      WindowEvent::MouseWheel { delta, ..} => {
        if let MouseScrollDelta::LineDelta(_, verti) = delta {
          self.camera.mouse_scroll(verti * 10.0);
        }
        Reply::Redraw
      },
      _ => Reply::Continue,
    }
  }

  fn resize(&mut self, size: PhysicalSize<u32>) {
    let surface = match self.surface {
      Some(ref mut suf) => suf,
      None => return,
    };
    if (self.size) == (size) { return; }
    surface.config.width = size.width;
    surface.config.height = size.height;
    surface.raw.configure(&self.device, &surface.config);

    self.size = size;
    self.depth = Self::rebuild_depth_(size, &self.device);
    self.camera.set_size(size.width as f32, size.height as f32);
  }

  fn rebuild_depth_(size: PhysicalSize<u32>, device: &wgpu::Device) -> (wgpu::Texture, wgpu::TextureView) {
    let desc = wgpu::TextureDescriptor {
      label: Some("Depth Texture"),
      size: wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1},
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: DEPTH_FORMAT,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING};
    let tex = device.create_texture(&desc);
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    // TODO sampler: https://sotrh.github.io/learn-wgpu/beginner/tutorial8-depth/#a-pixels-depth

    (tex, view)
  }

  // Redraw the GUI, returning true if the model was not drawn (which means
  // that the parent loop should keep calling redraw to force model load)
  pub fn redraw(&mut self) {
    // let frame = self.surface?.raw.get_current_texture().expect("Failed to acquire next swap chain texture").;
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label:Some("Cmd Encoder")});

    if let Some(surface) = &self.surface {
      let frame = surface.raw.get_current_texture().expect("Surface error");
      let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
      self.backdrop.draw(&view, &self.depth.1, &mut encoder);

      if let Some(model) = &self.model {
        model.draw(&self.camera, &self.queue, &view, &self.depth.1, &mut encoder);
      }
      self.queue.submit(Some(encoder.finish()));
      frame.present();
    }
  }
}
