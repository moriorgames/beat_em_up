use crate::geometry::position::Position;

#[derive(Clone, Debug)]
pub struct BoxCollision {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl BoxCollision {
    pub fn is_inside(&self, outer_box: &BoxCollision) -> bool {
        let left_inside: bool = self.x >= outer_box.x;
        let right_inside: bool = self.x + self.w <= outer_box.x + outer_box.w;
        let top_inside: bool = self.y >= outer_box.y;
        let bottom_inside: bool = self.y + self.h <= outer_box.y + outer_box.h;

        left_inside && right_inside && top_inside && bottom_inside
    }

    pub fn collides_with(pa: Position, a: &BoxCollision, pb: Position, b: &BoxCollision) -> bool {
        let a: BoxCollision = a.to_world_space(pa);
        let b: BoxCollision = b.to_world_space(pb);

        let self_right: f32 = a.x + a.w;
        let self_bottom: f32 = a.y + a.h;
        let other_right: f32 = b.x + b.w;
        let other_bottom: f32 = b.y + b.h;

        if a.x < other_right && self_right > b.x && b.y < other_bottom && self_bottom > b.y {
            return true;
        }

        false
    }

    pub fn to_world_space(&self, position: Position) -> BoxCollision {
        BoxCollision {
            x: position.x + self.x - self.w / 2.0,
            y: position.y + self.y - self.y / 2.0,
            w: self.w,
            h: self.h,
        }
    }
}
