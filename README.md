# colo <img src="https://aloso.github.io/colo/assets/logo.svg" alt="Logo">

Command-line tool for displaying colors, written in Rust

[![Crates.io](https://img.shields.io/crates/l/colo)](./LICENSE) [![Crates.io](https://img.shields.io/crates/v/colo)](https://crates.io/crates/colo) [![Tests](https://github.com/Aloso/colo/workflows/Test/badge.svg)](https://github.com/Aloso/colo/actions?query=workflow%3ATest)


## Changelog

[The changelog can be found here](./CHANGELOG.md).

Note that `colo` is very young and evolving rapidly. There will likely be big changes in the next releases.

## Usage

How to use `colo` is explained [on the website](https://aloso.github.io/colo).

[Here](https://aloso.github.io/colo/color_spaces) is the list of supported color spaces.

## Installation

How to install `colo` is explained on the [releases page](https://github.com/Aloso/colo/releases).

### Build from source

If you want to build `colo` from source, make sure you have the Rust toolchain (including Cargo) installed. Then clone this repository and run

```fish
cargo install --path .
```

Or, if you don't want to clone the repository, you can run

```fish
cargo install --git https://github.com/Aloso/colo
```

This builds the code from the main branch. You can specify a different branch with `--branch` or a tag with `--tag`.

## Code of Conduct

Since this program is written in Rust, the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies. Please be friendly and treat everyone with respect.

## Contributing

I appreciate your help! The easiest way to help is to file bug reports or suggest new features in the [issue tracker](https://github.com/Aloso/colo/issues).

If you want to create a pull request, make sure the following requirements are met:

  * The code is documented
  * If you add a dependency that includes unsafe code, please explain why it is required
  * Please try to keep compile times small, if feasible

Also, to pass continuous integration, the code must

  * be properly formatted with `cargo fmt`
  * pass `cargo clippy`
  * compile on the latest stable Rust version
  * all tests must succeed

You can also look in the issue tracker for issues with the label [help wanted](https://github.com/Aloso/colo/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).

To contribute to the website, send a PR to the [`gh-pages`](https://github.com/Aloso/colo/tree/gh-pages) branch.

That's it! If you have any questions, feel free to create an issue.
