use crate::{
    MountPhase,
    entity::registry::{EntityBoneId, EntityJointId, EntityPointId},
};

pub(crate) struct EntitySkeleton {
    pub(super) points: Vec<EntityPointId>,
    pub(super) bones: Vec<EntityBoneId>,
    pub(super) joints: Vec<EntityJointId>,
    pub(super) remount_enabled: bool,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) remounted_timer: u32,
    pub(super) use_initial_mount_phase_during_bones: bool,
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

    pub(crate) fn remounted_timer(&self) -> u32 {
        self.remounted_timer
    }

    pub(crate) fn use_initial_mount_phase_during_bones(&self) -> bool {
        self.use_initial_mount_phase_during_bones
    }

    pub(crate) fn get_phase_after_dismount(&self, current_mount_phase: MountPhase) -> MountPhase {
        if !self.remount_enabled {
            MountPhase::Dismounted {
                frames_until_can_remount: None,
            }
        } else if current_mount_phase.mounted() {
            MountPhase::Dismounting {
                frames_until_dismounted: self.dismounted_timer,
            }
        } else if current_mount_phase.remounting() {
            MountPhase::Dismounted {
                frames_until_can_remount: Some(self.remounting_timer),
            }
        } else {
            current_mount_phase
        }
    }
}
