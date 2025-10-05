use crate::entity::entity_registry::{
    EntityBoneId, EntityBoneTemplateId, EntityJointId, EntityJointTemplateId, EntityPointId,
    EntityPointTemplateId,
};

const REMOUNT_STRENGTH_FACTOR: f64 = 0.1;
const LRA_REMOUNT_STRENGTH_FACTOR: f64 = 0.5;

pub trait SkeletonTemplate {
    fn frames_until_dismounted() -> u32 {
        0
    }

    fn frames_until_remounting() -> u32 {
        0
    }

    fn frames_until_remounted() -> u32 {
        0
    }
}

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

pub struct EntitySkeleton {
    points: Vec<EntityPointId>,
    bones: Vec<EntityBoneId>,
    joints: Vec<EntityJointId>,
    remount_enabled: bool,
    dismounted_timer: u32,
    remounting_timer: u32,
    remounted_timer: u32,
    state: EntitySkeletonState,
}

pub struct EntitySkeletonTemplate {
    points: Vec<EntityPointTemplateId>,
    bones: Vec<EntityBoneTemplateId>,
    joints: Vec<EntityJointTemplateId>,
    remount_enabled: bool,
    dismounted_timer: Option<u32>,
    remounting_timer: Option<u32>,
    remounted_timer: Option<u32>,
}

impl EntitySkeletonTemplate {
    pub fn new() -> EntitySkeletonTemplate {
        EntitySkeletonTemplate {
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            remount_enabled: false,
            dismounted_timer: None,
            remounting_timer: None,
            remounted_timer: None,
        }
    }

    pub fn add_point(&mut self, id: EntityPointTemplateId) {
        self.points.push(id);
    }

    pub fn add_bone(&mut self, id: EntityBoneTemplateId) {
        self.bones.push(id);
    }

    pub fn add_joint(&mut self, id: EntityJointTemplateId) {
        self.joints.push(id);
    }

    pub fn enable_remount(&mut self) {
        self.remount_enabled = true;
    }

    pub fn time_until_dismounted(&mut self, limit: u32) {
        self.dismounted_timer = Some(limit);
    }

    pub fn time_until_remounting(&mut self, limit: u32) {
        self.remounting_timer = Some(limit);
    }

    pub fn time_until_remounted(&mut self, limit: u32) {
        self.remounted_timer = Some(limit);
    }

    pub fn build(&self) -> EntitySkeleton {
        EntitySkeleton {
            points: self.points,
            bones: self.bones,
            joints: self.joints,
            remount_enabled: self.remount_enabled,
            dismounted_timer: self.dismounted_timer.unwrap_or(0),
            remounting_timer: self.remounting_timer.unwrap_or(0),
            remounted_timer: self.remounted_timer.unwrap_or(0),
            state: EntitySkeletonState {
                mount_phase: MountPhase::Mounted,
                intact: true,
            },
        }
    }
}

impl EntitySkeleton {
    pub fn is_remounting(&self) -> bool {
        matches!(
            self.state.mount_phase,
            MountPhase::Remounting {
                frames_until_remounted: _
            }
        )
    }

    pub fn is_mounted(&self) -> bool {
        matches!(self.state.mount_phase, MountPhase::Mounted) || self.is_remounting()
    }

    pub fn dismount(&mut self) {
        // Currently does the same thing as destroy
        self.state.mount_phase = MountPhase::Dismounted {
            frames_until_can_remount: None,
        }
    }

    pub fn destroy(&mut self) {
        self.state.mount_phase = MountPhase::Dismounted {
            frames_until_can_remount: None,
        }
    }

    pub fn points(&self) -> &Vec<EntityPointId> {
        &self.points
    }

    pub fn bones(&self) -> &Vec<EntityBoneId> {
        &self.bones
    }

    pub fn joints(&self) -> &Vec<EntityJointId> {
        &self.joints
    }
}
