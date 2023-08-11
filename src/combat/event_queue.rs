use crate::combat::event::Event;
use std::collections::VecDeque;

pub struct EventQueue {
    events: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn get_events(&mut self) -> VecDeque<Event> {
        self.events.drain(..).collect()
    }
}
