[![Build Status](https://travis-ci.org/dan-t/rusty-release.svg?branch=master)](https://travis-ci.org/dan-t/rusty-release)

rusty-release
=============

A command line tool to make a semi-automatic release of a cargo project.

There's [semantic-rs](https://github.com/semantic-rs/semantic-rs) which
wants to be fully automatic by encoding information into the commit messages
und using it to determine the new version number and to create the Changelog.

I wanted a simpler tool which gives the user more control about the new version
number and the Changelog.

You've to be explicit about the new version number by specifing which part of
the version number should be incremented (e.g. `rusty-release minor`) and
you've to write the Changelog by yourself, `rusty-release` will only put the
new version number at the top of the Changelog and open it.

Installation
============

    $ cargo install rusty-release

The build binary will be located at `.cargo/bin/rusty-release`.

Usage
=====

Calling `rusty-release <VERSION>` - where `<VERSION>` has to be either `major`, `minor` or `patch` -
inside of the cargo project should start the release process:

* Checks if the git working tree isn't dirty, that there's nothing staged and that
  the local and the remote git repositories are synchronized.

* Runs the tests.

* The current version is read from the `Cargo.toml` and incremented according to
  `<VERSION>` and written back to the `Cargo.toml`.

* Builds a release.

* If available, the changelog - with the new version added at the top - and a temporary
  file containing all commits from HEAD to the previous release are opened in the configured editor.

  Every file which lower case base name is equal to `changelog` is considered as a changelog file.

* A git commit is created containing the changed and not ignored files with the configured commit message.

* A git tag is created with the configured name.

* The git commit and tag are pushed to the remote repository.

* `cargo publish` is called.

Configuration
=============

If avialable, the configuration file `.rusty-release.toml` is read from the home directory
and from the cargo project root directory (where the `Cargo.toml` resides).

The current supported configuration (default configuration displayed) is:

    # publish to crates.io
    cargo_publish = true

    # push to git remote repository
    git_push = true

    # string template for the creation of the commit message, currently the two
    # placeholders '<PROJ_NAME>' - the name of the cargo project - and
    # '<NEW_VERSION>' - the version of the release - are supported
    commit_message = "<PROJ_NAME> <NEW_VERSION>"

    # a string template like 'commit_message' supporting the same placeholders
    tag_name = "<PROJ_NAME>-<NEW_VERSION>"

    # the editor command for opening the changelog, for the best experience the
    # editor command should be able to open multiple files in a split view
    editor = "gvim -o"
