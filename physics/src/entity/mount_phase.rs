#[derive(Debug)]
pub enum MountPhase {
    Mounted,
    Dismounting {
        frames_until_dismounted: u32,
    },
    Dismounted {
        // Some still eligible to remount, None means skeleton no longer intact
        frames_until_remounting: u32,
    },
    Remounting {
        frames_until_mounted: u32,
    },
}

impl MountPhase {
    pub(crate) fn mounted(&self) -> bool {
        match self {
            MountPhase::Mounted => true,
            _ => false,
        }
    }

    pub(crate) fn dismounting(&self) -> bool {
        match self {
            MountPhase::Dismounting {
                frames_until_dismounted: _,
            } => true,
            _ => false,
        }
    }

    pub(crate) fn dismounted(&self) -> bool {
        match self {
            MountPhase::Dismounted {
                frames_until_remounting: _,
            } => true,
            _ => false,
        }
    }

    pub(crate) fn remounting(&self) -> bool {
        match self {
            MountPhase::Remounting {
                frames_until_mounted: _,
            } => true,
            _ => false,
        }
    }

    pub(crate) fn timer(&self) -> u32 {
        match self {
            MountPhase::Mounted => 0,
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => *frames_until_dismounted,
            MountPhase::Dismounted {
                frames_until_remounting,
            } => *frames_until_remounting,
            MountPhase::Remounting {
                frames_until_mounted,
            } => *frames_until_mounted,
        }
    }
}

impl Clone for MountPhase {
    fn clone(&self) -> Self {
        match self {
            MountPhase::Mounted => MountPhase::Mounted,
            MountPhase::Remounting {
                frames_until_mounted: frames_until_remounted,
            } => MountPhase::Remounting {
                frames_until_mounted: *frames_until_remounted,
            },
            MountPhase::Dismounted {
                frames_until_remounting: frames_until_can_remount,
            } => MountPhase::Dismounted {
                frames_until_remounting: *frames_until_can_remount,
            },
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => MountPhase::Dismounting {
                frames_until_dismounted: *frames_until_dismounted,
            },
        }
    }
}

impl Copy for MountPhase {}
