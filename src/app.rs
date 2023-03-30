use eframe::{emath::RectTransform, CreationContext};
use egui::{vec2, Align2, CentralPanel, Context, Pos2, Rect, Sense, Vec2};
use gobblet::{Action, Board, Game, Square, Status};

use crate::game_painter::{GamePainter, LocatedPiece, Location};

const SQUARE_NUM: Vec2 = vec2(Board::SIZE as f32, Board::SIZE as f32 + 2.);
pub struct App {
   game: Game,
   selected_piece: Option<LocatedPiece>,
   finished: bool,
}

impl Default for App {
   fn default() -> Self {
      Self {
         game: Game::new(),
         selected_piece: None,
         finished: false,
      }
   }
}

impl App {
   pub fn new(_cc: &CreationContext<'_>) -> Self {
      Default::default()
   }

   fn move_or_place(&mut self, to: Square) -> bool {
      if let Some(LocatedPiece { piece: _, location }) = self.selected_piece {
         let action = match location {
            Location::Board(from) => Action::Move { from, to },
            Location::Hand(index) => Action::PlaceFromHand { index, to },
         };
         match self.game.execute(action) {
            Ok(_) => true,
            Err(_) => false,
         }
      } else {
         false
      }
   }

   fn try_select(&mut self, clicked_piece: LocatedPiece) {
      if clicked_piece.piece.color == self.game.turn() {
         self.selected_piece = Some(clicked_piece);
      }
   }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
      CentralPanel::default().show(ctx, |ui| {
         let available_size = ui.available_size();
         let square_size = (available_size.y / SQUARE_NUM.y).min(available_size.x / SQUARE_NUM.x);
         let (response, painter) = ui.allocate_painter(square_size * SQUARE_NUM, Sense::hover());
         let to_screen = RectTransform::from_to(
            Rect::from_min_max(Pos2::ZERO, SQUARE_NUM.to_pos2()),
            response.rect,
         );
         let painter = GamePainter::new(ui, &response, &painter, square_size, to_screen);

         let is_background_clicked = ui
            .interact(
               Rect::from_min_size(Pos2::ZERO, available_size),
               ui.id().with("background"),
               Sense::click(),
            )
            .clicked();
         if is_background_clicked {
            self.selected_piece = None;
         }

         if let Some(clicked_square) = painter.paint_board() {
            if let Some(top) = self.game.board().get_top(clicked_square) {
               if self.selected_piece.is_none() {
                  self.try_select(LocatedPiece {
                     piece: top,
                     location: Location::Board(clicked_square),
                  });
               } else {
                  if self.move_or_place(clicked_square) {
                     self.selected_piece = None;
                  } else {
                     self.try_select(LocatedPiece {
                        piece: top,
                        location: Location::Board(clicked_square),
                     });
                  }
               }
            } else {
               if self.move_or_place(clicked_square) {
                  self.selected_piece = None;
               }
            }
         }

         if let Some(clicked_piece) = painter.paint_pieces(&self.game, self.selected_piece) {
            if let Location::Board(square) = clicked_piece.location {
               if self.selected_piece.is_none() {
                  self.try_select(clicked_piece);
               } else {
                  if self.move_or_place(square) {
                     self.selected_piece = None;
                  } else {
                     self.try_select(clicked_piece);
                  }
               }
            }
         }

         if let Some(clicked_piece) = painter.paint_hands(&self.game, self.selected_piece) {
            self.try_select(clicked_piece);
         }

         painter.paint_available_moves(&self.game, self.selected_piece);

         if self.game.status() != Status::OnGoing {
            let title = match self.game.status() {
               Status::OnGoing => unreachable!(),
               Status::BlackWins => "Black Wins!",
               Status::WhiteWins => "White Wins!",
            };
            egui::Window::new(title)
               .resizable(false)
               .collapsible(false)
               .anchor(Align2::CENTER_TOP, [0., 5.])
               .open(&mut self.finished)
               .show(ctx, |ui| {
                  if ui.button("Restart").clicked() {
                     self.selected_piece = None;
                     self.game = Game::new();
                  }
               });
            self.finished = true;
         }
      });
   }
}
