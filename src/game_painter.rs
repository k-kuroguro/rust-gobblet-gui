use eframe::{
   emath::RectTransform,
   epaint::{CircleShape, RectShape},
};
use egui::{
   pos2, vec2, Color32, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget,
};
use gobblet::{
   board::BOARD_SIZE,
   color::{ALL_COLORS, COLOR_NUM},
   game::Game,
   hand::PIECE_SET_NUM,
   piece::{Piece, PIECE_KIND_NUM},
};

struct Style {
   light_square_color: Color32,
   dark_square_color: Color32,
   piece_radius_ratio: [f32; PIECE_KIND_NUM],
   piece_fill_color: [Color32; COLOR_NUM],
   piece_stroke: Stroke,
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
      }
   }
}

pub struct GamePainter<'a> {
   game: &'a mut Game,
   size: Vec2,
   style: Style,
}

impl<'a> GamePainter<'a> {
   pub fn new(game: &'a mut Game, size: Vec2) -> Self {
      Self {
         game,
         size,
         style: Style::default(),
      }
   }
}

impl Widget for GamePainter<'_> {
   fn ui(self, ui: &mut Ui) -> Response {
      let (response, painter) = ui.allocate_painter(self.size, Sense::hover());
      let square_size = (response.rect.height() / (BOARD_SIZE) as f32)
         .min(response.rect.width() / (BOARD_SIZE + 2) as f32);
      let board_size = square_size * BOARD_SIZE as f32;
      let to_screen = RectTransform::from_to(
         Rect::from_min_max(Pos2::ZERO, pos2(BOARD_SIZE as f32, BOARD_SIZE as f32)),
         Rect::from_min_size(
            pos2(square_size, response.rect.min.y),
            vec2(board_size, board_size),
         ),
      );

      let mut shapes = Vec::with_capacity(2 * BOARD_SIZE.pow(2) + 2 * PIECE_SET_NUM);

      // Paint board.
      for x in 0..BOARD_SIZE {
         for y in 0..BOARD_SIZE {
            shapes.push(Shape::Rect(RectShape::filled(
               Rect::from_min_max(
                  to_screen * pos2(x as f32, y as f32),
                  to_screen * pos2(x as f32 + 1., y as f32 + 1.),
               ),
               Rounding::none(),
               if x % 2 == y % 2 {
                  self.style.light_square_color
               } else {
                  self.style.dark_square_color
               },
            )));
         }
      }

      // Paint pieces.
      for (i, set) in self.game.board().into_iter().enumerate() {
         if let Some(&Piece { color, kind }) = set.peek() {
            shapes.push(Shape::Circle(CircleShape {
               center: to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 0.5),
               radius: 0.5 * self.style.piece_radius_ratio[kind as usize] * square_size,
               fill: self.style.piece_fill_color[color as usize],
               stroke: self.style.piece_stroke,
            }));
         }
      }

      // Paint hands.
      for (i, hand) in ALL_COLORS
         .map(|color| self.game.hand(color))
         .iter()
         .enumerate()
      {
         for (j, set) in hand.into_iter().enumerate() {
            if let Some(&Piece { color, kind }) = set.peek() {
               shapes.push(Shape::Circle(CircleShape {
                  center: to_screen
                     * pos2(
                        if i == 0 {
                           -0.5
                        } else {
                           BOARD_SIZE as f32 + 0.5
                        },
                        j as f32 + 0.5,
                     ),
                  radius: 0.5 * self.style.piece_radius_ratio[kind as usize] * square_size,
                  fill: self.style.piece_fill_color[color as usize],
                  stroke: self.style.piece_stroke,
               }));
            }
         }
      }

      painter.extend(shapes);

      response
   }
}