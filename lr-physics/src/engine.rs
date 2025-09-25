use crate::entity::{
    bone::EntityBone, joint::EntityJoint, point::EntityPoint, skeleton::EntitySkeleton,
};

// TODO data structure w/ operations:
// - adding and removing individual skeletons (cascade delete)
// - adding and removing mounts (unlink skeletons)
// - caching point states for each frame (basic clone)
// - caching mount states for each frame (basic clone)
// - removing from and adding to cache size whenever skeletons/mounts change
// - clearing front of cache whenever lines or skeletons (remounting) change
// - hot swapping cached point states whenever frame info requested

// Engine Constraints
// Skeletons: <= 50
// Bones: <= 2,500
// Points: <= 2,500
// Mounts: <= 25
// Physics Lines <= 10,000
// Scenery Lines <= ~100,000,000?

/*
Call free on skeleton/mount, which frees up lower components?
When adding a new one, rebuild from reference data
  Reset cache back to frame 0? (or use smart caching)
Processing in order (Vec)

 */

pub type EntityRegistryIndex = usize;

pub struct EntityRegistry {
    points: Vec<EntityPoint>,
    bones: Vec<EntityBone>,
    joints: Vec<EntityJoint>,
    skeletons: Vec<EntitySkeleton>,
}

impl EntityRegistry {
    pub fn get_point(&self, index: EntityRegistryIndex) -> &EntityPoint {
        &self.points[index]
    }

    pub fn get_bone(&self, index: EntityRegistryIndex) -> &EntityBone {
        &self.bones[index]
    }

    pub fn get_joint(&self, index: EntityRegistryIndex) -> &EntityJoint {
        &self.joints[index]
    }

    pub fn get_skeleton(&self, index: EntityRegistryIndex) -> &EntitySkeleton {
        &self.skeletons[index]
    }
}
