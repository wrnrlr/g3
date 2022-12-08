use eframe::egui_glow;
use egui::CentralPanel;
use egui::mutex::Mutex;
use g3::prelude::*;

struct Demo {
  renderer: Arc<Mutex<Renderer>>
}

impl Demo {
  pub fn new(cc: &eframe::CreationContext<'_>, world:World)->Self {
    let gl = cc.gl.as_ref().unwrap();
    Self{renderer: Arc::new(Mutex::new(Renderer::new(gl, world)))}
  }
}

impl eframe::App for Demo {
  fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    CentralPanel::default().show(ctx, |ui| {
      let size = ui.available_size();
      let (rect, response) = ui.allocate_exact_size(size, egui::Sense::drag());

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
  // Log to stdout (if you run with `RUST_LOG=debug`).
  // tracing_subscriber::fmt::init();

  let mut world = World::new();

  let origin = world.spawn((point(0.0,0.0,0.0), 0xff0000));
  let a = world.spawn((point(0.0,1.0,0.0), 0xff0000));
  let b = world.spawn((point(-1.0,-1.0,0.0), 0xff0000));
  let c = world.spawn((point(1.0,-1.0,0.0), 0xff0000));
  // let d = world.spawn((point(0.0,2.0,0.0), 0x00ff00));
  let e = world.spawn((plane(1.0,0.0,0.0,1.0), 0x00ff00));

  eframe::run_native("Renderer", eframe::NativeOptions::default(),
    Box::new(|cc| Box::new(Demo::new(cc, world)))
  );
}
