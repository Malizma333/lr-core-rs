use std::collections::HashMap;

use crate::line::hitbox::Hitbox;

type LineId = u32;
type PhysicsLine = Box<dyn Hitbox>;

pub struct PhysicsLineManager {
    lines: HashMap<LineId, PhysicsLine>,
}

impl PhysicsLineManager {
    pub fn get_line(&self, id: LineId) -> Option<&PhysicsLine> {
        self.lines.get(&id)
    }

    pub fn get_line_mut(&mut self, id: LineId) -> Option<&mut PhysicsLine> {
        self.lines.get_mut(&id)
    }
}
