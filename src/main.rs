use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameResult,
};
use graphics::{Canvas, Color};
use ggez::GameError;
use event::EventHandler;

struct MainState {
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { })
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
    let cb = ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
