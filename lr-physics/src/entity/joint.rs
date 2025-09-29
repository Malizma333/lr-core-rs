use vector2d::Vector2Df;

use crate::{
    engine::{Engine, EntityRegistryIndex},
    entity::bone::EntityBoneLogic,
};

pub struct EntityJoint {
    bones_involved: (EntityRegistryIndex, EntityRegistryIndex),
}

pub struct EntityJointBuilder {
    bones_involved: Option<(EntityRegistryIndex, EntityRegistryIndex)>,
}

#[derive(Debug, Clone)]
pub enum EntityJointBuilderError {
    MissingBones,
}

impl EntityJointBuilder {
    pub fn new() -> EntityJointBuilder {
        EntityJointBuilder {
            bones_involved: None,
        }
    }

    pub fn bones(&mut self, bone1: EntityRegistryIndex, bone2: EntityRegistryIndex) -> &mut Self {
        self.bones_involved = Some((bone1, bone2));
        self
    }

    pub fn build(&self) -> Result<EntityJoint, EntityJointBuilderError> {
        if let Some(bones_involved) = self.bones_involved {
            Ok(EntityJoint { bones_involved })
        } else {
            Err(EntityJointBuilderError::MissingBones)
        }
    }
}

impl EntityJoint {
    pub fn get_intact(&self, engine: &Engine) -> bool {
        // TODO refactor this like bones
        Vector2Df::cross(
            engine
                .get_bone(self.bones_involved.0)
                .get_snapshot(engine, false)
                .get_vector(),
            engine
                .get_bone(self.bones_involved.1)
                .get_snapshot(engine, false)
                .get_vector(),
        ) < 0.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_intact() {
        todo!("bones parallel");
        todo!("bones perpendicular");
        todo!("bones positive angled not crossing");
        todo!("bones negative angled not crossing");
    }
}
