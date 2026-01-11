#[derive(Debug, Clone, Copy)]
pub enum MountPhase {
    Mounted,
    Dismounting { frames_until_dismounted: u32 },
    Dismounted { frames_until_remounting: u32 },
    Remounting { frames_until_mounted: u32 },
}

// TODO remove these in favor of if let ..
impl MountPhase {
    pub fn mounted(&self) -> bool {
        match self {
            MountPhase::Mounted => true,
            _ => false,
        }
    }

    pub fn dismounting(&self) -> bool {
        match self {
            MountPhase::Dismounting {
                frames_until_dismounted: _,
            } => true,
            _ => false,
        }
    }

    pub fn dismounted(&self) -> bool {
        match self {
            MountPhase::Dismounted {
                frames_until_remounting: _,
            } => true,
            _ => false,
        }
    }

    pub fn remounting(&self) -> bool {
        match self {
            MountPhase::Remounting {
                frames_until_mounted: _,
            } => true,
            _ => false,
        }
    }
}
