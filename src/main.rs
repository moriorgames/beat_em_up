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
use geometry::position::Position;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    graphics::{self},
    Context, ContextBuilder, GameError, GameResult,
};
use graphics::{Canvas, Color};
use player::player_controls::PlayerControls;
use sprite::sprite_repository::SpriteRepository;
use std::path::PathBuf;
use uuid::Uuid;
use window::window::Window;
use world::world::World;
use world::world_builder::world_builder;
use world::world_view::character_view::draw;

const GAME_ID: &str = "Beat 'em up";
const AUTHOR: &str = "MoriorGames";
const TARGET_FPS: u32 = 40;
const DEBUG_FPS: bool = false;

struct MainState {
    player_controls: PlayerControls,
    world: World,
    characters: Vec<Character>,
    sprite_repository: SpriteRepository,
    combat: Combat,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut player_id: Uuid = Uuid::new_v4();
        let characters: Vec<Character> = character_builder::build();
        if let Some(player) = characters.first() {
            player_id = player.id;
        }
        let player_controls: PlayerControls = PlayerControls::new(player_id);
        let world: World = world_builder::build();
        let sprite_repository: SpriteRepository = SpriteRepository::new(ctx);
        let combat: Combat = Combat::new();

        Ok(MainState {
            player_controls,
            world,
            characters,
            sprite_repository,
            combat,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS) {
            let player_actions: Vec<Action> =
                self.player_controls.handle_input(ctx, self.combat.turn);
            for action in player_actions {
                self.combat.add_action(action)
            }

            if let Some(player) = self.characters.first() {
                match player.character_type {
                    CharacterTypes::Player => {
                        let player_position: Position = player.position.clone();
                        let enemy_actions: Vec<Action> = update_enemy_behaviour(
                            self.characters.clone(),
                            player_position,
                            self.combat.turn,
                        );
                        for action in enemy_actions {
                            self.combat.add_action(action);
                        }
                    }
                    _ => (),
                }
            }

            self.combat.process(&mut self.characters, &self.world);

            self.characters.retain(|character| character.current_health > 0.0);
            if self.combat.turn % 700 == 0 {
                self.characters.push(character_builder::spawn_second_tower());
            } else if self.combat.turn % 450 == 0 {
                self.characters.push(character_builder::spawn_third_tower());
            } else if self.combat.turn % 200 == 0 {
                self.characters.push(character_builder::spawn_first_tower());
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now: std::time::Instant = std::time::Instant::now();

        let clear: Color = Color::from([0.4, 0.6, 0.3, 1.0]);

        let mut canvas: Canvas = Canvas::from_frame(ctx, clear);

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
