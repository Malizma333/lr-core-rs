use crate::entity::{
    bone::EntityBone, joint::EntityJoint, point::EntityPoint, skeleton::EntitySkeleton,
};

pub type EntityRegistryIndex = usize;

pub struct EntityRegistry {
    points: Vec<EntityPoint>,
    bones: Vec<EntityBone>,
    joints: Vec<EntityJoint>,
    skeletons: Vec<EntitySkeleton>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        EntityRegistry {
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            skeletons: Vec::new(),
        }
    }

    pub fn get_point(&self, index: EntityRegistryIndex) -> &EntityPoint {
        &self.points[index]
    }

    pub fn get_point_mut(&mut self, index: EntityRegistryIndex) -> &mut EntityPoint {
        &mut self.points[index]
    }

    pub fn get_bone(&self, index: EntityRegistryIndex) -> &EntityBone {
        &self.bones[index]
    }

    pub fn get_joint(&self, index: EntityRegistryIndex) -> &EntityJoint {
        &self.joints[index]
    }

    pub fn add_skeleton(&mut self) -> EntityRegistryIndex {
        todo!()
    }

    pub fn remove_skeleton(&mut self, index: EntityRegistryIndex) {
        let skeleton = self.skeletons.remove(index);

        // makes the assumption that skeleton.joint order is ascending by index, and is the same as engine processing order of joints
        // maybe we should ensure this somehow?
        for joint in skeleton.joints().iter().rev() {
            self.joints.remove(*joint);
        }

        for bone in skeleton.bones().iter().rev() {
            self.bones.remove(*bone);
        }

        for point in skeleton.points().iter().rev() {
            self.points.remove(*point);
        }

        todo!("update skeleton.other_skeleton mount state")
    }
}
