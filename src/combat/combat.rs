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
                Action::Moving { .. } => self.process_moving(action, characters, world),
                Action::Attacking { .. } => self.process_attacking(action, characters),
                Action::Jumping { .. } => self.process_jumping(action, characters, world),
                Action::Damage { .. } => self.process_damage(action, characters),
            }
        }

        println!("Quantity Actions: {}", self.actions.len());

        self.clean_actions();
    }

    fn process_damage(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Damage {
            id,
            damage,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if self.turn == to && character.is_damaged() {
                        character.back_to_idle()
                    }
                    if self.turn == from && !character.is_damaged() {
                        println!(" ------------ Action Damage {:?}", action);
                        character.start_damaged(damage)
                    }
                }
            }
        }
    }

    fn clean_actions(&mut self) {
        self.actions.retain(|action| match action {
            Action::Moving { to, .. } => self.turn <= *to,
            Action::Attacking { to, .. } => self.turn <= *to,
            Action::Jumping { to, .. } => self.turn <= *to,
            Action::Damage { to, .. } => self.turn <= *to,
        });
    }
}
