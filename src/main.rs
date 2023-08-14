mod behaviours;
mod character;
mod combat;
mod enemy;
mod geometry;
mod player;
mod window;

use character::character::Character;
use character::character_types::CharacterTypes;
use character::character_view::character_view::draw_characters;
use combat::combat::combat::process_combat_queue;
use combat::event::Event;
use combat::event_queue::EventQueue;
use enemy::enemy_behaviour::enemy_behavior::update_enemy_behaviour;
use event::EventHandler;
use geometry::position::Position;
use geometry::size::Size;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use player::player_controls::PlayerControls;
use uuid::Uuid;
use window::window::Window;

const GAME_ID: &str = "Beat 'em up";
const AUTHOR: &str = "MoriorGames";
const TARGET_FPS: u32 = 60;
const DEBUG_FPS: bool = true;

struct MainState {
    event_queue: EventQueue,
    player_controls: PlayerControls,
    characters: Vec<Character>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let event_queue: EventQueue = EventQueue::new();

        let mut characters: Vec<Character> = Vec::new();

        // Player
        let position: Position = Position::new(400.0, 300.0);
        let size: Size = Size::new(55.0, 45.0);
        let speed: f32 = 3.7;
        let max_health: f32 = 1000.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let player: Character = Character::new(position, size, speed, max_health, character_type);
        let player_id: Uuid = player.id;
        let player_controls: PlayerControls = PlayerControls::new(player_id);
        characters.push(player);

        // Enemy
        let position: Position = Position::new(600.0, 400.0);
        let size: Size = Size::new(45.0, 35.0);
        let speed: f32 = 1.3;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type);
        characters.push(enemy);

        // Enemy
        let position: Position = Position::new(700.0, 800.0);
        let size: Size = Size::new(45.0, 35.0);
        let speed: f32 = 1.3;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type);
        characters.push(enemy);

        Ok(MainState {
            event_queue,
            player_controls,
            characters,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS) {
            let player_events: Vec<Event> = self.player_controls.handle_input(ctx);
            for event in player_events {
                self.event_queue.push(event);
            }

            if let Some(player) = self.characters.first() {
                match player.character_type {
                    CharacterTypes::Player => {
                        let player_position: Position = player.position.clone();
                        let enemy_events: Vec<Event> =
                            update_enemy_behaviour(self.characters.clone(), player_position);
                        for event in enemy_events {
                            self.event_queue.push(event);
                        }
                    }
                    _ => (),
                }
            }

            process_combat_queue(self.event_queue.get_events(), &mut self.characters);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now: std::time::Instant = std::time::Instant::now();

        let clear: Color = Color::from([0.4, 0.6, 0.3, 1.0]);
        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

        let _ = draw_characters(ctx, &mut canvas, &self.characters);
        canvas.finish(ctx)?;

        if DEBUG_FPS {
            let fps = ctx.time.fps();
            println!("FPS: {:.0}", fps);
        }

        println!("draw {:?}", now.elapsed());
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
