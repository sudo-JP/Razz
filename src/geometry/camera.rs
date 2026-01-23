use crate::math::Vec3;
use crate::geometry::viewport::Viewport;
use std::sync::Arc;

type Point3 = Vec3; 

pub struct Camera {
    pos: Point3,
    viewport: Arc<Viewport>,
}

impl Camera {
    pub fn new(point: Point3, viewport: Arc<Viewport>) -> Self {
        Self { pos: point, viewport: viewport }
    }

    pub fn get_pos(&self) -> Point3 { self.pos.clone() }
}
