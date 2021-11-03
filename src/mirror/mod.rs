use baryon::{Color, geometry::{Geometry, Streams}, window::{Event, Key, Button, Window}};
use mint;
use crate::{Point};

mod cylinder;
mod triangle;

use triangle::triangle;
use cylinder::cylinder;

const POINT_RADIUS:f32 = 0.1;

pub struct Mirror {
  window:Window,
  scene:baryon::Scene,
  context:baryon::Context,
  camera:baryon::Camera,
}

impl Mirror {

  pub fn new()->Self {
    let window = Window::new().title("Mirror").build();
    let context = pollster::block_on(baryon::Context::init().build(&window));
    let mut scene = baryon::Scene::new();

    let camera = baryon::Camera {
      projection: baryon::Projection::Perspective { fov_y: 80.0 },
      depth: 1.0..10.0,
      node: scene
        .add_node()
        .position([2.0f32, 0.0, 5.0].into())
        .look_at([0f32; 3].into(), [0f32, 0.0, 1.0].into())
        .build(),
      background: Color(0xFFFFFFFF),
    };

    Self{window, scene, context, camera}
  }

  pub fn run(self) {
    let Self { window, mut scene, mut context, camera } = self;

    let mut pass = baryon::pass::Solid::new(
      &baryon::pass::SolidConfig {
        cull_back_faces: true,
      },
      &context,
    );

    let mut orbit = Orbit::default();

    window.run(move |event| {
      orbit.event(&event, &mut scene, &camera);
      match event {
        Event::Resize { width, height } => {
          context.resize(width, height);
        }
        Event::Draw => {
          //context.present(&mut pass, &scene, &camera);
        }
        Event::Keyboard { key: Key::Escape, pressed: true } => {
          std::process::exit(0x0100);
        }
        _ => {}
      }
      context.present(&mut pass, &scene, &camera);
    })
  }

  pub fn vertex(&mut self, p:Point, col:Color) {
    let sphere_prototype = Geometry::sphere(Streams::NORMAL, POINT_RADIUS, 4).bake(&mut self.context);
    self.scene
      .add_entity(&sphere_prototype)
      .position([p.x(), p.y(), p.z()].into())
      .component(col)
      .build();
  }

  pub fn edge(&mut self, _e:[Point;2], col:Color) {
    let cylinder_prototype = cylinder(Streams::NORMAL, 0.5, 1.0).bake(&mut self.context);
    self.scene
      .add_entity(&cylinder_prototype)
      .component(col)
      .build();
  }

  pub fn face(&mut self, f:[Point;3], col:Color) {
    // TODO: Instead of adding the same triangle in both the clockwise and counter-clockwise direction, instead use indexes.
    let triangle_prototype1 = triangle(f).bake(&mut self.context);
    let triangle_prototype2 = triangle([f[2],f[1],f[0]]).bake(&mut self.context);
    self.scene.add_entity(&triangle_prototype1).component(col).build();
    self.scene.add_entity(&triangle_prototype2).component(col).build();
  }
}

struct Orbit {
  dragging:bool,
  draggingStart:Option<mint::Vector2<f32>>

}

impl Default for Orbit {
  fn default() -> Self {
    Orbit{dragging: false, draggingStart: None}
  }
}

impl Orbit {
  fn event(&mut self, event:&Event, scene:&mut baryon::Scene, camera:&baryon::Camera) {
    match event {
      Event::Pointer { position } => {
        if !self.dragging { return; }
        if self.draggingStart == None {
          self.draggingStart = Some(*position);
        }
        println!("pointer: {:?}", position);
        let v = mint::Vector3{x:1.0, y:-1.0, z:0.0};
        scene[camera.node].post_rotate(v, 1.0);
      }
      Event::Click { button:Button::Left, pressed:true } => {
        self.dragging = true;
      }
      Event::Click { button:Button::Left, pressed:false } => {
        self.dragging = false;
        self.draggingStart = None;
      }
      Event::Scroll { delta } => {
        println!("scroll: {:?}", delta);
        let v = mint::Vector3{x:0.0, y:0.0, z:delta.y};
        scene[camera.node].post_move(v);
      },
      _ => {}
    }
  }
}
