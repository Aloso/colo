---
layout: startpage
title: colo – Simple color management in the terminal
---

## Simple color management in the terminal

Run `colo s <COLOR>` to view a color. Colo supports HTML color names, hex colors and 11 different color spaces.

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="arg">ff3377</span>

<span style='color:#ff3377'> ████████</span>  <b>#ff3377</b>                    <span class='faint'>rgb(255, 51, 119)        </span>
<span style='color:#ff3377'> ████████</span>  <span class='faint'>hsl(-20, 100%, 60%)      </span>  <span class='faint'>hsv(340, 80%, 100%)      </span>
<span style='color:#ff3377'> ████████</span>  <span class='faint'>cmy(0%, 80%, 53.3%)      </span>  <span class='faint'>cmyk(0%, 80%, 53.3%, 0%) </span>
<span style='color:#ff3377'> ████████</span>  <span class='faint'>lch(57, 78.1, 9.2)       </span>  <span class='faint'>luv(57, 136.2, -0.1)     </span>

<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="arg">orange</span>

<span style='color:#ffa500'> ████████</span>  <b>#ffa500</b>  <span class='faint'>orange          </span>  <span class='faint'>rgb(255, 165, 0)         </span>
<span style='color:#ffa500'> ████████</span>  <span class='faint'>hsl(38.8, 100%, 50%)     </span>  <span class='faint'>hsv(38.8, 100%, 100%)    </span>
<span style='color:#ffa500'> ████████</span>  <span class='faint'>cmy(0%, 35.3%, 100%)     </span>  <span class='faint'>cmyk(0%, 35.3%, 100%, 0%)</span>
<span style='color:#ffa500'> ████████</span>  <span class='faint'>lch(74.9, 82.5, 73.1)    </span>  <span class='faint'>luv(74.9, 74.9, 74)      </span>

<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="str">"hsv(300, 100%, 100%)"</span>

<span style='color:#ff00ff'> ████████</span>  <span class='faint'>#ff00ff</span>  <span class='faint'>fuchsia         </span>  <span class='faint'>rgb(255, 0, 255)         </span>
<span style='color:#ff00ff'> ████████</span>  <span class='faint'>hsl(-60, 100%, 50%)      </span>  <b>hsv(300, 100%, 100%)     </b>
<span style='color:#ff00ff'> ████████</span>  <span class='faint'>cmy(0%, 100%, 0%)        </span>  <span class='faint'>cmyk(0%, 100%, 0%, 0%)   </span>
<span style='color:#ff00ff'> ████████</span>  <span class='faint'>lch(60.3, 115.6, 328.2)  </span>  <span class='faint'>luv(60.3, 84.1, -108.7)  </span>

<span class="shell">&gt;</span> <span class="caret"> </span>
</pre>

## 11 supported color spaces

Colo supports widely used color spaces, that should cover the needs for professional developers and designers. [More information](color_spaces.md).

## Powerful terminal features

Colo supports piping input from/to other commands or files.

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">echo</span> <span class="arg">orange</span> <span class="arg">ff4400</span> <span class="pipe">|</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="flag">-o</span> <span class="arg">hsl</span> <span class="pipe">&gt;</span> <span class="pipe">somefile.txt</span>
<span class="shell">&gt;</span> <span class="cmd">cat</span> <span class="arg">somefile.txt</span>
hsl(38.8, 100%, 50%)
hsl(16, 100%, 50%)
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="pipe">&lt;</span> <span class="pipe">somefile.txt</span>

<span style='color:#ffa500'> ████████</span>  <span class='faint'>#ffa500</span>  <span class='faint'>orange          </span>  <span class='faint'>rgb(255, 164.9, 0)       </span>
<span style='color:#ffa500'> ████████</span>  <b>hsl(38.8, 100%, 50%)     </b>  <span class='faint'>hsv(38.8, 100%, 100%)    </span>
<span style='color:#ffa500'> ████████</span>  <span class='faint'>cmy(0%, 35.3%, 100%)     </span>  <span class='faint'>cmyk(0%, 35.3%, 100%, 0%)</span>
<span style='color:#ffa500'> ████████</span>  <span class='faint'>lch(74.9, 82.5, 73.1)    </span>  <span class='faint'>luv(74.9, 74.9, 74)      </span>

<span style='color:#ff4400'> ████████</span>  <span class='faint'>#ff4400</span>                    <span class='faint'>rgb(255, 68, 0)          </span>
<span style='color:#ff4400'> ████████</span>  <b>hsl(16, 100%, 50%)       </b>  <span class='faint'>hsv(16, 100%, 100%)      </span>
<span style='color:#ff4400'> ████████</span>  <span class='faint'>cmy(0%, 73.3%, 100%)     </span>  <span class='faint'>cmyk(0%, 73.3%, 100%, 0%)</span>
<span style='color:#ff4400'> ████████</span>  <span class='faint'>lch(57.5, 96.9, 45.3)    </span>  <span class='faint'>luv(57.5, 151.7, 45.3)   </span>

<span class="shell">&gt;</span> <span class="caret"> </span>
</pre>

## Flexible input

Hex colors can be entered in the formats `RGB`, `RRGGBB`, `RRRGGGBBB`, etc. Colors in different color spaces are entered in the format `color_space(value1, value2, value3)`, for example `hsl(30, 100%, 60%)`. The parentheses and commas are optional, so the following commands are equivalent:

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="str">"cmy(100%, 50%, 66.7%)"</span>
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="arg">cmy</span> <span class="arg">100%</span> <span class="arg">50%</span> <span class="arg">66.7%</span>
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="arg">cmy</span> <span class="arg">1</span> <span class="arg">50%</span> <span class="arg">66.7%</span>
</pre>

## Print colored text

`colo print <TEXT> <COLORS>...` prints text with certain colors applied. There are also flags to make the text bold (`-b`), italic (`-i`) or underlined (`-u`), and a flag to continue printing in the same line (`-n`):

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">print</span> <span class="str">"Hello "</span> <span class="arg">lime</span> <span class="flag">-n</span> <span class="op">&amp;&amp;</span>
  <span class="cmd">colo</span> <span class="hl">print</span> <span class="arg">world!</span> <span class="arg">black</span> <span class="arg">lime</span> <span class="flag">-ib</span>
<span style='color:#00ff00'>Hello </span><b><i><span style='background:#00ff00'><span style='color:#000000'>world!
</span></span></i></b><span class="shell">&gt;</span> <span class="caret"> </span>
</pre>

## Get color contrast

Check the contrast between two colors, and find out if white or black text is better readable on a background color.

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">textcolor</span> <span class="arg">green</span>
<span style='background:#008000'><span style='color:#ffffff'>  white  </span></span>
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">contrast</span> <span class="arg">green</span> <span class="arg">white</span>
 <span style='color:#008000'>████</span><span style='color:#ffffff'>████</span>  <span style='color:#AAA'>5.13</span>
 <span style='color:#008000'>████</span><span style='color:#ffffff'>████</span>  <span class='faint'>(relative luminance: 0.154 / 1.000)</span>
<span class="shell">&gt;</span> <span class="caret"> </span>
</pre>

## Show terminal colors

Display the default colors of your terminal.

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">term</span>
The appearance of these colors depends on your terminal.

   <span style='background:#FFF'><span style='color:#000'>  BrightWhite    </span></span><span style='background:#AAA'><span style='color:#000'>  White    </span></span>  <span style='color:#FFF'>  BrightWhite</span>   <span style='color:#AAA'>  White</span>
   <span style='background:#555'><span style='color:#FFF'>  BrightBlack    </span></span><span style='background:#000'><span style='color:#FFF'>  Black    </span></span>  <span style='color:#555'>  BrightBlack</span>   <span style='color:#000'>  Black</span>
   <span style='background:#F55'><span style='color:#000'>  BrightRed      </span></span><span style='background:#A00'><span style='color:#FFF'>  Red      </span></span>  <span style='color:#F55'>  BrightRed</span>     <span style='color:#A00'>  Red</span>
   <span style='background:#FF5'><span style='color:#000'>  BrightYellow   </span></span><span style='background:#A50'><span style='color:#FFF'>  Yellow   </span></span>  <span style='color:#FF5'>  BrightYellow</span>  <span style='color:#A50'>  Yellow</span>
   <span style='background:#5F5'><span style='color:#000'>  BrightGreen    </span></span><span style='background:#0A0'><span style='color:#FFF'>  Green    </span></span>  <span style='color:#5F5'>  BrightGreen</span>   <span style='color:#0A0'>  Green</span>
   <span style='background:#5FF'><span style='color:#000'>  BrightCyan     </span></span><span style='background:#0AA'><span style='color:#FFF'>  Cyan     </span></span>  <span style='color:#5FF'>  BrightCyan</span>    <span style='color:#0AA'>  Cyan</span>
   <span style='background:#55F'><span style='color:#000'>  BrightBlue     </span></span><span style='background:#00A'><span style='color:#FFF'>  Blue     </span></span>  <span style='color:#55F'>  BrightBlue</span>    <span style='color:#00A'>  Blue</span>
   <span style='background:#F5F'><span style='color:#000'>  BrightMagenta  </span></span><span style='background:#A0A'><span style='color:#FFF'>  Magenta  </span></span>  <span style='color:#F5F'>  BrightMagenta</span> <span style='color:#A0A'>  Magenta</span>

<span class="shell">&gt;</span> <span class="caret"> </span>
</pre>

## Experimental features

To get these features, build `colo` from the main branch:

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">cargo</span> <span class="hl">install</span> <span class="flag">--git</span> <span class="arg">https://github.com/Aloso/colo</span>
</pre>

### Random values

Random values can be used anywhere where colors can be used. For example:

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">contrast</span> <span class="arg">rand</span> <span class="arg">rand</span>
 <span style='color:#79cc23'>████</span><span style='color:#da4306'>████</span>  <span style='color:#A00'>2.18</span>
 <span style='color:#79cc23'>████</span><span style='color:#da4306'>████</span>  <span class='faint'>(relative luminance: 0.474 / 0.189)</span>
<span class="shell">&gt;</span> <span class="cmd">colo</span> <span class="hl">s</span> <span class="str">"hsl(rand, 100%, 50%)"</span>

<span style='color:#00bbff'> ████████</span>  <span class='faint'>#00bbff</span>                    <span class='faint'>rgb(0, 187, 255)         </span>
<span style='color:#00bbff'> ████████</span>  <b>hsl(196, 100%, 50%)      </b>  <span class='faint'>hsv(196, 100%, 100%)     </span>
<span style='color:#00bbff'> ████████</span>  <span class='faint'>cmy(100%, 26.7%, 0%)     </span>  <span class='faint'>cmyk(100%, 26.7%, 0%, 0%)</span>
<span style='color:#00bbff'> ████████</span>  <span class='faint'>lch(71.4, 47, 250.7)     </span>  <span class='faint'>luv(71.4, -47.9, -70.3)  </span>

<span class="shell">&gt;</span> <span class="caret"> </span>
</pre>
