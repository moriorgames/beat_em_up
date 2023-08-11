use crate::geometry::position::Position;

pub trait Movable {
    fn position(&self) -> &Position;
    fn position_mut(&mut self) -> &mut Position;
    fn speed(&self) -> f32;

    fn move_left(&mut self) {
        self.position_mut().x -= self.speed();
    }

    fn move_right(&mut self) {
        self.position_mut().x += self.speed();
    }

    fn move_up(&mut self) {
        self.position_mut().y -= self.speed();
    }

    fn move_down(&mut self) {
        self.position_mut().y += self.speed();
    }
}
