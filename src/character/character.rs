use super::animation::Animation;
use super::box_collision::BoxCollision;
use super::character_state::CharacterState;
use super::character_types::{CharacterTypes, Facing};
use super::stats::Stats;
use crate::combat::action::DAMAGE_DELAY;
use crate::combat::direction::Direction;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Character {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub stats: Stats,
    pub current_health: f32,
    pub health: f32,
    pub speed: f32,
    pub speed_jump: f32,
    pub fast_damage: f32,
    pub slow_damage: f32,
    pub counter_damage: f32,
    pub defense: f32,
    pub current_stamina: f32,
    pub stamina: f32,
    pub action_processed: bool,
    pub character_type: CharacterTypes,
    pub facing: Facing,
    pub move_animation: Animation,
    pub fast_attack_animation: Animation,
    pub slow_attack_animation: Animation,
    pub jump_animation: Animation,
    pub full_jump_timer: u8,
    pub jump_timer: u8,
    pub fast_attack_timer: u8,
    pub fast_attack_timer_hit: u8,
    pub slow_attack_timer: u8,
    pub slow_attack_timer_hit: u8,
    pub attack_timer: u8,
    pub damage_timer: u8,
    pub damage_processed: bool,
    pub body_collision: BoxCollision,
    pub foot_collision: BoxCollision,
    pub weapon_collision_template: BoxCollision,
    pub weapon_collision: Option<BoxCollision>,
    pub character_state: CharacterState,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        stats: Stats,
        character_type: CharacterTypes,
        move_animation: Animation,
        fast_attack_animation: Animation,
        slow_attack_animation: Animation,
        jump_animation: Animation,
        body_collision: BoxCollision,
        foot_collision: BoxCollision,
        weapon_collision_template: BoxCollision,
    ) -> Self {
        let id: Uuid = match character_type {
            CharacterTypes::Player => {
                Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
            }
            _ => Uuid::new_v4(),
        };
        let full_jump_timer: u8 = jump_animation.move_frames * jump_animation.delay;
        let fast_attack_timer: u8 = fast_attack_animation.move_frames * fast_attack_animation.delay;
        let fast_attack_timer_hit: u8 = fast_attack_timer / 3 + 2;
        let slow_attack_timer: u8 = slow_attack_animation.move_frames * slow_attack_animation.delay;
        let slow_attack_timer_hit: u8 = slow_attack_timer / 3 + 2;
        let (health, speed, speed_jump, fast_damage, slow_damage, counter_damage, defense, stamina) =
            stats.get_calculated_stats();
        Character {
            id,
            position,
            size,
            stats,
            current_health: health,
            health,
            speed,
            speed_jump,
            fast_damage,
            slow_damage,
            counter_damage,
            defense,
            current_stamina: stamina,
            stamina,
            action_processed: false,
            character_type,
            facing: Facing::Right,
            move_animation,
            fast_attack_animation,
            slow_attack_animation,
            jump_animation,
            full_jump_timer,
            jump_timer: 0,
            fast_attack_timer,
            fast_attack_timer_hit,
            slow_attack_timer,
            slow_attack_timer_hit,
            attack_timer: 0,
            damage_timer: 0,
            damage_processed: false,
            body_collision,
            foot_collision,
            weapon_collision_template,
            weapon_collision: None,
            character_state: CharacterState::Idle,
        }
    }

    pub fn rebuild_stats(&mut self) {
        let (health, speed, speed_jump, fast_damage, slow_damage, counter_damage, defense, stamina) =
            self.stats.get_calculated_stats();
        self.current_health = health;
        self.health = health;
        self.speed = speed;
        self.speed_jump = speed_jump;
        self.fast_damage = fast_damage;
        self.slow_damage = slow_damage;
        self.counter_damage = counter_damage;
        self.defense = defense;
        self.current_stamina = stamina;
        self.stamina = stamina;
    }

    pub fn back_to_idle(&mut self) {
        self.fast_attack_animation.reset();
        self.slow_attack_animation.reset();
        self.jump_animation.reset();
        self.character_state = CharacterState::Idle;
        self.attack_timer = 0;
        self.weapon_collision = None;
    }

    pub fn update(&mut self) {
        self.action_processed = false;
        self.regenerate_stamina();
        match self.character_state {
            CharacterState::Idle | CharacterState::Moving => {
                self.move_animation.update();
            }
            CharacterState::Defending => {}
            CharacterState::Jumping | CharacterState::BackJumping => {
                self.jump_animation.update();
                if self.jump_timer >= 1 {
                    self.jump_timer -= 1;
                } else {
                    self.back_to_idle()
                }
            }
            CharacterState::FastAttacking | CharacterState::CounterAttacking => {
                self.fast_attack_animation.update();
                if self.attack_timer >= 1 {
                    self.attack_timer -= 1;
                    if self.attack_timer <= self.fast_attack_timer_hit {
                        let x_offset: f32 = match self.facing {
                            Facing::Left => -self.weapon_collision_template.x,
                            Facing::Right => self.weapon_collision_template.x,
                        };
                        self.weapon_collision = Some(BoxCollision {
                            x: x_offset,
                            y: self.weapon_collision_template.y,
                            w: self.weapon_collision_template.w,
                            h: self.weapon_collision_template.h,
                        });
                    }
                } else {
                    self.back_to_idle()
                }
            }
            CharacterState::SlowAttacking => {
                self.slow_attack_animation.update();
                if self.attack_timer >= 1 {
                    self.attack_timer -= 1;
                    if self.attack_timer <= self.slow_attack_timer_hit {
                        let x_offset: f32 = match self.facing {
                            Facing::Left => -self.weapon_collision_template.x,
                            Facing::Right => self.weapon_collision_template.x,
                        };
                        self.weapon_collision = Some(BoxCollision {
                            x: x_offset,
                            y: self.weapon_collision_template.y,
                            w: self.weapon_collision_template.w,
                            h: self.weapon_collision_template.h,
                        });
                    }
                } else {
                    self.back_to_idle()
                }
            }
        }
        if self.damage_timer >= 1 {
            self.damage_timer -= 1;
        }
    }

    pub fn attack(&mut self) {
        if self.character_state == CharacterState::FastAttacking
            || self.character_state == CharacterState::SlowAttacking
        {
            self.action_processed = true;
        }
    }

    pub fn start_damaged(&mut self, damage: f32) {
        self.damage_timer = DAMAGE_DELAY;
        self.apply_damage(damage);
    }

    fn apply_damage(&mut self, damage: f32) {
        self.current_health -= damage;
        if self.current_health <= 0.0 {
            self.current_health = 0.0;
        }
        let push_x: f32 = match self.facing {
            Facing::Left => 1.5,
            Facing::Right => -1.5,
        };
        self.position.x += push_x;
    }

    pub fn reduce_stamina(&mut self, amount: f32) {
        self.current_stamina = (self.current_stamina - amount).max(-Stats::STAMINA_COST);
    }

    pub fn regenerate_stamina(&mut self) {
        self.current_stamina =
            (self.current_stamina + Stats::STAMINA_REGENERATION).min(self.stamina);
    }

    pub fn get_sprite_name(&self) -> String {
        match self.character_state {
            CharacterState::Idle | CharacterState::Moving => {
                let animation_frame: u8 =
                    self.move_animation.frame % self.move_animation.move_frames;
                let action_type: String = self.move_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.move_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Defending => {
                let animation_frame: u8 = 0;
                let action_type: String = "defense".to_string();

                format!(
                    "{}_{}_{}",
                    self.move_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Jumping | CharacterState::BackJumping => {
                let animation_frame: u8 =
                    self.jump_animation.frame % self.jump_animation.move_frames;
                let action_type: String = self.jump_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.jump_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::FastAttacking => {
                let animation_frame: u8 =
                    self.fast_attack_animation.frame % self.fast_attack_animation.move_frames;
                let action_type: String = self.fast_attack_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.fast_attack_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::SlowAttacking => {
                let animation_frame: u8 =
                    self.slow_attack_animation.frame % self.slow_attack_animation.move_frames;
                let action_type: String = self.slow_attack_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.slow_attack_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::CounterAttacking => {
                let animation_frame: u8 =
                    self.fast_attack_animation.frame % self.fast_attack_animation.move_frames;
                let action_type: String = "counter_attack".to_string();

                format!(
                    "{}_{}_{}",
                    self.fast_attack_animation.sprite, action_type, animation_frame
                )
            }
        }
    }

    pub fn next_foot_collision_to_world_space(&self, direction: Direction) -> BoxCollision {
        let mut x: f32 = self.position.x;
        let mut y: f32 = self.position.y;
        match direction {
            Direction::Left => x -= self.speed,
            Direction::Right => x += self.speed,
            Direction::Up => y -= self.speed,
            Direction::Down => y += self.speed,
            Direction::UpLeft => {
                y -= self.speed;
                x -= self.speed;
            }
            Direction::UpRight => {
                y -= self.speed;
                x += self.speed;
            }
            Direction::DownLeft => {
                y += self.speed;
                x -= self.speed;
            }
            Direction::DownRight => {
                y += self.speed;
                x += self.speed;
            }
        }

        let position: Position = Position::new(x, y);
        BoxCollision {
            x: position.x + self.foot_collision.x - self.foot_collision.w / 2.0,
            y: position.y + self.foot_collision.y - self.foot_collision.y / 2.0,
            w: self.foot_collision.w,
            h: self.foot_collision.h,
        }
    }

    pub fn body_collision_to_world_space(&self) -> BoxCollision {
        self.body_collision.to_world_space(self.position.clone())
    }

    pub fn foot_collision_to_world_space(&self) -> BoxCollision {
        self.foot_collision.to_world_space(self.position.clone())
    }

    pub fn weapon_collision_to_world_space(&self) -> Option<BoxCollision> {
        let weapon_collision: Option<BoxCollision> = self.weapon_collision.clone();
        weapon_collision.map(|collision| collision.to_world_space(self.position.clone()))
    }
}
