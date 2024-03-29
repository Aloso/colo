# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Replace `print` command with more powerful `printf` command

## [0.4.1] - 2020-11-28

This is a patch release with a few color picker improvements.

- [`#46`](https://github.com/Aloso/colo/pull/49): This adds a CIELAB color picker. Open it with `colo pick lab`. It also adds rectangle color pickers. By pressing <kbd>S</kbd>, you can now toggle between 3 sliders and a rectangle + a slider:

    ![sliders](https://aloso.github.io/colo/assets/img/0_4_1_sliders.png)
    ![rectangle](https://aloso.github.io/colo/assets/img/0_4_1_rectangle.png)

    Note that when pressing <kbd>S</kbd>, the currently selected slider will serve as the x-axis of the rectangle; the next slider becomes the y-axis. Select a slider with the <kbd>ArrowUp</kbd> and <kbd>ArrowDown</kbd> keys.

## [0.4] - 2020-11-21

This release brings many new features, including a terminal color picker and commands for mixing colors and generating gradients.

### Added

- [`#43`](https://github.com/Aloso/colo/pull/43): Add color picker. Pick a color with `colo pick`. You can select a color with the arrow keys, or by entering the numbers manually. You can switch between different color pickers with <kbd>Tab</kbd>. Currently supported color pickers are
    - HSL (hue, saturation, light)
    - HSV (hue, saturation, value)
    - RGB (red, green, blue)
    - CMY (cyan, magenta, yellow)

- [`#45`](https://github.com/Aloso/colo/pull/45): Allow `-` and `pick` everywhere. Wherever you're supposed to enter a color, you can now enter `-` to read the color from stdin, or `pick` to select the color in the color picker. For example:

    ```bash
    colo s orange - pick rand
    # first color: orange
    # second color: read from stdin
    # third color: selected in the color picker
    # fourth color: randomly generated
    ```

- [`#45`](https://github.com/Aloso/colo/pull/45): Add `gry` (grayscale) color space. This color space can't represent color hue and saturation, but it's useful to convert a color to grayscale.

- [`#46`](https://github.com/Aloso/colo/pull/46): Add `mix` command. This command mixes two (or more) colors in a given color space (defaults to `lab`). Each color is assigned a _weight_ to control the ratio in which the colors are mixed:

    ```bash
    colo mix orange blue purple    # ratio 1:1:1
    colo mix orange blue -w 1,2,3  # ratio 1:2:3
    ```

- [`#46`](https://github.com/Aloso/colo/pull/46): Add `gradient` command. This command generates a gradient between two colors in a given color space (defaults to `lab`). Usually the gradient is displayed in the terminal; if the command is piped to another program, a list of colors is printed.

## [0.3.3] - 2020-11-15

### Bug fixes

- [`#42`](https://github.com/Aloso/colo/pull/42): Regression in 0.3.2 fixed: `--output` flag didn't work because of a typo

## [0.3.2] - 2020-11-13

Announcing the [new website](https://aloso.github.io/colo/), which explains how to use colo with nice terminal graphics! The graphics are [rendered as HTML](https://github.com/Aloso/to-html), so you can copy+paste the commands.

### Additions

- [`#39`](https://github.com/Aloso/colo/pull/39): Colors can now be randomly generated. Random colors can be used everywhere:

    ```sh
    > colo s rand
    > colo contrast rand rand
    > colo print "Hello world" rand
    ```

    The `rand` keyword creates a random color in the sRGB space. You can also use a different color space and specify some of the values:

    ```sh
    > colo s "hsl(rand, 100%, 50%)"
    > colo s "cmy(100%, rand, rand)"
    ```
- [`#41`](https://github.com/Aloso/colo/pull/41): The `--color` flag can override the behavior when the output uses color:
    * `never`: Don't use color
    * `always`: Always print with color
    * `auto`: Use color if the standard output is a tty, i.e. if `colo` was invoked in the terminal and isn't behind a pipe.

### Changes

- [`#41`](https://github.com/Aloso/colo/pull/41): The output of `colo term` was updated.
- [`#40`](https://github.com/Aloso/colo/pull/40): Help pages were improved significantly. Show help with `colo help` or with `colo help <subcommand>`.

### Bug fixes

- Fixed pipes: A few bugs were fixed where `colo` printed with color or listened to stdin when it shouldn't.

## [0.3.1] - 2020-11-09

- [`#30`](https://github.com/Aloso/colo/pull/30): Align square vertically with text
- [`#31`](https://github.com/Aloso/colo/pull/31): Support piping, for example
    ```fish
    $ echo Hello world | colo print orange
    $ echo blue orange | colo s
    $ echo blue orange | colo textcolor
    ```
- [`#37`](https://github.com/Aloso/colo/pull/37): `textcolor` and `contrast` commands
    * The `textcolor` command shows a color (either black or white) that is readable on a given background color.
    * The `contrast` command shows the contrast ratio between two colors according [the W3 specification](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef).

## [0.3.0] - 2020-11-04

This release completely revamped the command line options, now using subcommands (see [`#21`](https://github.com/Aloso/colo/pull/21)):

- `colo show` (or short `colo s`) shows one or more colors, which are entered consecutively. You'll notice that the output is now more space efficient and informative. I hope you like the new layout!

    The flag to specify the color space was removed, now it can be entered as `'rgb(0, 50, 150)'`. Note that the parentheses and commas are optional for your convenience. For example, you could type:

    ```fish
    colo s rgb 0 50 150, cmy .5 .2 0
    ```

    The `--out` flag accepts two additional formats, `hex` and `html`. If converting the color to an HTML color name fails, it defaults to `hex`. Note that this flag is only really useful when piping to another command, e.g.

    ```fish
    colo s orange red fuchsia -o cmy > somefile.txt
    ```

    The output format is no longer JSON, but the human-readable format, which is also used by HTML and can be parsed by `colo`. If you needed JSON output, please file an issue so we can add this functionality again.

    When displaying certain color values, percentage values are now used, for example for saturation and light.
- `colo print` displays some text in a certain color. A second color can be specified for the background. There are also the following flags to alter the text style:
    - `-b` for bold text
    - `-i` for italic text
    - `-u` for underlined text
    - `-n` to _not_ print a new line afterwards – this is useful to change the style in the middle of a line:

    ```fish
    colo print -n "Hello " orange && colo print "world!" red
    ```
- `colo term` replaces `colo --term`. It is used to print the default terminal colors.
- `colo libs` replaces `colo --libs`. It prints the dependency tree `colo` was compiled with.
- `colo help` prints help information. Use `colo help <subcommand>` to show help for a subcommand.
- The version flag `-v` was renamed to `-V` again.
- [`#22`](https://github.com/Aloso/colo/pull/22): HTML colors containing the word "gray" can now also be spelled "grey".
- [`#23`](https://github.com/Aloso/colo/pull/23): Colo now suggests the correct HTML color name if you misspell it.
- [`#24`](https://github.com/Aloso/colo/pull/24), [`#29`](https://github.com/Aloso/colo/pull/29): Most color spaces now have a nice description [here](https://aloso.github.io/colo/color_spaces).
- [`#27`](https://github.com/Aloso/colo/pull/27): Added the `colo list` subcommand, which shows all HTML colors.
- [`#29`](https://github.com/Aloso/colo/pull/29): Colo now supports percent values, e.g. `cmy 0 100% 25%`.

#### Bug fixes

- [`#17`](https://github.com/Aloso/colo/pull/17): Converting a color to CMYK used to output `cmyk(0, 0, 0, 0)` for any input
- [`#28`](https://github.com/Aloso/colo/pull/28): Displaying hexadecimal colors was broken for colors where the R, G or B value was bigger than 255.

#### Internal

- The `crossterm` dependency was replaced with the more lightweight `colored`.
- Added `thiserror` for better enum error handling.

## [0.2.2] - 2020-10-31

#### Additions

- [`#10`](https://github.com/Aloso/colo/pull/10): Flag `--libraries` to print dependency tree

#### Internal

- Workflow added to publish binaries for Linux, Windows and macOS every time a new version is released
- Improved documentation and tests

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
