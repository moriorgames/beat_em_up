use uuid::Uuid;

pub enum Event {
    MoveLeft { id: Uuid },
    MoveRight { id: Uuid },
    MoveUp { id: Uuid },
    MoveDown { id: Uuid },
    Attack { id: Uuid },
}
