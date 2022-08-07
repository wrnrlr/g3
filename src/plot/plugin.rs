// use bevy::prelude::{shape,Msaa,DefaultPlugins,Plugin,App,Query,Without,Handle,Quat,Changed,Vec3,AlphaMode,Entity,Commands,PbrBundle,Assets,Mesh,StandardMaterial,PointLight,PointLightBundle,ResMut,Added,Transform};
// use crate::*;
// use crate::{DoubleSidedPlane, pan_orbit_camera, Color, spawn_camera};

// pub struct PlotPlugin;
//
// impl Plugin for PlotPlugin {
//   fn build(&self, app: &mut App) {
//     app
//       .insert_resource(Msaa { samples: 4 })
//       .add_plugins(DefaultPlugins)
//       .add_startup_system(setup_light)
//       .add_startup_system(spawn_camera)
//       .add_system(points_added)
//       .add_system(points_changed)
//       .add_system(lines_added)
//       .add_system(plane_added)
//       .add_system(pan_orbit_camera);
//   }
//
//   fn name(&self) -> &str {
//     "Plot3DPlugin"
//   }
// }
//
// const LINE_RADIUS:f32 = 0.05;
// const POINT_RADIUS:f32 = 0.1;
//
// fn setup_light(mut cmd: Commands) {
//   cmd.spawn_bundle(PointLightBundle {
//     point_light: PointLight {intensity: 1500.0, shadows_enabled: false, ..Default::default()},
//     transform: Transform::from_xyz(4.0, 8.0, 4.0), ..Default::default()
//   });
// }
//
// fn points_added(
//   mut cmd:Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
//   q:Query<(Entity, &Point, &Color, Added<Point>)>
// ) {
//   for (e,p,c,_) in q.iter() {
//     cmd.entity(e).insert_bundle(PbrBundle {
//       mesh: meshes.add(Mesh::from(shape::Icosphere { radius: POINT_RADIUS, subdivisions: 8 })),
//       material: materials.add(bevy::prelude::Color::rgb(c.red(), c.green(), c.blue()).into()),
//       transform: Transform::from_xyz(p.x(), p.y(), p.z()),
//       ..Default::default()
//     });
//   }
// }
//
// fn points_changed(mut q:Query<(&Point, &mut Transform, Changed<Point>)>) {
//   for (p,mut t,_) in q.iter_mut() {
//     *t.translation = *Vec3::from([p.x(), p.y(), p.z()]);
//
//   }
// }
//
// fn plane_changed(mut q:Query<(&Plane, &mut Transform, Changed<Point>)>) {
//   for (p,mut t,_) in q.iter_mut() {
//     *t.translation = *Vec3::from([p.x(), p.y(), p.z()]);
//   }
// }
//
// fn lines_added(
//   mut cmd:Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
//   q:Query<(Entity, &Line, &Color, Added<Line>), (Without<Handle<Mesh>>)>
// ) {
//   for (e,l,c,_) in q.iter() {
//     let b:Branch = l.into();
//     // let r:Rotor = Rotor{ p1: b.p1 };
//     // let ea:EulerAngles = r.into();
//
//     cmd.entity(e).insert_bundle(PbrBundle {
//       mesh: meshes.add(Mesh::from(shape::Capsule { radius: LINE_RADIUS, depth: 2.0, rings: 1, ..Default::default()})),
//       material: materials.add(bevy::prelude::Color::rgb(c.red(), c.green(), c.blue()).into()),
//       transform: Transform::from_rotation(Quat::from_array([b.x(), b.y(), b.z(), 0.0])),
//       // transform: Transform::from_rotation(Quat::from_euler(glam::EulerRot::XYZ, ea.roll, ea.pitch, ea.yaw)),
//       ..Default::default()
//     });
//   }
// }
//
// fn plane_added(
//   mut cmd:Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
//   q:Query<(Entity, &Plane, &Color, Added<Plane>), (Without<Handle<Mesh>>)>
// ) {
//   for (e,p,c,_) in q.iter() {
//     print!("add plane ");
//     let mut material:StandardMaterial = bevy::prelude::Color::rgba(c.red(), c.green(), c.blue(), 0.5).into();
//     material.alpha_mode = AlphaMode::Blend;
//     material.double_sided = true;
//     let p0 = plane(0.0,1.0,0.0,0.0);
//     let _:Motor = (*p / p0).sqrt();
//     cmd.entity(e).insert_bundle(PbrBundle {
//       mesh: meshes.add(Mesh::from(DoubleSidedPlane{ size: 1.0 })),
//       material: materials.add(material),
//       // transform: Transform::from_rotation(Quat::from_array([p.x(), p.y(), b.z(), 0.0])),
//       // transform: Transform::from_rotation(Quat::from_euler(glam::EulerRot::XYZ, ea.roll, ea.pitch, ea.yaw)),
//       ..Default::default()
//     });
//   }
// }

// fn line_changed(
//   mut q:Query<(&Line, &mut Transform, Changed<Point>)>
// ) {
//   for (p,mut t,_) in q.iter_mut() {
//     *t.translation = *Vec3::from([p.x(), p.y(), p.z()]);
//   }
// }

// fn edge_added(
//   mut cmd:Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
//   q:Query<(Entity, &(Point,Point), &Rgba, Added<Point>)>
// ) {
//   for (e,p,c,_) in q.iter() {
//     cmd.entity(e).insert_bundle(PbrBundle {
//       mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 8 })),
//       material: materials.add(Color::rgb(c.red(), c.green(), c.blue()).into()),
//       transform: Transform::from_xyz(p.x(), p.y(), p.z()),
//       ..Default::default()
//     });
//   }
// }
//
// fn edge_changed(
//   mut q:Query<(&(Point,Point), &mut Transform, Changed<Point>)>
// ) {
//   for (p,mut t,_) in q.iter_mut() {
//     *t.translation = *Vec3::from([p.x(), p.y(), p.z()]);
//   }
// }
