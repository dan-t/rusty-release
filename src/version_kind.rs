use semver::Version;

/// A part of a semver version: Major.Minor.Patch
arg_enum! {
    #[derive(Eq, PartialEq, Debug)]
    pub enum VersionKind {
        Major,
        Minor,
        Patch
    }
}

impl VersionKind {
    pub fn increment(&self, mut version: Version) -> Version {
        match *self {
            VersionKind::Major => version.increment_major(),
            VersionKind::Minor => version.increment_minor(),
            VersionKind::Patch => version.increment_patch()
        }

        version
    }
}
