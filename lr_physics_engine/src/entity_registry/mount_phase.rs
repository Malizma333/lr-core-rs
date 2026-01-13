#[derive(Debug, Clone, Copy)]
pub enum MountPhase {
    Mounted,
    Dismounting { frames_until_dismounted: u32 },
    Dismounted { frames_until_remounting: u32 },
    Remounting { frames_until_mounted: u32 },
}

impl MountPhase {
    pub fn is_mounted(&self) -> bool {
        matches!(self, MountPhase::Mounted)
    }

    pub fn is_dismounting(&self) -> bool {
        matches!(self, MountPhase::Dismounting { .. })
    }

    pub fn is_dismounted(&self) -> bool {
        matches!(self, MountPhase::Dismounted { .. })
    }

    pub fn is_remounting(&self) -> bool {
        matches!(self, MountPhase::Remounting { .. })
    }
}
