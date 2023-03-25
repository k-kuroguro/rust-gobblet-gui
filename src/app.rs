use eframe::CreationContext;
use eframe::{
   emath::RectTransform,
   epaint::{CircleShape, RectShape},
};
use egui::{pos2, vec2, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke};
use egui::{CentralPanel, Context};
use gobblet::{
   board::BOARD_SIZE,
   color::{ALL_COLORS, COLOR_NUM},
   game::{Action, Game},
   hand::PIECE_SET_NUM,
   piece::{Piece, PIECE_KIND_NUM},
   square::{Square, ALL_SQUARES},
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

pub struct App {
   game: Game,
   selected_piece_index: Option<(usize, Option<usize>)>,
   selected_piece: Option<Piece>,
   style: Style,
}

impl Default for App {
   fn default() -> Self {
      Self {
         game: Game::new(),
         selected_piece_index: None,
         selected_piece: None,
         style: Style::default(),
      }
   }
}

impl App {
   pub fn new(_cc: &CreationContext<'_>) -> Self {
      Default::default()
   }

   fn move_or_place(&mut self, x: usize, y: usize) {
      if let Some((i, j)) = self.selected_piece_index {
         if let Some(j) = j {
            let square = ALL_SQUARES[x + 4 * y];
            let action = Action::PlaceFromHand {
               index: j,
               to: square,
            };
            match self.game.execute(action) {
               Ok(std) => {
                  println!("{:?}", std)
               }
               Err(e) => {
                  println!("{:?}", e)
               }
            }
         } else {
            let from = ALL_SQUARES[i];
            let to = ALL_SQUARES[x + 4 * y];
            let action = Action::Move { from, to };
            match self.game.execute(action) {
               Ok(std) => {
                  println!("{:?}", std)
               }
               Err(e) => {
                  println!("{:?}", e)
               }
            }
         }
      }
   }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
      CentralPanel::default().show(ctx, |ui| {
         let square_num = vec2(BOARD_SIZE as f32, BOARD_SIZE as f32 + 2.);
         let square_size =
            (ui.available_size().y / square_num.y).min(ui.available_size().x / square_num.x);
         let (response, painter) = ui.allocate_painter(square_size * square_num, Sense::hover());
         let to_screen = RectTransform::from_to(
            Rect::from_min_max(Pos2::ZERO, square_num.to_pos2()),
            response.rect,
         );

         let mut shapes = Vec::with_capacity(2 * (BOARD_SIZE.pow(2) + PIECE_SET_NUM));

         // Paint board.
         for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
               let rect = Rect::from_min_max(
                  to_screen * pos2(x as f32, y as f32 + 1.),
                  to_screen * pos2(x as f32 + 1., y as f32 + 2.),
               );
               let piece_response =
                  ui.interact(rect, response.id.with((x, y, BOARD_SIZE)), Sense::click());
               if piece_response.clicked() {
                  self.move_or_place(x, y);
                  self.selected_piece = None;
                  self.selected_piece_index = None;
               }
               shapes.push(Shape::Rect(RectShape::filled(
                  rect,
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
               let center = to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
               let radius = 0.5 * self.style.piece_radius_ratio[kind as usize] * square_size;
               let piece_response = ui.interact(
                  Rect::from_center_size(center, vec2(2. * radius, 2. * radius)),
                  response.id.with((i, PIECE_SET_NUM)),
                  Sense::click(),
               );
               if piece_response.clicked() {
                  if color == *self.game.turn() {
                     self.selected_piece = Some(Piece { color, kind });
                     self.selected_piece_index = Some((i, None));
                  } else {
                     self.move_or_place(i % 4, i / 4);
                     self.selected_piece = None;
                     self.selected_piece_index = None;
                  }
               }
               shapes.push(Shape::Circle(CircleShape {
                  center,
                  radius,
                  fill: self.style.piece_fill_color[color as usize],
                  stroke: if (piece_response.clicked() && color == *self.game.turn())
                     || self.selected_piece_index == Some((i, None))
                  {
                     Stroke::new(2., Color32::RED)
                  } else {
                     self.style.piece_stroke
                  },
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
                  let center = to_screen
                     * pos2(
                        j as f32 + 1.,
                        if i == 0 { 0.5 } else { BOARD_SIZE as f32 + 1.5 },
                     );
                  let radius = 0.5 * self.style.piece_radius_ratio[kind as usize] * square_size;
                  let piece_response = ui.interact(
                     Rect::from_center_size(center, vec2(2. * radius, 2. * radius)),
                     response.id.with((i, j)),
                     Sense::click(),
                  );
                  if piece_response.clicked() && color == *self.game.turn() {
                     self.selected_piece = Some(Piece { color, kind });
                     self.selected_piece_index = Some((i, Some(j)));
                  }
                  shapes.push(Shape::Circle(CircleShape {
                     center,
                     radius,
                     fill: self.style.piece_fill_color[color as usize],
                     stroke: if (piece_response.clicked() && color == *self.game.turn())
                        || self.selected_piece_index == Some((i, Some(j)))
                     {
                        Stroke::new(2., Color32::RED)
                     } else {
                        self.style.piece_stroke
                     },
                  }));
               }
            }
         }

         if let Some((_, j)) = self.selected_piece_index {
            if let Some(_) = j {
               for (i, &square) in ALL_SQUARES.iter().enumerate() {
                  if self
                     .game
                     .board()
                     .can_place(self.selected_piece.unwrap(), square)
                  {
                     let center = to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
                     let radius = 0.5 * 0.1 * square_size;
                     shapes.push(Shape::Circle(CircleShape::filled(
                        center,
                        radius,
                        Color32::GRAY,
                     )));
                  }
               }
            } else {
               for (i, &square) in ALL_SQUARES.iter().enumerate() {
                  if self
                     .game
                     .board()
                     .can_move(self.selected_piece.unwrap(), square)
                  {
                     let center = to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
                     let radius = 0.5 * 0.1 * square_size;
                     shapes.push(Shape::Circle(CircleShape::filled(
                        center,
                        radius,
                        Color32::GRAY,
                     )));
                  }
               }
            }
         }

         painter.extend(shapes);
      });
   }
}
