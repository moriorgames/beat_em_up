use crate::{
    character::{box_collision::BoxCollision, character::Character},
    combat::action::Action,
    world::world::World,
};

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
                Action::StartAttacking { .. } => self.process_start_attacking(action, characters),
                Action::Attacking { .. } => self.process_attacking(action, characters),
                Action::Damage { .. } => self.process_damage(action, characters),
            }
        }

        println!("Quantity Actions: {}", self.actions.len());

        self.clean_actions();
    }

    fn process_start_attacking(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::StartAttacking { id, from, to } = action {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if character.is_idle() {
                        character.start_attacking();
                        self.actions.push(Action::Attacking {
                            id: character.id,
                            damage: 10.0,
                            from: self.turn,
                            to: self.turn + 22,
                        });
                    }
                }
            }
        }
    }

    fn process_attacking(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Attacking {
            id,
            damage,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                let bodies: Vec<Character> = characters.clone();
                for weapon in characters.iter_mut().filter(|c| c.id == id) {
                    for body in bodies.iter() {
                        if weapon.id != body.id {
                            if BoxCollision::collides_with(
                                body.position.clone(),
                                &body.body_collision,
                                weapon.position.clone(),
                                &weapon.weapon_collision,
                            ) {
                                let action: Action = Action::Damage {
                                    id: body.id,
                                    damage: damage,
                                    from: self.turn + 1,
                                    to: self.turn + 2,
                                };
                                self.actions.push(action);
                            }
                        }
                    }

                    if self.turn == to {
                        weapon.back_to_idle();
                    }
                }
            }
        }
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
                    character.apply_damage(damage);
                }
            }
        }
    }

    fn clean_actions(&mut self) {
        self.actions.retain(|action| match action {
            Action::Moving { to, .. } => self.turn <= *to,
            Action::StartAttacking { to, .. } => self.turn <= *to,
            Action::Attacking { to, .. } => self.turn <= *to,
            Action::Damage { to, .. } => self.turn <= *to,
        });
    }
}
