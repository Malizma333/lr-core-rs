pub enum EntityRegistryIndex {
    Stable(usize),
    Unstable(usize),
}

impl Clone for EntityRegistryIndex {
    fn clone(&self) -> Self {
        match self {
            Self::Stable(i) => Self::Stable(*i),
            Self::Unstable(i) => Self::Unstable(*i),
        }
    }
}

impl Copy for EntityRegistryIndex {}
