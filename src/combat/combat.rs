use crate::{character::character::Character, combat::action::Action, world::world::World};

pub struct Combat {
    pub turn: u128,
    pub actions: Vec<Action>,
}

impl Combat {
    pub fn new() -> Self {
        Self {
            turn: 0,
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn process(&mut self, characters: &mut Vec<Character>, world: &World) {
        self.turn += 1;
        println!("Turn: {:?}", self.turn);

        for character in &mut *characters {
            character.update();
        }

        for action in self.actions.clone() {
            match action {
                Action::StartMoving { .. } => self.process_start_moving(action, characters),
                Action::Moving { .. } => self.process_moving(action, characters, world),
                Action::Attack { .. } => self.process_attack_action(action, characters),
                _ => {}
            }
        }

        println!("Quantity Actions: {}", self.actions.len());

        self.clean_actions();
    }

    fn process_attack_action(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Attack { id, from, to } = action {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    character.attack();
                }
            }
        }
    }

    fn clean_actions(&mut self) {
        self.actions.retain(|action| match action {
            Action::StartMoving { to, .. } => self.turn <= *to,
            Action::Moving { to, .. } => self.turn <= *to,
            Action::Attack { to, .. } => self.turn <= *to,
            Action::Damage { to, .. } => self.turn <= *to,
        });
    }
}
