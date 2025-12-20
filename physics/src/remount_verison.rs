pub enum RemountVersion {
    None,
    ComV1,
    ComV2,
    LRA,
}

impl Clone for RemountVersion {
    fn clone(&self) -> Self {
        match self {
            RemountVersion::None => RemountVersion::None,
            RemountVersion::ComV1 => RemountVersion::ComV1,
            RemountVersion::ComV2 => RemountVersion::ComV2,
            RemountVersion::LRA => RemountVersion::LRA,
        }
    }
}

impl Copy for RemountVersion {}

impl RemountVersion {
    pub(crate) fn none(&self) -> bool {
        match self {
            RemountVersion::None => true,
            _ => false,
        }
    }

    pub(crate) fn comv1(&self) -> bool {
        match self {
            RemountVersion::ComV1 => true,
            _ => false,
        }
    }

    pub(crate) fn comv2(&self) -> bool {
        match self {
            RemountVersion::ComV2 => true,
            _ => false,
        }
    }

    pub(crate) fn lra(&self) -> bool {
        match self {
            RemountVersion::LRA => true,
            _ => false,
        }
    }
}
