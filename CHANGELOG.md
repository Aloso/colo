# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

#### Additional

- Flag `--libraries` to print dependency tree

#### Internal

Improved documentation and tests

## [0.2.1] - 2020-10-29

#### Bug fixes:

- [`1bab356`](https://github.com/Aloso/colo/commit/1bab3560157fe24a9a093805a69afbe091482ec1): Fix false version (the version in the CLI is now automatically kept up to date)

## [0.2] - 2020-10-29

This release revamps the CLI arguments, adds the `--size` argument and fixes a few bugs.

#### Additions:

- [`#3`](https://github.com/Aloso/colo/pull/3): Add `--size`/`-s` argument to specify the size of the color square

#### Breaking changes:

- [`#3`](https://github.com/Aloso/colo/pull/3): Input color spaces are now entered with the `--in` or `-i` argument.

    There are now 5 flags (`-R, -C, -K, -V, -L`) as abbreviations for `--in rgb, --in cmy, --in cmyk, --in hsv, --in hsl`.
- [`#3`](https://github.com/Aloso/colo/pull/3): The shortcut for `--version` is now `-v` (lowercase) instead of `-V`

#### Bug fixes:

- [`#1`](https://github.com/Aloso/colo/pull/1): Fix `colo -t` not working
- [`#2`](https://github.com/Aloso/colo/pull/2): Fix OS-specific lifetime error

#### Internal

This release adds a GitHub workflow to test pull requests on Windows, Linux and macOS. Of course, this is no substitute for real-world testing, so please report any issues you encounter!

This release also adds some documentation, but the code quality is still not too good. If you want to help by writing tests or documenting code, just file a PR ;)

## [0.1] - 2020-10-28

Initial release
