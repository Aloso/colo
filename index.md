---
layout: startpage
title: colo – Simple color management in the terminal
---

## Simple color management in the terminal

Run `colo s <COLOR>` to view a color. Colo supports HTML color names, hex colors and 11 different color spaces.

<pre class="toh-text">
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-arg">ff3377</span>

<span style="color:#ff3377"> ████████</span>  <b>#ff3377</b>                    <span class="toh-dim">rgb(255, 51, 119)        </span>
<span style="color:#ff3377"> ████████</span>  <span class="toh-dim">hsl(-20, 100%, 60%)      </span>  <span class="toh-dim">hsv(340, 80%, 100%)      </span>
<span style="color:#ff3377"> ████████</span>  <span class="toh-dim">cmy(0%, 80%, 53.3%)      </span>  <span class="toh-dim">cmyk(0%, 80%, 53.3%, 0%) </span>
<span style="color:#ff3377"> ████████</span>  <span class="toh-dim">lch(57, 78.1, 9.2)       </span>  <span class="toh-dim">luv(57, 136.2, -0.1)     </span>

<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-arg">orange</span>

<span style="color:#ffa500"> ████████</span>  <b>#ffa500</b>  <span class="toh-dim">orange          </span>  <span class="toh-dim">rgb(255, 165, 0)         </span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">hsl(38.8, 100%, 50%)     </span>  <span class="toh-dim">hsv(38.8, 100%, 100%)    </span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">cmy(0%, 35.3%, 100%)     </span>  <span class="toh-dim">cmyk(0%, 35.3%, 100%, 0%)</span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">lch(74.9, 82.5, 73.1)    </span>  <span class="toh-dim">luv(74.9, 74.9, 74)      </span>

<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-str">"hsv(300, 100%, 100%)"</span>

<span style="color:#ff00ff"> ████████</span>  <span class="toh-dim">#ff00ff</span>  <span class="toh-dim">fuchsia         </span>  <span class="toh-dim">rgb(255, 0, 255)         </span>
<span style="color:#ff00ff"> ████████</span>  <span class="toh-dim">hsl(-60, 100%, 50%)      </span>  <b>hsv(300, 100%, 100%)     </b>
<span style="color:#ff00ff"> ████████</span>  <span class="toh-dim">cmy(0%, 100%, 0%)        </span>  <span class="toh-dim">cmyk(0%, 100%, 0%, 0%)   </span>
<span style="color:#ff00ff"> ████████</span>  <span class="toh-dim">lch(60.3, 115.6, 328.2)  </span>  <span class="toh-dim">luv(60.3, 84.1, -108.7)  </span>

<span class="toh-arrow">&gt;</span> <span class="toh-caret"> </span>
</pre>

## 11 supported color spaces

Colo supports widely used color spaces, that should cover the needs for professional developers and designers. [More information](color_spaces.md).

## Powerful terminal features

Colo supports piping input from/to other commands or files.

<pre class="toh-text">
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">echo</span> <span class="toh-arg">orange</span> <span class="toh-arg">ff4400</span> <span class="toh-pipe">|</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-flag">-o</span> <span class="toh-arg">hsl</span> <span class="toh-pipe">&gt;</span> <span class="toh-pipe">somefile.txt</span>
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">cat</span> <span class="toh-arg">somefile.txt</span>
#ffa500
#ff4400
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-pipe">&lt;</span> <span class="toh-pipe">somefile.txt</span>

<span style="color:#ffa500"> ████████</span>  <b>#ffa500</b>  <span class="toh-dim">orange          </span>  <span class="toh-dim">rgb(255, 165, 0)         </span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">hsl(38.8, 100%, 50%)     </span>  <span class="toh-dim">hsv(38.8, 100%, 100%)    </span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">cmy(0%, 35.3%, 100%)     </span>  <span class="toh-dim">cmyk(0%, 35.3%, 100%, 0%)</span>
<span style="color:#ffa500"> ████████</span>  <span class="toh-dim">lch(74.9, 82.5, 73.1)    </span>  <span class="toh-dim">luv(74.9, 74.9, 74)      </span>

<span style="color:#ff4400"> ████████</span>  <b>#ff4400</b>                    <span class="toh-dim">rgb(255, 68, 0)          </span>
<span style="color:#ff4400"> ████████</span>  <span class="toh-dim">hsl(16, 100%, 50%)       </span>  <span class="toh-dim">hsv(16, 100%, 100%)      </span>
<span style="color:#ff4400"> ████████</span>  <span class="toh-dim">cmy(0%, 73.3%, 100%)     </span>  <span class="toh-dim">cmyk(0%, 73.3%, 100%, 0%)</span>
<span style="color:#ff4400"> ████████</span>  <span class="toh-dim">lch(57.5, 96.9, 45.3)    </span>  <span class="toh-dim">luv(57.5, 151.7, 45.3)   </span>

<span class="toh-arrow">&gt;</span> <span class="toh-caret"> </span>
</pre>

## Flexible input

Hex colors can be entered in the formats `RGB`, `RRGGBB`, `RRRGGGBBB`, etc. Colors in different color spaces are entered in the format `color_space(value1, value2, value3)`, for example `hsl(30, 100%, 60%)`. The parentheses and commas are optional, so the following commands are equivalent:

<pre class="toh-text">
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-str">"cmy(100%, 50%, 66.7%)"</span>
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-arg">cmy</span> <span class="toh-arg">100%</span> <span class="toh-arg">50%</span> <span class="toh-arg">66.7%</span>
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-arg">cmy</span> <span class="toh-arg">1</span> <span class="toh-arg">50%</span> <span class="toh-arg">66.7%</span>
</pre>

## Print colored text

`colo print <TEXT> <COLORS>...` prints text with certain colors applied. There are also flags to make the text bold (`-b`), italic (`-i`) or underlined (`-u`), and a flag to continue printing in the same line (`-n`):

<div class="terminal">
    <img src="./assets/img/text.png" alt="">
</div>

## Get color contrast

Check the contrast between two colors, and find out if white or black text is better readable on a background color.

<div class="terminal">
    <img src="./assets/img/contrast.png" alt="">
</div>

## Show terminal colors

Display the default colors of your terminal.

<div class="terminal">
    <img src="./assets/img/terminal_colors.png" alt="">
</div>

## Experimental features

To get these features, build `colo` from the main branch:

<pre class="toh-text">
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">cargo</span> <span class="toh-hl">install</span> <span class="toh-flag">--git</span> <span class="toh-arg">https://github.com/Aloso/colo</span>
</pre>

### Random values

Random values can be used anywhere where colors can be used. For example:

<pre class="toh-text">
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">contrast</span> <span class="toh-arg">rand</span> <span class="toh-arg">rand</span>
 <span style="color:#fab294">████</span><span style="color:#317fd7">████</span>  <span style="color:#A00">2.30</span>
 <span style="color:#fab294">████</span><span style="color:#317fd7">████</span>  <span class="toh-dim">(relative luminance: 0.543 / 0.207)</span>
<span class="toh-arrow">&gt;</span> <span class="toh-cmd">colo</span> <span class="toh-hl">s</span> <span class="toh-str">"hsl(rand, 100%, 50%)"</span>

<span style="color:#bf00ff"> ████████</span>  <span class="toh-dim">#bf00ff</span>                    <span class="toh-dim">rgb(191.3, 0, 255)       </span>
<span style="color:#bf00ff"> ████████</span>  <b>hsl(285, 100%, 50%)      </b>  <span class="toh-dim">hsv(285, 100%, 100%)     </span>
<span style="color:#bf00ff"> ████████</span>  <span class="toh-dim">cmy(25%, 100%, 0%)       </span>  <span class="toh-dim">cmyk(25%, 100%, 0%, 0%)  </span>
<span style="color:#bf00ff"> ████████</span>  <span class="toh-dim">lch(49.9, 118.8, 318.8)  </span>  <span class="toh-dim">luv(49.9, 42.1, -126.2)  </span>

<span class="toh-arrow">&gt;</span> <span class="toh-caret"> </span>
</pre>
