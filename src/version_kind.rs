use semver::Version;

// A part of a semver version: Major.Minor.Patch
arg_enum! {
    #[derive(Eq, PartialEq, Debug)]
    pub enum VersionKind {
        // increments the major part of the version
        Major,

        // increments the minor part of the version
        Minor,

        // increments the patch part of the version
        Patch,

        // keeps the current version without incrementing anything
        Current
    }
}

impl VersionKind {
    pub fn increment(&self, version: &Version) -> Version {
        let mut new_vers = version.clone();
        match *self {
            VersionKind::Major   => new_vers.increment_major(),
            VersionKind::Minor   => new_vers.increment_minor(),
            VersionKind::Patch   => new_vers.increment_patch(),
            VersionKind::Current => {}
        }

        new_vers
    }
}
