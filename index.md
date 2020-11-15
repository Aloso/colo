---
layout: startpage
title: colo – Simple color management in the terminal
---

## Simple color management in the terminal

Run `colo s <COLOR>` to view a color. Colo supports HTML color names, hex colors and 11 different color spaces.

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class="h-arg">ff3377</span>

<span style='color:#ff3377'> ████████</span>  <b>#ff3377</b>                    <span style='opacity:0.67'>rgb(255, 51, 119)        </span>
<span style='color:#ff3377'> ████████</span>  <span style='opacity:0.67'>hsl(-20, 100%, 60%)      </span>  <span style='opacity:0.67'>hsv(340, 80%, 100%)      </span>
<span style='color:#ff3377'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 80%, 53.3%)      </span>  <span style='opacity:0.67'>cmyk(0%, 80%, 53.3%, 0%) </span>
<span style='color:#ff3377'> ████████</span>  <span style='opacity:0.67'>lch(57, 78.1, 9.2)       </span>  <span style='opacity:0.67'>luv(57, 136.2, -0.1)     </span>

<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class="h-arg">orange</span>

<span style='color:#ffa500'> ████████</span>  <b>#ffa500</b>  <span style='opacity:0.67'>orange          </span>  <span style='opacity:0.67'>rgb(255, 165, 0)         </span>
<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>hsl(38.8, 100%, 50%)     </span>  <span style='opacity:0.67'>hsv(38.8, 100%, 100%)    </span>
<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 35.3%, 100%)     </span>  <span style='opacity:0.67'>cmyk(0%, 35.3%, 100%, 0%)</span>
<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>lch(74.9, 82.5, 73.1)    </span>  <span style='opacity:0.67'>luv(74.9, 74.9, 74)      </span>

<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class='h-str'>&#39;hsv(300, 100%, 100%)&#39;</span>

<span style='color:#ff00ff'> ████████</span>  <span style='opacity:0.67'>#ff00ff</span>  <span style='opacity:0.67'>fuchsia         </span>  <span style='opacity:0.67'>rgb(255, 0, 255)         </span>
<span style='color:#ff00ff'> ████████</span>  <span style='opacity:0.67'>hsl(-60, 100%, 50%)      </span>  <b>hsv(300, 100%, 100%)     </b>
<span style='color:#ff00ff'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 100%, 0%)        </span>  <span style='opacity:0.67'>cmyk(0%, 100%, 0%, 0%)   </span>
<span style='color:#ff00ff'> ████████</span>  <span style='opacity:0.67'>lch(60.3, 115.6, 328.2)  </span>  <span style='opacity:0.67'>luv(60.3, 84.1, -108.7)  </span>

<span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

## 11 supported color spaces

Colo supports widely used color spaces, that should cover the needs for professional developers and designers. [More information](color_spaces.md).

## Powerful terminal features

Colo supports piping input from/to other commands or files.

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">echo</span> <span class="h-arg">orange</span> <span class="h-arg">ff4400</span> <span class='h-pipe'>|</span> <span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class="h-flag">-o</span> <span class="h-arg">hsl</span> <span class='h-pipe'>&gt;</span> <span class='h-pipe'>somefile.txt</span>
<span class="h-shell">&gt; </span><span class="h-cmd">cat</span> <span class="h-arg">somefile.txt</span>
hsl(38.8, 100%, 50%)
hsl(16, 100%, 50%)
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class='h-pipe'>&lt;</span> <span class='h-pipe'>somefile.txt</span>

<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>#ffa500</span>  <span style='opacity:0.67'>orange          </span>  <span style='opacity:0.67'>rgb(255, 164.9, 0)       </span>
<span style='color:#ffa500'> ████████</span>  <b>hsl(38.8, 100%, 50%)     </b>  <span style='opacity:0.67'>hsv(38.8, 100%, 100%)    </span>
<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 35.3%, 100%)     </span>  <span style='opacity:0.67'>cmyk(0%, 35.3%, 100%, 0%)</span>
<span style='color:#ffa500'> ████████</span>  <span style='opacity:0.67'>lch(74.9, 82.5, 73.1)    </span>  <span style='opacity:0.67'>luv(74.9, 74.9, 74)      </span>

<span style='color:#ff4400'> ████████</span>  <span style='opacity:0.67'>#ff4400</span>                    <span style='opacity:0.67'>rgb(255, 68, 0)          </span>
<span style='color:#ff4400'> ████████</span>  <b>hsl(16, 100%, 50%)       </b>  <span style='opacity:0.67'>hsv(16, 100%, 100%)      </span>
<span style='color:#ff4400'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 73.3%, 100%)     </span>  <span style='opacity:0.67'>cmyk(0%, 73.3%, 100%, 0%)</span>
<span style='color:#ff4400'> ████████</span>  <span style='opacity:0.67'>lch(57.5, 96.9, 45.3)    </span>  <span style='opacity:0.67'>luv(57.5, 151.7, 45.3)   </span>

<span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

## Flexible input

Hex colors can be entered in the formats `RGB`, `RRGGBB`, `RRRGGGBBB`, etc. Colors in different color spaces are entered in the format `color_space(value1, value2, value3)`, for example `hsl(30, 100%, 60%)`. The parentheses and commas are optional, so the following commands are equivalent:

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class='h-str'>&#39;cmy(100%, 50%, 66.7%)&#39;</span>
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class="h-arg">cmy</span> <span class="h-arg">100%</span> <span class="h-arg">50%</span> <span class="h-arg">66.7%</span>
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class="h-arg">cmy</span> <span class="h-arg">1</span> <span class="h-arg">50%</span> <span class="h-arg">66.7%</span>
</pre>

## Print colored text

`colo print <TEXT> <COLORS>...` prints text with certain colors applied. There are also flags to make the text bold (`-b`), italic (`-i`) or underlined (`-u`), and a flag to continue printing in the same line (`-n`):

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">print</span> <span class='h-str'>&#39;Hello &#39;</span> <span class="h-arg">lime</span> <span class="h-flag">-n</span> <span class='h-punct'>&amp;&amp;</span>
  <span class="h-cmd">colo</span> <span class="h-hl">print</span> <span class='h-str'>&#39;world!&#39;</span> <span class="h-arg">black</span> <span class="h-arg">lime</span> <span class="h-flag">-ib</span>
<span style='color:#00ff00'>Hello </span><b><i><span style='background:#00ff00'><span style='color:#000000'>world!
</span></span></i></b><span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

## Get color contrast

Check the contrast between two colors, and find out if white or black text is better readable on a background color.

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">textcolor</span> <span class="h-arg">green</span>
<span style='background:#008000'><span style='color:#ffffff'>  white  </span></span>
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">contrast</span> <span class="h-arg">green</span> <span class="h-arg">white</span>
 <span style='color:#008000'>████</span><span style='color:#ffffff'>████</span>  <span style='color:#aaa'>5.13</span>
 <span style='color:#008000'>████</span><span style='color:#ffffff'>████</span>  <span style='opacity:0.67'>(relative luminance: 0.154 / 1.000)</span>
<span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

## Show terminal colors

Display the default colors of your terminal.

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">term</span>
The appearance of these colors depends on your terminal.

   <span style='background:#fff'><span style='color:#000'>  BrightWhite    </span></span><span style='background:#aaa'><span style='color:#000'>  White    </span></span>  <span style='color:#fff'>  BrightWhite</span>   <span style='color:#aaa'>  White</span>
   <span style='background:#555'><span style='color:#fff'>  BrightBlack    </span></span><span style='background:#000'><span style='color:#fff'>  Black    </span></span>  <span style='color:#555'>  BrightBlack</span>   <span style='color:#000'>  Black</span>
   <span style='background:#f55'><span style='color:#000'>  BrightRed      </span></span><span style='background:#a00'><span style='color:#fff'>  Red      </span></span>  <span style='color:#f55'>  BrightRed</span>     <span style='color:#a00'>  Red</span>
   <span style='background:#ff5'><span style='color:#000'>  BrightYellow   </span></span><span style='background:#a60'><span style='color:#fff'>  Yellow   </span></span>  <span style='color:#ff5'>  BrightYellow</span>  <span style='color:#a60'>  Yellow</span>
   <span style='background:#5f5'><span style='color:#000'>  BrightGreen    </span></span><span style='background:#0a0'><span style='color:#fff'>  Green    </span></span>  <span style='color:#5f5'>  BrightGreen</span>   <span style='color:#0a0'>  Green</span>
   <span style='background:#5ff'><span style='color:#000'>  BrightCyan     </span></span><span style='background:#0aa'><span style='color:#fff'>  Cyan     </span></span>  <span style='color:#5ff'>  BrightCyan</span>    <span style='color:#0aa'>  Cyan</span>
   <span style='background:#55f'><span style='color:#000'>  BrightBlue     </span></span><span style='background:#00a'><span style='color:#fff'>  Blue     </span></span>  <span style='color:#55f'>  BrightBlue</span>    <span style='color:#00a'>  Blue</span>
   <span style='background:#f5f'><span style='color:#000'>  BrightMagenta  </span></span><span style='background:#a0a'><span style='color:#fff'>  Magenta  </span></span>  <span style='color:#f5f'>  BrightMagenta</span> <span style='color:#a0a'>  Magenta</span>

<span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

## Random values

Random values can be used anywhere where colors can be used. For example:

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">contrast</span> <span class="h-arg">rand</span> <span class="h-arg">rand</span>
 <span style='color:#14389f'>████</span><span style='color:#7388c9'>████</span>  <span style='color:#a00'>2.90</span>
 <span style='color:#14389f'>████</span><span style='color:#7388c9'>████</span>  <span style='opacity:0.67'>(relative luminance: 0.055 / 0.255)</span>
<span class="h-shell">&gt; </span><span class="h-cmd">colo</span> <span class="h-hl">s</span> <span class='h-str'>&#39;hsl(rand, 100%, 50%)&#39;</span>

<span style='color:#ff0090'> ████████</span>  <span style='opacity:0.67'>#ff0090</span>                    <span style='opacity:0.67'>rgb(255, 0, 144.5)       </span>
<span style='color:#ff0090'> ████████</span>  <b>hsl(326, 100%, 50%)      </b>  <span style='opacity:0.67'>hsv(326, 100%, 100%)     </span>
<span style='color:#ff0090'> ████████</span>  <span style='opacity:0.67'>cmy(0%, 100%, 43.3%)     </span>  <span style='opacity:0.67'>cmyk(0%, 100%, 43.3%, 0%)</span>
<span style='color:#ff0090'> ████████</span>  <span style='opacity:0.67'>lch(55.4, 86, 356.6)     </span>  <span style='opacity:0.67'>luv(55.4, 134.5, -23.8)  </span>

<span class="h-shell">&gt; </span><span class="h-caret"> </span>
</pre>

<!--

## Experimental features

To get these features, build `colo` from the main branch:

<pre class="terminal">
<span class="shell">&gt;</span> <span class="cmd">cargo</span> <span class="hl">install</span> <span class="flag">--git</span> <span class="arg">https://github.com/Aloso/colo</span>
</pre>

-->
