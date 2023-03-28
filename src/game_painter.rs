use eframe::{
   emath::RectTransform,
   epaint::{CircleShape, RectShape},
};
use egui::{pos2, vec2, Id, Rect, Response, Rounding, Sense, Shape, Ui};
use gobblet::{board::Board, color::Color, game::Game, piece::Piece, square::Square};

use crate::style::Style;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Location {
   Board(Square),
   Hand(usize),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct LocatedPiece {
   pub piece: Piece,
   pub location: Location,
}

pub struct GamePainter<'a> {
   style: Style,
   square_size: f32,
   to_screen: RectTransform,
   ui: &'a Ui,
   response: &'a Response,
   painter: &'a egui::Painter,
}

impl<'a> GamePainter<'a> {
   pub fn new(
      ui: &'a Ui,
      response: &'a Response,
      painter: &'a egui::Painter,
      square_size: f32,
      to_screen: RectTransform,
   ) -> Self {
      Self {
         style: Style::default(),
         square_size,
         to_screen,
         ui,
         response,
         painter,
      }
   }

   pub fn paint_board(&self) -> Option<Square> {
      let mut clicked_square = None;
      let mut shapes = Vec::with_capacity(Board::SIZE.pow(2));

      for x in 0..Board::SIZE {
         for y in 0..Board::SIZE {
            let rect = Rect::from_min_max(
               self.to_screen * pos2(x as f32, y as f32 + 1.),
               self.to_screen * pos2(x as f32 + 1., y as f32 + 2.),
            );

            if self
               .ui
               .interact(rect, self.generate_id(x, y, 10), Sense::click())
               .clicked()
            {
               clicked_square = Some(Square::from_pos(x, y));
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

      self.painter.extend(shapes);

      clicked_square
   }

   pub fn paint_pieces(
      &self,
      game: &Game,
      selected_piece: Option<LocatedPiece>,
   ) -> Option<LocatedPiece> {
      let mut clicked_piece: Option<LocatedPiece> = None;
      let mut shapes = Vec::new();

      for (i, set) in game.board().into_iter().enumerate() {
         if let Some(&piece) = set.peek() {
            let Piece { color, kind } = piece;
            let center = self.to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
            let radius = 0.5 * self.style.piece_radius_ratio[kind as usize] * self.square_size;

            if self
               .ui
               .interact(
                  Rect::from_center_size(center, vec2(2. * radius, 2. * radius)),
                  self.generate_id(i, 1, 100),
                  Sense::click(),
               )
               .clicked()
            {
               clicked_piece = Some(LocatedPiece {
                  piece,
                  location: Location::Board(Square::ALL[i]),
               });
            }

            shapes.push(Shape::Circle(CircleShape {
               center,
               radius,
               fill: self.style.piece_fill_color[color as usize],
               stroke: if self.is_selected(
                  LocatedPiece {
                     piece,
                     location: Location::Board(Square::ALL[i]),
                  },
                  selected_piece,
               ) {
                  self.style.selected_piece_stroke
               } else {
                  self.style.piece_stroke
               },
            }));
         }
      }

      self.painter.extend(shapes);

      clicked_piece
   }

   pub fn paint_hands(
      &self,
      game: &Game,
      selected_piece: Option<LocatedPiece>,
   ) -> Option<LocatedPiece> {
      let mut clicked_piece: Option<LocatedPiece> = None;
      let mut shapes = Vec::new();

      for (i, hand) in Color::ALL.map(|color| game.hand(color)).iter().enumerate() {
         for (j, set) in hand.into_iter().enumerate() {
            if let Some(&piece) = set.peek() {
               let Piece { color, kind } = piece;
               let center = self.to_screen
                  * pos2(
                     j as f32 + 1.,
                     if i == 0 {
                        0.5
                     } else {
                        Board::SIZE as f32 + 1.5
                     },
                  );
               let radius = 0.5 * self.style.piece_radius_ratio[kind as usize] * self.square_size;

               if self
                  .ui
                  .interact(
                     Rect::from_center_size(center, vec2(2. * radius, 2. * radius)),
                     self.generate_id(i, j, 1000),
                     Sense::click(),
                  )
                  .clicked()
               {
                  clicked_piece = Some(LocatedPiece {
                     piece,
                     location: Location::Hand(j),
                  });
               }

               shapes.push(Shape::Circle(CircleShape {
                  center,
                  radius,
                  fill: self.style.piece_fill_color[color as usize],
                  stroke: if self.is_selected(
                     LocatedPiece {
                        piece,
                        location: Location::Hand(j),
                     },
                     selected_piece,
                  ) {
                     self.style.selected_piece_stroke
                  } else {
                     self.style.piece_stroke
                  },
               }));
            }
         }
      }

      self.painter.extend(shapes);

      clicked_piece
   }

   pub fn paint_available_moves(&self, game: &Game, selected_piece: Option<LocatedPiece>) {
      let mut shapes = Vec::new();

      if let Some(LocatedPiece { piece, location }) = selected_piece {
         match location {
            Location::Board(_) => {
               for (i, &square) in Square::ALL.iter().enumerate() {
                  if game.board().can_move(piece, square) {
                     let center = self.to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
                     let radius = 0.5 * self.style.available_move_radius_ratio * self.square_size;

                     shapes.push(Shape::Circle(CircleShape::filled(
                        center,
                        radius,
                        self.style.available_move_color,
                     )));
                  }
               }
            }
            Location::Hand(_) => {
               for (i, &square) in Square::ALL.iter().enumerate() {
                  if game.board().can_place(piece, square) {
                     let center = self.to_screen * pos2((i % 4) as f32 + 0.5, (i / 4) as f32 + 1.5);
                     let radius = 0.5 * self.style.available_move_radius_ratio * self.square_size;

                     shapes.push(Shape::Circle(CircleShape::filled(
                        center,
                        radius,
                        self.style.available_move_color,
                     )));
                  }
               }
            }
         }
      }

      self.painter.extend(shapes);
   }

   fn generate_id(&self, i: usize, j: usize, k: usize) -> Id {
      self.response.id.with((i, j, k))
   }

   fn is_selected(&self, piece: LocatedPiece, selected_piece: Option<LocatedPiece>) -> bool {
      match selected_piece {
         Some(LocatedPiece {
            piece: selected_piece,
            location: selected_location,
         }) => selected_location == piece.location && selected_piece == piece.piece,
         None => false,
      }
   }
}
