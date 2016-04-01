cargo-release
=============

A command line tool to make a semi-automatic release of a cargo project.

There's [semantic-rs](https://github.com/semantic-rs/semantic-rs) which
wants to be fully automatic by encoding information into the commit messages
und using it to determine the new version number and to create the Changelog.

I wanted a simpler tool which I can just run locally and which gives the user
more control about the new version number and the Changelog.

You've to be explicit about the new version number by specifing which part of
the version number should be incremented (e.g. `cargo-release minor`) and
you've to write the Changelog by yourself, `cargo-release` will only put the
new version number at the top of the Changelog and open it.

Installation
============

    $ `git clone https://github.com/dan-t/cargo-release.git`
    $ `cd cargo-release`
    $ `cargo build --release`

The build binary will be located at `target/release/cargo-release`.

Usage
=====

Calling `cargo-release <VERSION>` - where `<VERSION>` has to be either `major`, `minor` or `patch` -
inside of the cargo project should start the release process:

* Checks if the git working tree isn't dirty, that there's nothing staged and that
  the local and the remote git repositories are synchronized.

* Runs the tests.

* The current version is read from the `Cargo.toml` and incremented according to
  `<VERSION>` and written back to the `Cargo.toml`.

* Builds a release.

* The changelog (if available) is opened with the new version added at the top of it.
  Every file which lower case base name is equal to `changelog` is considered as a changelog
  file. The default editor for opening the changelog file is `gvim` and can be configured with
  the environment variable `CARGO_RELEASE_EDITOR`.

* A git commit is created containing the changed and not ignored files with the message
  `<PROJ_NAME> <NEW_VERSION>`, where `<PROJ_NAME>` is the cargo project name and `<NEW_VERSION>`
  the version of the release.

* A git tag is created with the name `<PROJ_NAME>-<NEW_VERSION>`.

* The git commit and tag are pushed to the remote repository.

* `cargo publish` is called.
