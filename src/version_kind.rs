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
    pub fn increment(&self, version: &Version) -> Version {
        let mut new_vers = version.clone();
        match *self {
            VersionKind::Major => new_vers.increment_major(),
            VersionKind::Minor => new_vers.increment_minor(),
            VersionKind::Patch => new_vers.increment_patch()
        }

        new_vers
    }
}
