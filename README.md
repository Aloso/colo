# colo

Command-line tool for displaying colors, written in Rust

![Crates.io](https://img.shields.io/crates/l/colo) ![Crates.io](https://img.shields.io/crates/v/colo)

### Jump to...

* [Status](#status)
* [Installation](#installation)
* [Usage](#usage)
    * [Supported color spaces](#supported-color-spaces)
    * [Specifying hexadecimal colors](#specifying-hexadecimal-colors)
    * [Color square size](#color-square-size)
    * [Default terminal colors](#default-terminal-colors)
* [Code of Conduct](#code-of-conduct)
* [Contributing](#contributing)


## Status

Experimental – expect to see some big changes in the next releases. See the [changelog](./CHANGELOG.md).

## Installation

Installation is explained on the [release page](https://github.com/Aloso/colo/releases).

If you want to build `colo` from source, make sure you have the Rust toolchain (including Cargo) installed. Then clone this repository and run

```fish
cargo install --path .
```

## Usage

`colo` recognizes hexadecimal RGB colors as well as HTML color names:

![usage](docs/html_colors.png)

Color spaces other than RGB are supported as well:

![usage with other color spaces](docs/color_spaces.png)

If `colo` is used outside of a terminal, it outputs JSON, because it is the most ubiquitous data exchange format:

![json usage](docs/json_output.png)

### Supported color spaces

| Name    | Flag  | Description                        | Range of values |
|---------|-------|------------------------------------|-----------------|
| `rgb`   | `R`   | red, green, blue                   | 0 to 255        |
| `cmy`   | `C`   | cyan, magenta, yellow              | 0 to 1          |
| `cmyk`  | `K`   | cyan, magenta, yellow, key         | 0 to 1          |
| `hsv`   | `V`   | hue, saturation, value             | hue: 0 to 360, saturation: 0 to 1, value: 0 to 1     |
| `hsl`   | `L`   | hue, saturation, light             | hue: 0 to 360, saturation: 0 to 1, light: 0 to 1     |
| `lch`   |       | luminance, chroma, hue             | luminance: 0 to 100, chroma: 0 to 100, hue: 0 to 360 |
| `luv`   |       | CIELUV color (luminance, u, v)     | luminance: 0 to 100, u: -134 to 220, v: -140 to 122  |
| `lab`   |       | CIELAB color (lightness, a, b)     | lightness: 0 to 100, a: ??, b: ??                    |
| `hunterlab` |   | Hunter Lab color (lightness, a, b) | lightness: 0 to 100, a: ??, b: ??                    |
| `xyz`   |       | CIE 1931 XYZ color                 | ??              |
| `yxy`   |       | CIE YXY color                      | ??              |

The input color space can be specified with `--in` or `-i`. The output color space can be specified with `--out` or `-o`. If no input color space is entered, `colo` expects a hexadecimal or HTML color.

There are also flags for the most common input color spaces, e.g. `colo -i rgb 15/0/255` can be abbreviated as `colo -R 15/0/255`.

### Specifying hexadecimal colors

Hexadecimal colors are a different notation for RGB colors. They can optionally prefixed with a `#`, e.g. `colo "#F00"`.

Hexadecimal colors can be specified with varying precision: Each color channel can be between 1 and 8 digits long, for example

![hex numbers](docs/hex_colors.png)

### Color square size

The color square size can be adjusted with `--size` or `-s`:

![color square size](docs/square_sizes.png)

### Default terminal colors

With `--terminal` or `-t`, the default terminal colors are printed:

![terminal usage](docs/terminal_colors.png)

## Code of Conduct

Since this program is written in Rust, the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies. Please be friendly and treat everyone with respect.

## Contributing

I appreciate your help! The easiest way to help is to file bug reports or suggest new features in the [issue tracker](https://github.com/Aloso/colo/issues).

If you want to create a pull request, make sure the following requirements are met:

  * The code compiles on the latest stable Rust version
  * The code is properly formatted with `cargo fmt`
  * The code is documented
  * If you add a dependency that includes unsafe code, please explain why it is required
  * Please try to keep compile times small, if feasible

That's it! If you have any questions, feel free to create an issue.
