use crate::entity::{
    joint::template::EntityJointTemplate,
    registry::{EntityBoneTemplateId, EntityJointTemplateId},
    skeleton::builder::EntitySkeletonBuilder,
};

pub struct EntityJointBuilder<'a, 'b> {
    skeleton: &'a mut EntitySkeletonBuilder<'b>,
    bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
}

impl<'a: 'b, 'b> EntityJointBuilder<'a, 'b> {
    pub fn new(
        skeleton: &'a mut EntitySkeletonBuilder<'b>,
        b1: EntityBoneTemplateId,
        b2: EntityBoneTemplateId,
    ) -> EntityJointBuilder<'a, 'b> {
        Self {
            skeleton,
            bones_involved: (b1, b2),
        }
    }

    pub fn build(self) -> EntityJointTemplateId {
        let joint_template = EntityJointTemplate {
            bones_involved: self.bones_involved,
        };
        let id = self.skeleton.add_joint(joint_template);
        id
    }
}
