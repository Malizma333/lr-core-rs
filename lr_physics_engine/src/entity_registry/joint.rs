use vector2d::Vector2Df;

use crate::entity_registry::{EntityBoneId, EntityState, EntityTemplate};

/// Computed properties when built
struct Computed {
    is_mount: bool,
}

/// Constructed joint that holds props after building
pub struct EntityJoint {
    bones: (EntityBoneId, EntityBoneId),
    computed: Computed,
}

impl EntityJoint {
    pub(crate) fn bones(&self) -> (EntityBoneId, EntityBoneId) {
        self.bones
    }

    pub(crate) fn is_mount(&self) -> bool {
        self.computed.is_mount
    }

    pub(crate) fn should_break(&self, state: &EntityState, template: &EntityTemplate) -> bool {
        let bones = (
            template
                .bones()
                .get(&self.bones().0)
                .expect("Template should have bones needed for this joint"),
            template
                .bones()
                .get(&self.bones().1)
                .expect("Template should have bones needed for this joint"),
        );
        let bone0_p0 = state.point_state(&bones.0.point_ids().0);
        let bone0_p1 = state.point_state(&bones.0.point_ids().1);
        let bone1_p0 = state.point_state(&bones.1.point_ids().0);
        let bone1_p1 = state.point_state(&bones.1.point_ids().1);
        let bone_vectors = (
            bone0_p0.position().vector_from(bone0_p1.position()),
            bone1_p0.position().vector_from(bone1_p1.position()),
        );
        Vector2Df::cross(bone_vectors.0, bone_vectors.1) < 0.0
    }
}

/// Joint builder for custom skeletons
pub struct EntityJointBuilder {
    bones: (EntityBoneId, EntityBoneId),
    is_mount: bool,
}

impl EntityJointBuilder {
    pub fn new(b1: EntityBoneId, b2: EntityBoneId) -> EntityJointBuilder {
        Self {
            bones: (b1, b2),
            is_mount: false,
        }
    }

    // TODO remove this by using computed graph
    pub fn is_mount(mut self, is_mount: bool) -> Self {
        self.is_mount = is_mount;
        self
    }

    pub fn build(self) -> EntityJoint {
        EntityJoint {
            bones: self.bones,
            computed: Computed {
                is_mount: self.is_mount,
            },
        }
    }
}

impl From<EntityJoint> for EntityJointBuilder {
    fn from(joint: EntityJoint) -> Self {
        Self {
            bones: joint.bones,
            is_mount: joint.computed.is_mount,
        }
    }
}
