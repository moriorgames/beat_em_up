mod player;
mod window;

use event::EventHandler;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use player::player::Player;
use window::window::Window;

const GAME_ID: &str = "Beat 'em up";
const AUTHOR: &str = "MoriorGames";

struct MainState {
    player: Player,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player: Player = Player::new();
        Ok(MainState { player })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let clear: Color = Color::from([0.1, 0.2, 0.3, 1.0]);
        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

        self.player.draw(ctx, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let setup: WindowSetup = Window::create_window_setup();
    let mode: WindowMode = Window::create_window_mode();
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(setup)
        .window_mode(mode);
    let (mut ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
