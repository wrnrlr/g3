// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use std::cell::RefCell;
use std::rc::Rc;
use crate::{line, Line, plane, Plane, point, Point};
use baryon::{Position,geometry::{Geometry, Streams},window::{Event, Window}};

use std::time;

mod cylinder;
mod triangle;

use triangle::triangle;

// struct Vertex(Point);
// struct Edge([Point;2]);
// struct Face([Point;3]);

// enum Element {
//   Vertex(Point),
//   Edge([Point;2]),
//   Face([Point;3]),
//   Line(Line)
// }

const black:u32 = 0xFF000000;
const white:u32 = 0xFFFFFFFF;
const red:u32 = 0xFF0000FF;
const green:u32 = 0xFF00FF00;
const blue:u32 = 0xFFFF0000;
const pink:u32 = 0xFFCCBBFF;

pub struct Mirror {
  window:Window,
  scene:baryon::Scene,
  context:baryon::Context,
  camera:baryon::Camera,
}

impl Mirror {

  pub fn new()->Self {
    let window = Window::new().title("Mirror").build();
    let mut context = pollster::block_on(baryon::Context::init().build(&window));
    let mut scene = baryon::Scene::new();

    let camera = baryon::Camera {
      projection: baryon::Projection::Perspective { fov_y: 45.0 },
      depth: 1.0..10.0,
      node: scene
        .add_node()
        .position([1.8f32, -8.0, 3.0].into())
        .look_at([0f32; 3].into(), [0f32, 0.0, 1.0].into())
        .build(),
      background: baryon::Color(0xFFFFFFFF),
    };

    Self{window, scene, context, camera}
  }

  pub fn run(self) {

    let Self {
      window, scene, mut context, camera
    } = self;

    let mut pass = baryon::pass::Solid::new(
      &baryon::pass::SolidConfig {
        cull_back_faces: true,
      },
      &context,
    );

    // let elements = [point(1.0, 1.0, 1.0), line(1.0,1.0,1.0,2.0,2.0,2.0)]
    // let mut moment = time::Instant::now();

    window.run(move |event| match event {
      Event::Resize { width, height } => {
        context.resize(width, height);
      }
      Event::Draw => {
        // scene[node].pre_rotate(mint::Vector3{x: 0.0, y: 0.0, z: 1.0 }, delta * 20.0);
        context.present(&mut pass, &scene, &camera);
      }
      _ => {}
    })
  }

  pub fn vertex(&mut self, p:Point, rgba:u32) {
    let sphere_prototype = Geometry::sphere(Streams::NORMAL, 0.1, 4).bake(&mut self.context);
    self.scene
      .add_entity(&sphere_prototype)
      .position([p.x(), p.y(), p.z()].into())
      .component(baryon::Color(rgba))
      // .component(baryon::pass::Shader::Phong { glossiness: 10 })
      .build();
  }

  pub fn edge(&mut self) {}

  pub fn face(&mut self, f:[Point;3], rgba:u32) {
    let triangle_prototype = triangle(f).bake(&mut self.context);
    self.scene
      .add_entity(&triangle_prototype)
      // .position([p.x(), p.y(), p.z()].into())
      .component(baryon::Color(rgba))
      // .component(baryon::pass::Shader::Phong { glossiness: 10 })
      .build();
  }
}

// https://users.rust-lang.org/t/self-has-an-anonymous-lifetime-but-it-needs-to-satisfy-a-static-lifetime-requirement/58641/3

pub fn mirror() {
    // let _point_light = scene
    //     .add_point_light()
    //     .position([3.0, 3.0, 3.0].into())
    //     .color(baryon::Color(0xFFFF8080))
    //     .build();
    // let _dir_light = scene
    //     .add_directional_light()
    //     .position([0.0, 0.0, 5.0].into())
    //     .intensity(4.0)
    //     .color(baryon::Color(white))
    //     .build();

    let mut mr = Mirror::new();

    let a = point(0.0, 0.0, 0.0);
    let b = point(1.0, 1.0, 1.0);
    let c = point(1.0, -1.0, 2.0);
    let d = point(0.0, -1.0, 3.0);
    // let e = point(0.0, -1.0, 3.0);

    mr.vertex(a, black);
    mr.vertex(b, red);
    mr.vertex(c, green);
    mr.vertex(d, blue);

    mr.face([a, b, c], pink);

    // mr.edge();

    mr.run();
}
