use eframe::CreationContext;
use egui::{CentralPanel, Context, TopBottomPanel};
use gobblet::{
   game::{Action, Game},
   square::Square,
};

use crate::game_painter::GamePainter;

pub struct App {
   game: Game,
}

impl Default for App {
   fn default() -> Self {
      Self { game: Game::new() }
   }
}

impl App {
   pub fn new(_cc: &CreationContext<'_>) -> Self {
      Default::default()
   }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
      CentralPanel::default().show(ctx, |ui| {
         let mut game = Game::new();
         _ = game.execute(Action::PlaceFromHand {
            index: 0,
            to: Square::A1,
         });
         ui.add(GamePainter::new(&mut game, ui.available_size()));
      });
   }
}
