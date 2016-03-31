/// A part of a semver version: Major.Minor.Patch
arg_enum! {
    #[derive(Eq, PartialEq, Debug)]
    pub enum VersionKind {
        Major,
        Minor,
        Patch
    }
}
