use crate::{
    RemountVersion,
    entity::registry::{EntityBoneId, EntityJointId, EntityPointId},
};

pub(crate) struct EntitySkeleton {
    pub(super) points: Vec<EntityPointId>,
    pub(super) bones: Vec<EntityBoneId>,
    pub(super) joints: Vec<EntityJointId>,
    pub(super) remount_enabled: bool,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) mounted_timer: u32,
    pub(super) remount_version: RemountVersion,
}

impl EntitySkeleton {
    pub(crate) fn points(&self) -> &[EntityPointId] {
        &self.points
    }

    pub(crate) fn bones(&self) -> &[EntityBoneId] {
        &self.bones
    }

    pub(crate) fn joints(&self) -> &[EntityJointId] {
        &self.joints
    }

    pub(crate) fn remount_enabled(&self) -> bool {
        self.remount_enabled
    }

    pub(crate) fn dismounted_timer(&self) -> u32 {
        self.dismounted_timer
    }

    pub(crate) fn remounting_timer(&self) -> u32 {
        self.remounting_timer
    }

    pub(crate) fn mounted_timer(&self) -> u32 {
        self.mounted_timer
    }

    pub(crate) fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }
}
