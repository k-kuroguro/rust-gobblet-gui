use egui::{Color32, Stroke};
use gobblet::{color::Color, piece::PieceKind};

pub struct Style {
   pub light_square_color: Color32,
   pub dark_square_color: Color32,
   pub piece_radius_ratio: [f32; PieceKind::NUM],
   pub piece_fill_color: [Color32; Color::NUM],
   pub piece_stroke: Stroke,
   pub selected_piece_stroke: Stroke,
   pub available_move_radius_ratio: f32,
   pub available_move_color: Color32,
}

impl Default for Style {
   fn default() -> Self {
      Self {
         light_square_color: Color32::from_rgb(235, 235, 200),
         dark_square_color: Color32::from_rgb(120, 150, 80),
         piece_radius_ratio: [0.2, 0.4, 0.6, 0.8],
         piece_fill_color: [
            Color32::from_rgb(85, 85, 85),
            Color32::from_rgb(250, 250, 250),
         ],
         piece_stroke: Stroke::new(2., Color32::from_rgb(30, 30, 30)),
         selected_piece_stroke: Stroke::new(2., Color32::RED),
         available_move_radius_ratio: 0.1,
         available_move_color: Color32::GRAY,
      }
   }
}
