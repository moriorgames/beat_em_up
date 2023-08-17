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
}
