#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
   use eframe::{NativeOptions, Theme};

   let native_options = NativeOptions {
      default_theme: Theme::Light,
      ..NativeOptions::default()
   };
   eframe::run_native(
      "Gobblet",
      native_options,
      Box::new(|cc| Box::new(gobblet_gui::App::new(cc))),
   )
}

#[cfg(target_arch = "wasm32")]
fn main() {
   console_error_panic_hook::set_once();
   tracing_wasm::set_as_global_default();

   let web_options = eframe::WebOptions::default();
   wasm_bindgen_futures::spawn_local(async {
      eframe::start_web(
         "the_canvas_id", // hardcode it
         web_options,
         Box::new(|cc| Box::new(gobblet_gui::App::new(cc))),
      )
      .await
      .expect("failed to start eframe");
   });
}
