use crate::math::Vec3;

type Point3 = Vec3; 

pub struct Camera {
    pos: Point3,
}

impl Camera {
    pub fn new(point: Point3) -> Self {
        Self { pos: point }
    }

    pub fn get_pos(&self) -> Point3 { self.pos.clone() }
}
