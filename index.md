---
layout: startpage
title: colo – Simple color management in the terminal
---

## Simple color management in the terminal

Run `colo s <COLOR>` to view a color. Colo supports HTML color names, hex colors and 11 different color spaces.

<div class="terminal">
    <img src="/assets/img/main_example.png" alt="">
</div>

## 11 supported color spaces

Colo supports widely used color spaces, that should cover the needs for professional developers and designers. [More information](color_spaces.md).

## Powerful terminal features

Colo supports piping input from/to other commands or files.

<div class="terminal">
    <img src="/assets/img/pipes.png" alt="">
</div>

## Flexible input

Hex colors can be entered in the formats `RGB`, `RRGGBB`, `RRRGGGBBB`, etc. Colors in different color spaces are entered in the format `color_space(value1, value2, value3)`, for example `hsl(30, 100%, 60%)`. The parentheses and commas are optional, so the following commands are equivalent:

```bash
$ colo s 'cmy(100%, 50%, 66.7%)'
$ colo s cmy 100% 50% 66.7%
$ colo s cmy 1 50% 66.7%
```

## Print colored text

`colo print <TEXT> <COLORS>...` prints text with certain colors applied. There are also flags to make the text bold (`-b`), italic (`-i`) or underlined (`-u`), and a flag to continue printing in the same line (`-n`):

<div class="terminal">
    <img src="/assets/img/text.png" alt="">
</div>

## Get color contrast

Check the contrast between two colors, and find out if white or black text is better readable on a background color.

<div class="terminal">
    <img src="/assets/img/contrast.png" alt="">
</div>

## Show terminal colors

Display the default colors of your terminal.

<div class="terminal">
    <img src="/assets/img/terminal_colors.png" alt="">
</div>

## Experimental features

To get these features, build `colo` from the main branch:

```sh
$ cargo install --git https://github.com/Aloso/colo
```

### Random values

Random values can be used anywhere where colors can be used. For example:

<div class="terminal">
    <img src="/assets/img/rand.png" alt="">
</div>
