use std::collections::HashMap;

use vector2d::Vector2Df;

use crate::entity::{
    entity_registry::{EntityBoneId, EntityBoneTemplateId, EntityRegistry},
    logic::{bone::EntityBoneLogic, joint::EntityJointLogic},
};

pub struct EntityJoint {
    bones_involved: (EntityBoneId, EntityBoneId),
}

pub struct EntityJointTemplate {
    bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
}

impl EntityJointTemplate {
    pub fn new(
        bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
    ) -> EntityJointTemplate {
        EntityJointTemplate { bones_involved }
    }

    pub fn build(&self, mapping: &HashMap<EntityBoneTemplateId, EntityBoneId>) -> EntityJoint {
        EntityJoint {
            bones_involved: (
                mapping[&self.bones_involved.0],
                mapping[&self.bones_involved.1],
            ),
        }
    }
}

pub struct EntityJointSnapshot {
    bone_vectors: (Vector2Df, Vector2Df),
}

impl EntityJointLogic for EntityJointSnapshot {
    fn bone_vectors(&self) -> (Vector2Df, Vector2Df) {
        self.bone_vectors
    }
}

impl EntityJoint {
    pub fn get_snapshot(&self, registry: &EntityRegistry) -> EntityJointSnapshot {
        // Don't care about remounting when getting joint snapshot
        let remounting = false;
        let bones = (
            registry
                .get_bone(self.bones_involved.0)
                .get_snapshot(registry, remounting),
            registry
                .get_bone(self.bones_involved.0)
                .get_snapshot(registry, remounting),
        );
        EntityJointSnapshot {
            bone_vectors: (bones.0.vector(), bones.1.vector()),
        }
    }
}
