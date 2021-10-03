/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2021-10-03
 */

use ggez::{conf, event, graphics, ContextBuilder, Context, GameError, GameResult};
use std::{path, env, collections::HashMap};
use chess_template::{Game, Colour, PieceType};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color = graphics::Color::new(228.0/255.0, 196.0/255.0, 108.0/255.0, 1.0);
const WHITE: graphics::Color = graphics::Color::new(188.0/255.0, 140.0/255.0, 76.0/255.0, 1.0);

/// GUI logic and event implementation structure. 
struct AppState {
    sprites: HashMap<(Colour, PieceType), graphics::Image>,
    board: [[Option<(Colour, PieceType)>; 8]; 8],
    game: Game
    // Save piece positions, which tiles has been clicked, current colour, etc...
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {

        let royal_rank = |_colour| [
            Some((_colour, PieceType::Rook)),
            Some((_colour, PieceType::Knight)),
            Some((_colour, PieceType::Bishop)),
            Some((_colour, PieceType::Queen)),
            Some((_colour, PieceType::King)),
            Some((_colour, PieceType::Rook)),
            Some((_colour, PieceType::Knight)),
            Some((_colour, PieceType::Bishop))
        ];
        let pawn_rank = |_colour| [Some((_colour, PieceType::Pawn)); 8];
        let empty_rank = || [None; 8];
        
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            board: [
                royal_rank(Colour::Black), pawn_rank(Colour::Black),
                empty_rank(), empty_rank(), empty_rank(), empty_rank(),
                pawn_rank(Colour::White), royal_rank(Colour::White)
            ],
            game: Game::new()
        };

        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites(ctx: &mut Context) -> HashMap<(Colour, PieceType), graphics::Image> {

        [
            ((Colour::Black, PieceType::King), "/black_king.png".to_string()),
            ((Colour::Black, PieceType::Queen), "/black_queen.png".to_string()),
            ((Colour::Black, PieceType::Rook), "/black_rook.png".to_string()),
            ((Colour::Black, PieceType::Pawn), "/black_pawn.png".to_string()),
            ((Colour::Black, PieceType::Bishop), "/black_bishop.png".to_string()),
            ((Colour::Black, PieceType::Knight), "/black_knight.png".to_string()),
            ((Colour::White, PieceType::King), "/white_king.png".to_string()),
            ((Colour::White, PieceType::Queen), "/white_queen.png".to_string()),
            ((Colour::White, PieceType::Rook), "/white_rook.png".to_string()),
            ((Colour::White, PieceType::Pawn), "/white_pawn.png".to_string()),
            ((Colour::White, PieceType::Bishop), "/white_bishop.png".to_string()),
            ((Colour::White, PieceType::Knight), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(_piece, _path)| {
                (*_piece, graphics::Image::new(ctx, _path).unwrap())
            })
            .collect::<HashMap<(Colour, PieceType), graphics::Image>>()
    }
}

impl event::EventHandler<GameError> for AppState {

    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
                graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state())
            )
            .scale(graphics::PxScale { x: 30.0, y: 30.0 }));

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            graphics::Rect::new((SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                                text_dimensions.w as f32 + 16.0, text_dimensions.h as f32),
                                [1.0, 1.0, 1.0, 1.0].into()
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default()).expect("Failed to draw background.");

        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {

                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), match _col % 2 {
                        0 => 
                            if _row % 2 == 0 { WHITE } 
                            else { BLACK },
                        _ => 
                            if _row % 2 == 0 { BLACK } 
                            else { WHITE },
                    }).expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");

                // draw piece
                if let Some(_piece) = self.board[_row as usize][_col as usize] {
                    graphics::draw(ctx, self.sprites.get(&_piece).unwrap(), graphics::DrawParam::default()
                        .scale([2.0, 2.0])  // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest(
                            [_col as f32 * GRID_CELL_SIZE.0 as f32, _row as f32 * GRID_CELL_SIZE.1 as f32],
                        )
                    ).expect("Failed to draw piece.");
                }
            }
        }

        // draw text with dark gray colouring and center position
        graphics::draw(ctx, &state_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
            .dest(ggez::mint::Point2 {
                x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
            })).expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
        }
    }
}

pub fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "viola")
        .add_resource_path(resource_dir)        // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()  
                .title("Schack")                // Set window title "Schack"
                .icon("/icon.png")              // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false)               // Fixate window size
        );
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state)       // Run window event loop
}