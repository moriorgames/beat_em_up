mod window;

use event::EventHandler;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use window::window::Window;

const GAME_ID: &str = "";
const AUTHOR: &str = "";

struct MainState {
    _window: Window,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { _window: Window {} })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let clear: Color = Color::from([0.1, 0.2, 0.3, 1.0]);
        let canvas: Canvas = Canvas::from_frame(ctx, clear);

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let setup: WindowSetup = WindowSetup::default().title(Window::TITLE);
    let mode: WindowMode = WindowMode::default().dimensions(Window::WIDTH, Window::HEIGHT);
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(setup)
        .window_mode(mode);
    let (mut ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
