#[derive(Debug, Clone, Copy)]
pub enum RemountVersion {
    None,
    ComV1,
    ComV2,
    LRA,
}

impl RemountVersion {
    pub fn is_none(&self) -> bool {
        matches!(self, RemountVersion::None)
    }

    pub fn is_comv1(&self) -> bool {
        matches!(self, RemountVersion::ComV1)
    }

    pub fn is_comv2(&self) -> bool {
        matches!(self, RemountVersion::ComV2)
    }

    pub fn is_lra(&self) -> bool {
        matches!(self, RemountVersion::LRA)
    }
}
