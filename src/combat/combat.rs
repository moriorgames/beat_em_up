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

    pub fn process(&mut self, characters: &mut Vec<Character>, world: &World) {
        self.turn += 1;
        println!("Combat Turn: {:?}", self.turn);

        for character in &mut *characters {
            character.update();
        }

        for action in self.actions.clone() {
            // println!("Action: {:?}", action.clone());

            match action {
                Action::Attack { .. } => self.process_attacking(action, characters),
                Action::Move { .. } => self.process_moving(action, characters, world),
                Action::Jump { .. } => self.process_jumping(action, characters, world),
                Action::BackJump { .. } => self.process_jumping(action, characters, world),
                Action::CounterAttack { .. } => {
                    self.process_counter_attack(action, characters, world)
                }
                Action::Damage { .. } => self.process_damage(action, characters),
            }
        }

        println!("Quantity Actions: {}", self.actions.len());

        self.clean_actions();
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    fn clean_actions(&mut self) {
        self.actions.retain(|action| match action {
            Action::Attack { to, .. } => self.turn <= *to,
            Action::Move { to, .. } => self.turn <= *to,
            Action::Jump { to, .. } => self.turn <= *to,
            Action::BackJump { to, .. } => self.turn <= *to,
            Action::CounterAttack { to, .. } => self.turn <= *to,
            Action::Damage { from, .. } => self.turn < *from,
        });
    }
}
