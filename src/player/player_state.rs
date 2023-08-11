use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

pub struct PlayerState {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    attack_frame_counter: u32,
    is_attacking: bool,
}

impl PlayerState {
    pub const W: f32 = 55.0;
    pub const H: f32 = 45.0;
    const SPEED: f32 = 3.7;
    const ATTACK_FRAME_SPEED: u32 = 15;

    pub fn new() -> Self {
        let id: Uuid = Uuid::new_v4();

        PlayerState {
            id,
            position: Position::new(400.0, 300.0),
            size: Size::new(Self::W, Self::H),
            attack_frame_counter: 0,
            is_attacking: false,
        }
    }

    pub fn attack(&mut self) {
        if !self.is_attacking {
            self.is_attacking = true;
            self.attack_frame_counter = Self::ATTACK_FRAME_SPEED;
        }
    }

    pub fn update(&mut self) {
        if self.is_attacking {
            if self.attack_frame_counter > 0 {
                self.attack_frame_counter -= 1;
            } else {
                self.is_attacking = false;
            }
        }
    }

    pub fn move_left(&mut self) {
        if !self.is_attacking {
            self.position.x -= Self::SPEED;
        }
    }

    pub fn move_right(&mut self) {
        if !self.is_attacking {
            self.position.x += Self::SPEED;
        }
    }

    pub fn move_up(&mut self) {
        if !self.is_attacking {
            self.position.y -= Self::SPEED;
        }
    }

    pub fn move_down(&mut self) {
        if !self.is_attacking {
            self.position.y += Self::SPEED;
        }
    }
}
