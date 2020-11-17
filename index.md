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
</span></span></i></b>
<span class="h-shell">&gt; </span><span class="h-caret"> </span>
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

## Experimental features

To get these features, build `colo` from the main branch:

<pre class="h-terminal">
<span class="h-shell">&gt; </span><span class="h-cmd">cargo</span> <span class="h-hl">install</span> <span class="h-flag">--git</span> <span class="h-arg">https://github.com/Aloso/colo</span>
</pre>

### Terminal color picker

Pick a color by entering `colo pick`, and then setting the desired values with the arrow keys, or by entering the values directly. With <kbd>Tab</kbd>, you can switch between the sRGB, HSL, HSV and CMY color space. When you're done, press <kbd>Enter</kbd> or <kbd>Esc</kbd>.

<pre class="h-terminal">
<span class="h-shell">&gt;</span> <span class="h-cmd">colo</span> <span class="h-hl">pick</span>

 <span style="background-color:#FF0000">        </span>  HSL
 <span style="background-color:#FF0000">        </span>  <b>H</b>  <span style="background-color:#76777A"><font color="#1C2124">0     </font></span> <font color="#FF0000">▕</font><span style="background-color:#FF1000"><font color="#171421">▌</font></span><span style="background-color:#FF3000"><font color="#FF2000">▌</font></span><span style="background-color:#FF5000"><font color="#FF4000">▌</font></span><span style="background-color:#FF7000"><font color="#FF6000">▌</font></span><span style="background-color:#FF8F00"><font color="#FF8000">▌</font></span><span style="background-color:#FFAF00"><font color="#FF9F00">▌</font></span><span style="background-color:#FFCF00"><font color="#FFBF00">▌</font></span><span style="background-color:#FFEF00"><font color="#FFDF00">▌</font></span><span style="background-color:#EFFF00"><font color="#FFFF00">▌</font></span><span style="background-color:#CFFF00"><font color="#DFFF00">▌</font></span><span style="background-color:#AFFF00"><font color="#BFFF00">▌</font></span><span style="background-color:#8FFF00"><font color="#9FFF00">▌</font></span><span style="background-color:#70FF00"><font color="#80FF00">▌</font></span><span style="background-color:#50FF00"><font color="#60FF00">▌</font></span><span style="background-color:#30FF00"><font color="#40FF00">▌</font></span><span style="background-color:#10FF00"><font color="#20FF00">▌</font></span><span style="background-color:#00FF10"><font color="#00FF00">▌</font></span><span style="background-color:#00FF30"><font color="#00FF20">▌</font></span><span style="background-color:#00FF50"><font color="#00FF40">▌</font></span><span style="background-color:#00FF70"><font color="#00FF60">▌</font></span><span style="background-color:#00FF8F"><font color="#00FF80">▌</font></span><span style="background-color:#00FFAF"><font color="#00FF9F">▌</font></span><span style="background-color:#00FFCF"><font color="#00FFBF">▌</font></span><span style="background-color:#00FFEF"><font color="#00FFDF">▌</font></span><span style="background-color:#00EFFF"><font color="#00FFFF">▌</font></span><span style="background-color:#00CFFF"><font color="#00DFFF">▌</font></span><span style="background-color:#00AFFF"><font color="#00BFFF">▌</font></span><span style="background-color:#008FFF"><font color="#009FFF">▌</font></span><span style="background-color:#0070FF"><font color="#0080FF">▌</font></span><span style="background-color:#0050FF"><font color="#0060FF">▌</font></span><span style="background-color:#0030FF"><font color="#0040FF">▌</font></span><span style="background-color:#0010FF"><font color="#0020FF">▌</font></span><span style="background-color:#1000FF"><font color="#0000FF">▌</font></span><span style="background-color:#3000FF"><font color="#2000FF">▌</font></span><span style="background-color:#5000FF"><font color="#4000FF">▌</font></span><span style="background-color:#7000FF"><font color="#6000FF">▌</font></span><span style="background-color:#8F00FF"><font color="#8000FF">▌</font></span><span style="background-color:#AF00FF"><font color="#9F00FF">▌</font></span><span style="background-color:#CF00FF"><font color="#BF00FF">▌</font></span><span style="background-color:#EF00FF"><font color="#DF00FF">▌</font></span><span style="background-color:#FF00EF"><font color="#FF00FF">▌</font></span><span style="background-color:#FF00CF"><font color="#FF00DF">▌</font></span><span style="background-color:#FF00AF"><font color="#FF00BF">▌</font></span><span style="background-color:#FF008F"><font color="#FF009F">▌</font></span><span style="background-color:#FF0070"><font color="#FF0080">▌</font></span><span style="background-color:#FF0050"><font color="#FF0060">▌</font></span><span style="background-color:#FF0030"><font color="#FF0040">▌</font></span><span style="background-color:#FF0010"><font color="#FF0020">▌</font></span><font color="#FF0000">▏</font>
 <span style="background-color:#FF0000">        </span>  <b>S</b>  100%   <font color="#808080">▕</font><span style="background-color:#817E7E"><font color="#808080">▌</font></span><span style="background-color:#837C7C"><font color="#827D7D">▌</font></span><span style="background-color:#867979"><font color="#857A7A">▌</font></span><span style="background-color:#897676"><font color="#877878">▌</font></span><span style="background-color:#8B7474"><font color="#8A7575">▌</font></span><span style="background-color:#8E7171"><font color="#8D7272">▌</font></span><span style="background-color:#916E6E"><font color="#8F7070">▌</font></span><span style="background-color:#936C6C"><font color="#926D6D">▌</font></span><span style="background-color:#966969"><font color="#956A6A">▌</font></span><span style="background-color:#996666"><font color="#976868">▌</font></span><span style="background-color:#9B6464"><font color="#9A6565">▌</font></span><span style="background-color:#9E6161"><font color="#9D6262">▌</font></span><span style="background-color:#A15E5E"><font color="#9F6060">▌</font></span><span style="background-color:#A35C5C"><font color="#A25D5D">▌</font></span><span style="background-color:#A65959"><font color="#A55A5A">▌</font></span><span style="background-color:#A95656"><font color="#A75858">▌</font></span><span style="background-color:#AB5454"><font color="#AA5555">▌</font></span><span style="background-color:#AE5151"><font color="#AD5252">▌</font></span><span style="background-color:#B14E4E"><font color="#AF5050">▌</font></span><span style="background-color:#B34C4C"><font color="#B24D4D">▌</font></span><span style="background-color:#B64949"><font color="#B54A4A">▌</font></span><span style="background-color:#B94646"><font color="#B74848">▌</font></span><span style="background-color:#BB4444"><font color="#BA4545">▌</font></span><span style="background-color:#BE4141"><font color="#BD4242">▌</font></span><span style="background-color:#C13E3E"><font color="#BF4040">▌</font></span><span style="background-color:#C33C3C"><font color="#C23D3D">▌</font></span><span style="background-color:#C63939"><font color="#C53A3A">▌</font></span><span style="background-color:#C93636"><font color="#C73838">▌</font></span><span style="background-color:#CB3434"><font color="#CA3535">▌</font></span><span style="background-color:#CE3131"><font color="#CD3232">▌</font></span><span style="background-color:#D12E2E"><font color="#CF3030">▌</font></span><span style="background-color:#D32C2C"><font color="#D22D2D">▌</font></span><span style="background-color:#D62929"><font color="#D52B2B">▌</font></span><span style="background-color:#D82727"><font color="#D72828">▌</font></span><span style="background-color:#DB2424"><font color="#DA2525">▌</font></span><span style="background-color:#DE2121"><font color="#DC2323">▌</font></span><span style="background-color:#E01F1F"><font color="#DF2020">▌</font></span><span style="background-color:#E31C1C"><font color="#E21D1D">▌</font></span><span style="background-color:#E61919"><font color="#E41B1B">▌</font></span><span style="background-color:#E81717"><font color="#E71818">▌</font></span><span style="background-color:#EB1414"><font color="#EA1515">▌</font></span><span style="background-color:#EE1111"><font color="#EC1313">▌</font></span><span style="background-color:#F00F0F"><font color="#EF1010">▌</font></span><span style="background-color:#F30C0C"><font color="#F20D0D">▌</font></span><span style="background-color:#F60909"><font color="#F40B0B">▌</font></span><span style="background-color:#F80707"><font color="#F70808">▌</font></span><span style="background-color:#FB0404"><font color="#FA0505">▌</font></span><span style="background-color:#171421"><font color="#FC0303">▌</font></span><font color="#FF0000">▏</font>
 <span style="background-color:#FF0000">        </span>  <b>L</b>  50%    <font color="#000000">▕</font><span style="background-color:#050000"><font color="#000000">▌</font></span><span style="background-color:#100000"><font color="#0B0000">▌</font></span><span style="background-color:#1B0000"><font color="#150000">▌</font></span><span style="background-color:#250000"><font color="#200000">▌</font></span><span style="background-color:#300000"><font color="#2B0000">▌</font></span><span style="background-color:#3A0000"><font color="#350000">▌</font></span><span style="background-color:#450000"><font color="#400000">▌</font></span><span style="background-color:#500000"><font color="#4A0000">▌</font></span><span style="background-color:#5A0000"><font color="#550000">▌</font></span><span style="background-color:#650000"><font color="#600000">▌</font></span><span style="background-color:#700000"><font color="#6A0000">▌</font></span><span style="background-color:#7A0000"><font color="#750000">▌</font></span><span style="background-color:#850000"><font color="#800000">▌</font></span><span style="background-color:#8F0000"><font color="#8A0000">▌</font></span><span style="background-color:#9A0000"><font color="#950000">▌</font></span><span style="background-color:#A50000"><font color="#9F0000">▌</font></span><span style="background-color:#AF0000"><font color="#AA0000">▌</font></span><span style="background-color:#BA0000"><font color="#B50000">▌</font></span><span style="background-color:#C50000"><font color="#BF0000">▌</font></span><span style="background-color:#CF0000"><font color="#CA0000">▌</font></span><span style="background-color:#DA0000"><font color="#D50000">▌</font></span><span style="background-color:#E40000"><font color="#DF0000">▌</font></span><span style="background-color:#EF0000"><font color="#EA0000">▌</font></span><span style="background-color:#FA0000"><font color="#F40000">▌</font></span><span style="background-color:#FF0505"><font color="#171421">▌</font></span><span style="background-color:#FF1010"><font color="#FF0B0B">▌</font></span><span style="background-color:#FF1B1B"><font color="#FF1515">▌</font></span><span style="background-color:#FF2525"><font color="#FF2020">▌</font></span><span style="background-color:#FF3030"><font color="#FF2B2B">▌</font></span><span style="background-color:#FF3A3A"><font color="#FF3535">▌</font></span><span style="background-color:#FF4545"><font color="#FF4040">▌</font></span><span style="background-color:#FF5050"><font color="#FF4A4A">▌</font></span><span style="background-color:#FF5A5A"><font color="#FF5555">▌</font></span><span style="background-color:#FF6565"><font color="#FF6060">▌</font></span><span style="background-color:#FF7070"><font color="#FF6A6A">▌</font></span><span style="background-color:#FF7A7A"><font color="#FF7575">▌</font></span><span style="background-color:#FF8585"><font color="#FF8080">▌</font></span><span style="background-color:#FF8F8F"><font color="#FF8A8A">▌</font></span><span style="background-color:#FF9A9A"><font color="#FF9595">▌</font></span><span style="background-color:#FFA5A5"><font color="#FF9F9F">▌</font></span><span style="background-color:#FFAFAF"><font color="#FFAAAA">▌</font></span><span style="background-color:#FFBABA"><font color="#FFB5B5">▌</font></span><span style="background-color:#FFC5C5"><font color="#FFBFBF">▌</font></span><span style="background-color:#FFCFCF"><font color="#FFCACA">▌</font></span><span style="background-color:#FFDADA"><font color="#FFD5D5">▌</font></span><span style="background-color:#FFE4E4"><font color="#FFDFDF">▌</font></span><span style="background-color:#FFEFEF"><font color="#FFEAEA">▌</font></span><span style="background-color:#FFFAFA"><font color="#FFF4F4">▌</font></span><font color="#FFFFFF">▏</font>

</pre>
