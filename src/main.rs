mod combat;
mod enemy;
mod geometry;
mod player;
mod window;

use combat::action::Action;
use combat::combat::combat::process_combat_queue;
use combat::event::Event;
use combat::event_queue::EventQueue;
use enemy::enemy::Enemy;
use event::EventHandler;
use geometry::position::Position;
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
const TARGET_FPS: u32 = 60;
const DEBUG_FPS: bool = true;

struct MainState {
    event_queue: EventQueue,
    player: Player,
    enemy: Enemy,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let event_queue: EventQueue = EventQueue::new();
        let player: Player = Player::new();
        let enemy: Enemy = Enemy::new();

        Ok(MainState {
            event_queue,
            player,
            enemy,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS) {
            let player_events: Vec<Event> = self.player.update(ctx);
            for event in player_events {
                self.event_queue.push(event);
            }

            let player_position: Position = self.player.get_last_player_position();
            let enemy_events: Vec<Event> = self.enemy.update(player_position);
            for event in enemy_events {
                self.event_queue.push(event);
            }

            let actions: Vec<Action> = process_combat_queue(self.event_queue.get_events());
            self.player.apply_game_actions(actions.clone());
            self.enemy.apply_game_actions(actions.clone());
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let clear: Color = Color::from([0.4, 0.4, 0.4, 1.0]);
        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

        self.player.draw(ctx, &mut canvas);
        self.enemy.draw(ctx, &mut canvas);

        canvas.finish(ctx)?;

        if DEBUG_FPS {
            let fps = ctx.time.fps();
            println!("FPS: {:.0}", fps);
        }

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
