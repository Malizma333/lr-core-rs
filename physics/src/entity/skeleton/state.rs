use crate::MountPhase;

#[derive(Debug)]
pub struct EntitySkeletonState {
    mount_phase: MountPhase,
    sled_intact: bool,
}

impl Clone for EntitySkeletonState {
    fn clone(&self) -> Self {
        EntitySkeletonState {
            mount_phase: self.mount_phase,
            sled_intact: self.sled_intact,
        }
    }
}

impl EntitySkeletonState {
    pub(crate) fn new(mount_phase: MountPhase, sled_intact: bool) -> Self {
        EntitySkeletonState {
            mount_phase,
            sled_intact,
        }
    }
}

impl EntitySkeletonState {
    pub(crate) fn mount_phase(&self) -> MountPhase {
        self.mount_phase
    }

    pub(crate) fn sled_intact(&self) -> bool {
        self.sled_intact
    }

    pub(crate) fn set_mount_phase(&mut self, mount_phase: MountPhase) {
        self.mount_phase = mount_phase;
    }

    pub(crate) fn set_sled_intact(&mut self, sled_intact: bool) {
        self.sled_intact = sled_intact;
    }

    pub(crate) fn timer(&self) -> u32 {
        match self.mount_phase {
            MountPhase::Dismounted {
                frames_until_remounting,
            } => frames_until_remounting,
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => frames_until_dismounted,
            MountPhase::Mounted => 0,
            MountPhase::Remounting {
                frames_until_mounted,
            } => frames_until_mounted,
        }
    }
}
