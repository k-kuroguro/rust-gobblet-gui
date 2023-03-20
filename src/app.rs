use eframe::{CreationContext, Frame};
use egui::{CentralPanel, Context};

pub struct App {}

impl Default for App {
   fn default() -> Self {
      Self {}
   }
}

impl App {
   pub fn new(_cc: &CreationContext<'_>) -> Self {
      Default::default()
   }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
      CentralPanel::default().show(ctx, |ui| {
         ui.heading("eframe template");
         ui.hyperlink("https://github.com/emilk/eframe_template");
         ui.add(egui::github_link_file!(
            "https://github.com/emilk/eframe_template/blob/master/",
            "Source code."
         ));
      });
   }
}
