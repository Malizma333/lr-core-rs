pub(crate) mod builder;
pub(crate) mod entity;
pub(crate) mod snapshot;
pub(crate) mod template;

const REMOUNT_STRENGTH_FACTOR: f64 = 0.1;
const LRA_REMOUNT_STRENGTH_FACTOR: f64 = 0.5;

pub enum MountPhase {
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
