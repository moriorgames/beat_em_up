mod character;
mod combat;
mod enemy;
mod game;
mod geometry;
mod level_up;
mod player;
mod sprite;
mod window;
mod world;

use character::character::Character;
use character::character_builder::character_builder;
use combat::combat::Combat;
use event::EventHandler;
use game::{game_draw_in_combat, game_draw_level_up, game_update_in_combat, game_update_level_up};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use level_up::level_up::LevelUp;
use player::player::Player;
use player::player_controls::PlayerControls;
use sprite::sprite_repository::SpriteRepository;
use std::path::PathBuf;
use window::window::Window;
use world::world::World;
use world::world_builder::world_builder;

const GAME_ID: &str = "Beat 'em up";
const AUTHOR: &str = "MoriorGames";
pub const TARGET_FPS: u32 = 60;
const DEBUG_FPS: bool = false;

pub enum GameState {
    InCombat,
    LevelUp,
}

pub struct MainState {
    player: Player,
    player_controls: PlayerControls,
    world: World,
    characters: Vec<Character>,
    sprite_repository: SpriteRepository,
    combat: Combat,
    level_up: LevelUp,
    current_state: GameState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let player_character: Character = character_builder::build_player();
        let cloned_player_character: Character = player_character.clone();
        let player: Player = Player::new(player_character);
        let player_controls: PlayerControls = PlayerControls::new();
        let world: World = world_builder::build();
        let mut characters: Vec<Character> = Vec::new();
        characters.push(cloned_player_character);
        characters.push(character_builder::spawn_first_tower());
        let sprite_repository: SpriteRepository = SpriteRepository::new(ctx);
        let combat: Combat = Combat::new();
        let level_up: LevelUp = LevelUp::new();
        let current_state: GameState = GameState::InCombat;

        Ok(MainState {
            player,
            player_controls,
            world,
            characters,
            sprite_repository,
            combat,
            level_up,
            current_state,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.current_state {
            GameState::InCombat => game_update_in_combat::execute(ctx, self),
            GameState::LevelUp => game_update_level_up::execute(ctx, self),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now: std::time::Instant = std::time::Instant::now();

        let clear: Color = Color::from([0.4, 0.6, 0.3, 1.0]);

        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

        match self.current_state {
            GameState::InCombat => game_draw_in_combat::execute(ctx, &mut canvas, self),
            GameState::LevelUp => game_draw_level_up::execute(ctx, &mut canvas, self),
        }

        canvas.finish(ctx)?;

        if DEBUG_FPS {
            let fps: f64 = ctx.time.fps();
            println!("FPS: {:.0}", fps);
            println!("draw {:?}", now.elapsed());
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir: PathBuf = PathBuf::from("./resources");
    let setup: WindowSetup = Window::create_window_setup();
    let mode: WindowMode = Window::create_window_mode();
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR)
        .add_resource_path(resource_dir)
        .window_setup(setup)
        .window_mode(mode);
    let (mut ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
