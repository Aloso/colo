---
layout: startpage
---

## Simple color management in the terminal

Run `colo s <COLOR>` to view a color. Colo supports HTML color names, hex colors and 11 different color spaces.

![example](/assets/img/main_example.png)

## 11 supported color spaces

Colo supports widely used color spaces, that should cover the needs for professional developers and designers. [More information](color_spaces.md).

## Powerful terminal features

Colo supports piping input from/to other commands or files.

![pipes](/assets/img/pipes.png)

## Flexible input

Hex colors can be entered in the formats `RGB`, `RRGGBB`, `RRRGGGBBB`, etc. Colors in different color spaces are entered in the format `color_space(value1, value2, value3)`, for example `hsl(30, 100%, 60%)`. The parentheses and commas are optional, so the following commands are equivalent:

```bash
$ colo s 'cmy(100%, 50%, 66.7%)'
$ colo s cmy 100% 50% 66.7%
$ colo s cmy 1 50% 66.7%
```

## Print colored text

`colo print <TEXT> <COLORS>...` prints text with certain colors applied. There are also flags to make the text bold (`-b`), italic (`-i`) or underlined (`-u`), and a flag to continue printing in the same line (`-n`):

![text](/assets/img/text.png)
