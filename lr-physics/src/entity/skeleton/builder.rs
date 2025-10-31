use vector2d::Vector2Df;

use crate::entity::{
    bone::{builder::EntityBoneBuilder, template::EntityBoneTemplate},
    joint::{builder::EntityJointBuilder, template::EntityJointTemplate},
    point::{builder::EntityPointBuilder, template::EntityPointTemplate},
    registry::{
        EntityBoneTemplateId, EntityJointTemplateId, EntityPointTemplateId, EntityRegistry,
        EntitySkeletonTemplateId,
    },
    skeleton::template::EntitySkeletonTemplate,
};

pub struct EntitySkeletonBuilder<'a> {
    registry: &'a mut EntityRegistry,
    points: Vec<EntityPointTemplateId>,
    bones: Vec<EntityBoneTemplateId>,
    joints: Vec<EntityJointTemplateId>,
    remount_enabled: bool,
    dismounted_timer: Option<u32>,
    remounting_timer: Option<u32>,
    remounted_timer: Option<u32>,
}

impl<'a> EntitySkeletonBuilder<'a> {
    pub(crate) fn new(registry: &'a mut EntityRegistry) -> EntitySkeletonBuilder {
        EntitySkeletonBuilder {
            registry,
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            remount_enabled: false,
            dismounted_timer: None,
            remounting_timer: None,
            remounted_timer: None,
        }
    }

    pub(crate) fn add_point(
        &'a mut self,
        point_template: EntityPointTemplate,
    ) -> EntityPointTemplateId {
        let id = self.registry.add_point_template(point_template);
        self.points.push(id);
        id
    }

    pub(crate) fn add_bone(
        &'a mut self,
        bone_template: EntityBoneTemplate,
    ) -> EntityBoneTemplateId {
        let id = self.registry.add_bone_template(bone_template);
        self.bones.push(id);
        id
    }

    pub(crate) fn add_joint(
        &'a mut self,
        joint_template: EntityJointTemplate,
    ) -> EntityJointTemplateId {
        let id = self.registry.add_joint_template(joint_template);
        self.joints.push(id);
        id
    }

    pub fn point(&'a mut self, initial_position: Vector2Df) -> EntityPointBuilder<'a, '_> {
        EntityPointBuilder::new(self, initial_position)
    }

    pub fn bone(
        &'a mut self,
        p1: EntityPointTemplateId,
        p2: EntityPointTemplateId,
    ) -> EntityBoneBuilder<'a, '_> {
        EntityBoneBuilder::new(self, p1, p2)
    }

    pub fn joint(
        &'a mut self,
        b1: EntityBoneTemplateId,
        b2: EntityBoneTemplateId,
    ) -> EntityJointBuilder<'a, '_> {
        EntityJointBuilder::new(self, b1, b2)
    }

    pub fn enable_remount(&mut self) -> &mut Self {
        self.remount_enabled = true;
        self
    }

    pub fn dismounted_timer(&mut self, duration: u32) -> &mut Self {
        self.dismounted_timer = Some(duration);
        self
    }

    pub fn remounting_timer(&mut self, duration: u32) -> &mut Self {
        self.remounting_timer = Some(duration);
        self
    }

    pub fn remounted_timer(&mut self, duration: u32) -> &mut Self {
        self.remounted_timer = Some(duration);
        self
    }

    pub fn build(self) -> EntitySkeletonTemplateId {
        let skeleton_template = EntitySkeletonTemplate {
            points: self.points,
            bones: self.bones,
            joints: self.joints,
            remount_enabled: self.remount_enabled,
            dismounted_timer: self.dismounted_timer.unwrap_or(0),
            remounting_timer: self.remounting_timer.unwrap_or(0),
            remounted_timer: self.remounted_timer.unwrap_or(0),
        };
        let id = self.registry.add_skeleton_template(skeleton_template);
        id
    }
}
