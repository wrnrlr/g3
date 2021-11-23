// use baryon::{Color,geometry::{Geometry,Streams},window::{Event,Key,Window,Button},pass::{Solid,SolidConfig}};
// use mint::{Vector2,Vector3};
use glm::{Vec2,Vec3,Vec4,Mat4};

use winit::{
  event::{ElementState, Event, KeyboardInput, MouseScrollDelta, MouseButton, VirtualKeyCode as Vkc, WindowEvent},
  event_loop::{EventLoop,ControlFlow},window::Window};

use crate::{Point, mirror::{context::{Context, Reply}, Color}};

// use crate::mirror::triangle::triangle;
// use crate::mirror::cylinder::cylinder;

const POINT_RADIUS:f32 = 0.1;

pub struct Mirror {
  event_loop: EventLoop<()>,
  window: winit::window::Window,
  context: Context,
}

impl Mirror {

  pub async fn new()->Self {
    let title = "Mirror";
    let width = 600;
    let height = 400;

    let event_loop = EventLoop::new();
    let size = winit::dpi::PhysicalSize{width, height};

    let window = winit::window::WindowBuilder::new()
      .with_min_inner_size(winit::dpi::Size::Logical((64, 64).into()))
      .with_inner_size(size)
      .with_title(title)
      .build(&event_loop).unwrap();

    let context = pollster::block_on(Context::new(size, &window));

    Self{event_loop, window, context}
  }

  pub fn run(mut self) {
    let Self { event_loop, window, mut context } = self;

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
      match event {
        Event::WindowEvent { event, .. } => match context.window_event(event) {
          Reply::Continue => (),
          Reply::Quit => *control_flow = ControlFlow::Exit,
          Reply::Redraw => {
            context.redraw();
            window.request_redraw();
          },
        },
        Event::RedrawRequested(_) => {
          context.redraw();
          window.request_redraw();
        },
        Event::DeviceEvent { event, .. } => context.device_event(event),
        _ => (),
      }
    })
  }

  pub fn vertex(&mut self, p:Point, col:Color) {
    // let sphere_prototype = Geometry::sphere(Streams::NORMAL, POINT_RADIUS, 4).bake(&mut self.context);
    // self.scene
    //   .add_entity(&sphere_prototype)
    //   .position([p.x(), p.y(), p.z()].into())
    //   .component(col)
    //   .build();
  }

  pub fn edge(&mut self, _e:[Point;2], col:Color) {
    // let cylinder_prototype = cylinder(Streams::NORMAL, 0.5, 1.0).bake(&mut self.context);
    // self.scene
    //   .add_entity(&cylinder_prototype)
    //   .component(col)
    //   .build();
  }

  pub fn face(&mut self, f:[Point;3], col:Color) {
    // TODO: Instead of adding the same triangle in both the clockwise and counter-clockwise direction, instead use indexes.
    // let triangle_prototype1 = triangle(f).bake(&mut self.context);
    // let triangle_prototype2 = triangle([f[2],f[1],f[0]]).bake(&mut self.context);
    // self.scene.add_entity(&triangle_prototype1).component(col).build();
    // self.scene.add_entity(&triangle_prototype2).component(col).build();
  }
}
