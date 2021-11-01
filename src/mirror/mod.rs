use baryon::{Color,geometry::{Geometry, Streams},window::{Event, Key, Window}};
use crate::{Point};

mod cylinder;
mod triangle;

use triangle::triangle;
use cylinder::cylinder;

// struct Vertex(Point);
// struct Edge([Point;2]);
// struct Face([Point;3]);

// enum Element {
//   Vertex(Point),
//   Edge([Point;2]),
//   Face([Point;3]),
//   Line(Line)
// }

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
        .position([1.0f32, 0.0, 4.0].into())
        .look_at([0f32; 3].into(), [0f32, 0.0, 1.0].into())
        .build(),
      background: Color(0xFFFFFFFF),
    };

    Self{window, scene, context, camera}
  }

  pub fn run(self) {
    let Self { window, scene, mut context, camera } = self;

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
      Event::Keyboard {key:Key::Escape, pressed:true} => {
        std::process::exit(0x0100);
      }
      _ => {}
    })
  }

  pub fn vertex(&mut self, p:Point, col:Color) {
    let sphere_prototype = Geometry::sphere(Streams::NORMAL, 0.1, 4).bake(&mut self.context);
    self.scene
      .add_entity(&sphere_prototype)
      .position([p.x(), p.y(), p.z()].into())
      .component(col)
      // .component(baryon::pass::Shader::Phong { glossiness: 10 })
      .build();
  }

  pub fn edge(&mut self, e:[Point;2], col:Color) {
    let cylinder_prototype = cylinder(Streams::NORMAL, 0.5, 1.0).bake(&mut self.context);
    self.scene
      .add_entity(&cylinder_prototype)
      // .position([p.x(), p.y(), p.z()].into())
      .component(col)
      // .component(baryon::pass::Shader::Phong { glossiness: 10 })
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
    //     .color(baryon::Color(WHITE))
    //     .build();
}
