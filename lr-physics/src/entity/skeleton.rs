use crate::engine::EntityRegistryIndex;

enum MountPhase {
    Mounted,
    Dismounting {
        frames_until_dismounted: u32,
    },
    Dismounted {
        // Some still eligible to remount, None means skeleton no longer intact
        frames_until_can_remount: Option<u32>,
    },
    Remounting {
        frames_until_remounted: u32,
    },
}

struct MountState {
    mount_bones: Vec<EntityRegistryIndex>,
    mount_joints: Vec<EntityRegistryIndex>,
    mount_phase: MountPhase,
    other: Option<EntityRegistryIndex>,
}

impl Clone for MountState {
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

        MountState {
            // TODO: these reference invalid bones + joints
            mount_bones: self.mount_bones.clone(),
            mount_joints: self.mount_joints.clone(),
            mount_phase: mount_phase_clone,
            other: self.other,
        }
    }
}

pub struct EntitySkeleton {
    points: Vec<EntityRegistryIndex>,
    bones: Vec<EntityRegistryIndex>,
    joints: Vec<EntityRegistryIndex>,
    remount_enabled: bool,
    dismounted_timer: u32,
    remounting_timer: u32,
    remounted_timer: u32,
    mount_state: MountState,
}
