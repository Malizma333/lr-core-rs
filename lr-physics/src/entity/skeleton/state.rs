use crate::entity::skeleton::MountPhase;

pub struct EntitySkeletonState {
    mount_phase: MountPhase,
    intact: bool,
}

impl Clone for EntitySkeletonState {
    fn clone(&self) -> Self {
        let mount_phase_clone = match self.mount_phase {
            MountPhase::Mounted => MountPhase::Mounted,
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => MountPhase::Dismounting {
                frames_until_dismounted,
            },
            MountPhase::Dismounted {
                frames_until_can_remount,
            } => MountPhase::Dismounted {
                frames_until_can_remount,
            },
            MountPhase::Remounting {
                frames_until_remounted,
            } => MountPhase::Remounting {
                frames_until_remounted,
            },
        };

        EntitySkeletonState {
            mount_phase: mount_phase_clone,
            intact: self.intact,
        }
    }
}

impl EntitySkeletonState {
    pub fn mount_phase(&self) -> MountPhase {
        self.mount_phase
    }
}
