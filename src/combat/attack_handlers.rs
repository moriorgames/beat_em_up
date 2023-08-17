pub mod attack_handlers {
    use crate::combat::{action::Action, event::Event};

    pub fn attack(event: &Event) -> Option<Action> {
        if let Event::Attack { id } = event {
            return Some(Action::Attack { id: *id });
        }
        None
    }
}
