

// Example based on https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga3d_objects

use crate::{point,line,plane,Point,Line,Plane};
use baryon::{Position,
  geometry::{Geometry, Streams},
  window::{Event, Window}};

use std::time;

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
const pink:u32 = 0x33C1B6FF;

fn vertex(scene:&mut baryon::Scene, context: &mut baryon::Context, p:Point, rgba:u32) {
  let sphere_prototype = Geometry::sphere(Streams::NORMAL, 0.1, 4).bake(context);
  scene
    .add_entity(&sphere_prototype)
    .position([p.x(), p.y(), p.z()].into())
    .component(baryon::Color(rgba))
    // .component(baryon::pass::Shader::Phong { glossiness: 10 })
    .build();
}

fn face(scene:&mut baryon::Scene, context: &mut baryon::Context, f:[Point;3], rgba:u32) {
  let triangle_prototype = triangle(f).bake(context);
  scene
    .add_entity(&triangle_prototype)
    // .position([p.x(), p.y(), p.z()].into())
    .component(baryon::Color(rgba))
    // .component(baryon::pass::Shader::Phong { glossiness: 10 })
    .build();
}

// fn cross(a:[f32;3],b:[f32;3])->[f32;3] {
//
// }

fn triangle(f:[Point;3])->Geometry {
  // https://gamedev.stackexchange.com/questions/60630/how-do-i-find-the-circumcenter-of-a-triangle-in-3d
  // let ac = c-a;
  // let ab = b-a;
  // let ab_ac = cross(ac,bc);
  let a = [f[0].x(),f[0].y(),f[0].z()];
  let b = [f[1].x(),f[1].y(),f[1].z()];
  let c = [f[2].x(),f[2].y(),f[2].z()];

  let radius:f32 = 5.0;
  Geometry{
    positions: vec!(Position(a),Position(b),Position(c)),
    normals: None, indices: None, radius
  }
}

pub fn mirror() {
    let window = Window::new().title("Elements").build();
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

    vertex(&mut scene, &mut context, point(0.0, 0.0, 0.0), black);
    vertex(&mut scene, &mut context, point(1.0, 1.0, 1.0), red);
    vertex(&mut scene, &mut context, point(1.0, -1.0, 2.0), green);
    vertex(&mut scene, &mut context, point(0.0, -1.0, 3.0), blue);

    face(&mut scene, &mut context, [point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0), point(1.0, -1.0, 2.0)], pink);


    // let mut pass = baryon::pass::Phong::new(
    //   &baryon::pass::PhongConfig {
    //       cull_back_faces: true,
    //       max_lights: 10,
    //       ambient: baryon::pass::Ambient {
    //           color: baryon::Color(0xFFFFFFFF),
    //           intensity: 0.2,
    //       },
    //   },
    //   &context,
    // );

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
          // let delta = moment.elapsed().as_secs_f32();
          // moment = time::Instant::now();

          // scene[node].pre_rotate(
          //   mint::Vector3{x: 0.0, y: 0.0, z: 1.0 },
          //   delta * 20.0,
          // );

          context.present(&mut pass, &scene, &camera);
        }
        _ => {}
    })
}
