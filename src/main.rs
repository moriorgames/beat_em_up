mod character;
mod combat;
mod enemy;
mod geometry;
mod player;
mod sprite;
mod window;
mod world;

use character::character::Character;
use character::character_builder::character_builder;
use character::character_types::CharacterTypes;
use character::character_view::character_view::draw_characters;
use combat::action::Action;
use combat::combat::Combat;
use enemy::enemy_behaviour::enemy_behavior::update_enemy_behaviour;
use event::EventHandler;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use player::player::Player;
use player::player_controls::PlayerControls;
use sprite::sprite_repository::SpriteRepository;
use std::path::PathBuf;
use window::window::Window;
use world::world::World;
use world::world_builder::world_builder;
use world::world_view::character_view::draw;

const GAME_ID: &str = "Beat 'em up";
const AUTHOR: &str = "MoriorGames";
const TARGET_FPS: u32 = 50;
const DEBUG_FPS: bool = false;

pub enum GameState {
    InCombat,
    LevelUp,
}

struct MainState {
    player: Player,
    player_controls: PlayerControls,
    world: World,
    characters: Vec<Character>,
    sprite_repository: SpriteRepository,
    combat: Combat,
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
        let current_state: GameState = GameState::InCombat;

        Ok(MainState {
            player,
            player_controls,
            world,
            characters,
            sprite_repository,
            combat,
            current_state,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.current_state {
            GameState::InCombat => update_in_combat(ctx, self),
            GameState::LevelUp => update_level_up(),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now: std::time::Instant = std::time::Instant::now();

        let clear: Color = Color::from([0.4, 0.6, 0.3, 1.0]);

        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

        match self.current_state {
            GameState::InCombat => draw_in_combat(ctx, &mut canvas, self),
            GameState::LevelUp => draw_level_up(),
        }

        let _ = draw(ctx, &mut canvas, &self.world, &self.sprite_repository);

        let _ = draw_characters(ctx, &mut canvas, &self.characters, &self.sprite_repository);
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

fn update_in_combat(ctx: &mut Context, main_state: &mut MainState) {
    while ctx.time.check_update_time(TARGET_FPS) {
        if let Some(player) = main_state.characters.first() {
            match player.character_type {
                CharacterTypes::Player => {
                    let player: Character = player.clone();
                    let player_actions: Vec<Action> = main_state.player_controls.handle_input(
                        ctx,
                        &player,
                        main_state.combat.turn,
                    );
                    for action in player_actions {
                        main_state.combat.add_action(action)
                    }

                    let enemy_actions: Vec<Action> = update_enemy_behaviour(
                        main_state.characters.clone(),
                        &player,
                        main_state.combat.turn,
                    );
                    for action in enemy_actions {
                        main_state.combat.add_action(action);
                    }
                }
                _ => (),
            }
        }

        main_state
            .combat
            .process(&mut main_state.characters, &main_state.world);

        main_state
            .characters
            .retain(|character| character.current_health > 0.0);

        if main_state.combat.turn == 3000 {
            main_state
                .characters
                .push(character_builder::spawn_first_tower_orc_lord());
        } else if main_state.combat.turn > 3000 {
        } else {
            if main_state.characters.len() < 7 {
                if main_state.combat.turn % 1033 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_second_tower());
                } else if main_state.combat.turn % 617 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_third_tower());
                } else if main_state.combat.turn % 293 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_first_tower());
                }
            }
        }
    }
}

fn update_level_up() {}

fn draw_in_combat(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    let _ = draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );

    let _ = draw_characters(
        ctx,
        canvas,
        &main_state.characters,
        &main_state.sprite_repository,
    );
}

fn draw_level_up() {}
