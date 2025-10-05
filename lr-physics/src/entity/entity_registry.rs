use std::{collections::HashMap, hash::Hash};

use crate::entity::{
    bone::{EntityBone, EntityBoneTemplate},
    joint::{EntityJoint, EntityJointTemplate},
    point::{EntityPoint, EntityPointTemplate},
    skeleton::{EntitySkeleton, EntitySkeletonTemplate},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityPointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityBoneId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityJointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntitySkeletonId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityPointTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityBoneTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityJointTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntitySkeletonTemplateId(usize);

pub struct EntityRegistry {
    points: HashMap<EntityPointId, EntityPoint>,
    bones: HashMap<EntityBoneId, EntityBone>,
    joints: HashMap<EntityJointId, EntityJoint>,
    skeletons: HashMap<EntitySkeletonId, EntitySkeleton>,
    point_templates: HashMap<EntityPointTemplateId, EntityPointTemplate>,
    bone_templates: HashMap<EntityBoneTemplateId, EntityBoneTemplate>,
    joint_templates: HashMap<EntityJointTemplateId, EntityJointTemplate>,
    skeleton_templates: HashMap<EntitySkeletonTemplateId, EntitySkeletonTemplate>,
}

// Note: unchecked index access is safe here, because users shouldn't be able to
// construct their own Id structs to pass in (we provide them)
impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            bones: HashMap::new(),
            joints: HashMap::new(),
            skeletons: HashMap::new(),
            point_templates: HashMap::new(),
            bone_templates: HashMap::new(),
            joint_templates: HashMap::new(),
            skeleton_templates: HashMap::new(),
        }
    }

    pub fn get_point(&self, id: EntityPointId) -> &EntityPoint {
        &self.points[&id]
    }

    pub fn get_point_mut(&mut self, id: EntityPointId) -> &mut EntityPoint {
        self.points.get_mut(&id).unwrap()
    }

    pub fn get_bone(&self, id: EntityBoneId) -> &EntityBone {
        &self.bones[&id]
    }

    pub fn get_joint(&self, id: EntityJointId) -> &EntityJoint {
        &self.joints[&id]
    }

    pub fn add_point_template(&mut self, template: EntityPointTemplate) -> EntityPointTemplateId {
        let id = EntityPointTemplateId(self.point_templates.len());
        self.point_templates.insert(id, template);
        id
    }

    pub fn add_bone_template(&mut self, template: EntityBoneTemplate) -> EntityBoneTemplateId {
        let id = EntityBoneTemplateId(self.bone_templates.len());
        self.bone_templates.insert(id, template);
        id
    }

    pub fn add_joint_template(&mut self, template: EntityJointTemplate) -> EntityJointTemplateId {
        let id = EntityJointTemplateId(self.joint_templates.len());
        self.joint_templates.insert(id, template);
        id
    }

    pub fn add_skeleton_template(
        &mut self,
        template: EntitySkeletonTemplate,
    ) -> EntitySkeletonTemplateId {
        let id = EntitySkeletonTemplateId(self.skeleton_templates.len());
        self.skeleton_templates.insert(id, template);
        id
    }

    pub fn get_skeleton(&self, id: EntitySkeletonId) -> &EntitySkeleton {
        &self.skeletons[&id]
    }

    pub fn get_skeleton_template(&self, id: EntitySkeletonTemplateId) -> &EntitySkeletonTemplate {
        &self.skeleton_templates[&id]
    }

    pub fn add_skeleton(&mut self, template_id: EntitySkeletonTemplateId) {
        let template = &self.skeleton_templates[&template_id];
    }

    pub fn remove_skeleton(&mut self, id: EntitySkeletonId) {
        let skeleton = self.skeletons.remove(&id).unwrap();

        for joint in skeleton.joints() {
            self.joints.remove(&joint);
        }

        for bone in skeleton.bones() {
            self.bones.remove(&bone);
        }

        for point in skeleton.points() {
            self.points.remove(&point);
        }
    }
}
