use std::sync::Arc;
use eframe::{egui_glow};
use egui::CentralPanel;
use egui::mutex::Mutex;
use g3::{*};
use hecs::World;


struct Demo {
  renderer: Arc<Mutex<Renderer>>
}

impl Demo {
  pub fn new(cc: &eframe::CreationContext<'_>, world:World, run: Option<fn(&mut World)>) ->Self {
    let gl = cc.gl.as_ref().unwrap();
    Self{renderer: Arc::new(Mutex::new(Renderer::new(gl, world, run)))}
  }
}

impl eframe::App for Demo {
  fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    CentralPanel::default().show(ctx, |ui| {
      let size = ui.available_size();
      let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::drag());

      let renderer = self.renderer.clone();

      let cb = egui_glow::CallbackFn::new(move |_info, painter| {
        renderer.lock().paint(painter.gl());
      });

      let callback = egui::PaintCallback { rect, callback: Arc::new(cb) };
      ui.painter().add(callback);
    });
  }
}

fn main() {
  let mut world = World::new();

  let m = (e1 * e2).sqrt();
  let p = e1;
  let a = m(point(-1.0,0.0,-1.0));
  let b = m(point(-1.0,0.0,1.0));
  let c = m(point(1.0,0.0,1.0));
  let d = m(point(1.0,0.0,-1.0));

  // world.spawn_batch([
    // (point(0.0,0.0,0.0), Color::MAGENTA),
    // (point(0.0,1.0,0.0), Color::RED),
    // (point(-1.0,-1.0,0.0), Color::GREEN),
    // (point(1.0,-1.0,0.0), Color::YELLOW),

    // (a, Color::CYAN),
    // (b, Color::CYAN),
    // (c, Color::CYAN),
    // (d, Color::CYAN),
  // ]);

  world.spawn((e1 ^ e2, Color::BLUE));
  world.spawn((e2 ^ e3, Color::GREEN));
  world.spawn((e3 ^ e1, Color::RED));

  world.spawn_batch([
    (e1+e2, Color(0xff000088)),
    // (e2, Color(0x00ff0088)),
    // (e3, Color(0x0000ff88)),
    // (plane(0.0,1.0,0.0,0.0), Color::GREEN),
    // (plane(0.0,0.0,1.0,0.0), Color::BLUE)
  ]);

  world.spawn((Box::new(||{ e1 }),));

  fn run(world:&mut World) {
    for (_id, (l,c)) in world.query_mut::<(&Line, &Color)>() {
      println!("{:?}", l);
    }
  }

  eframe::run_native("Renderer", eframe::NativeOptions::default(),
    Box::new(|cc| Box::new(Demo::new(cc, world, Some(run))))
  );
}
