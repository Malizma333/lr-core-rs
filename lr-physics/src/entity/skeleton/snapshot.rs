use vector2d::Vector2Df;

use crate::entity::skeleton::MountPhase;

pub(crate) struct EntitySkeletonSnapshot {
    pub(super) points: Vec<Vector2Df>,
    pub(super) bones: Vec<usize>,
    pub(super) joints: Vec<usize>,
    pub(super) remount_enabled: bool,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) remounted_timer: u32,
    pub(super) mount_phase: MountPhase,
    pub(super) intact: bool,
}

impl EntitySkeletonSnapshot {
    pub(crate) fn is_remounting(&self) -> bool {
        matches!(
            self.mount_phase,
            MountPhase::Remounting {
                frames_until_remounted: _
            }
        )
    }

    pub(crate) fn is_mounted(&self) -> bool {
        matches!(self.mount_phase, MountPhase::Mounted)
    }
}
