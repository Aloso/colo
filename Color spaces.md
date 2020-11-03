# Color spaces

Color spaces are reproducible representations of color. Most color spaces have three dimensions that describe properties of the color (such as color hue, saturation, luminance), or their composition of primary colors, using a certain [color model](https://en.wikipedia.org/wiki/Color_model).


## sRGB

[sRGB](https://en.wikipedia.org/wiki/SRGB) (often simply called _RGB_) is a color space of the three primary colors **red**, **green** and **blue**. It uses additive color mixing, which is how colored light works. The sRGB color space is therefore used by computer screens, for example.

Colors in the sRGB color space are represented as three values between 0 and 255. 0 means, the primary color is not present, whereas 255 means that the primary color is at full intensity.

### Examples

|           |                         |                      |
| --------- | ----------------------- |--------------------- |
| Black     | `rgb(0, 0, 0)`          | <i style="background:black">&nbsp; &nbsp; &nbsp;</i>
| Red       | `rgb(255, 0, 0)`        | <i style="background:red">&nbsp; &nbsp; &nbsp;</i>
| Green     | `rgb(0, 255, 0)`        | <i style="background:lime">&nbsp; &nbsp; &nbsp;</i>
| Blue      | `rgb(0, 0, 255)`        | <i style="background:blue">&nbsp; &nbsp; &nbsp;</i>
| Yellow    | `rgb(255, 255, 0)`      | <i style="background:yellow">&nbsp; &nbsp; &nbsp;</i>
| Cyan      | `rgb(0, 255, 255)`      | <i style="background:cyan">&nbsp; &nbsp; &nbsp;</i>
| Magenta   | `rgb(255, 0, 255)`      | <i style="background:magenta">&nbsp; &nbsp; &nbsp;</i>
| White     | `rgb(255, 255, 255)`    | <i style="background:white">&nbsp; &nbsp; &nbsp;</i>

## CMY

[CMY](https://en.wikipedia.org/wiki/CMY_color_model) is the counterpart of RGB: It consists of the primary colors **cyan**, **magenta** and **yellow**, and uses subtractive color mixing, so it behaves like color pigments or dyes.

CMY colors are represented as three values between 0 and 1, or 100%.

### Examples

|           |                |                      |
| --------- | ---------------|--------------------- |
| White     | `cmy(0, 0, 0)` | <i style="background:white">&nbsp; &nbsp; &nbsp;</i>
| Cyan      | `cmy(1, 0, 0)` | <i style="background:cyan">&nbsp; &nbsp; &nbsp;</i>
| Magenta   | `cmy(0, 1, 0)` | <i style="background:magenta">&nbsp; &nbsp; &nbsp;</i>
| Yellow    | `cmy(0, 0, 1)` | <i style="background:yellow">&nbsp; &nbsp; &nbsp;</i>
| Red       | `cmy(0, 1, 1)` | <i style="background:red">&nbsp; &nbsp; &nbsp;</i>
| Green     | `cmy(1, 0, 1)` | <i style="background:lime">&nbsp; &nbsp; &nbsp;</i>
| Blue      | `cmy(1, 1, 0)` | <i style="background:blue">&nbsp; &nbsp; &nbsp;</i>
| Black     | `cmy(1, 1, 1)` | <i style="background:black">&nbsp; &nbsp; &nbsp;</i>

## CMYK

CMYK is a variation of CMY that adds a fourth component, the **key**. This is used primarily by color printers: The CMY color components correspond to the color cartridges, whereas the _key_ corresponds to the black cartridge.

### Examples

|           |                   |                      |
| --------- | ------------------|--------------------- |
| White     | `cmy(0, 0, 0, 0)` | <i style="background:white">&nbsp; &nbsp; &nbsp;</i>
| Cyan      | `cmy(1, 0, 0, 0)` | <i style="background:cyan">&nbsp; &nbsp; &nbsp;</i>
| Magenta   | `cmy(0, 1, 0, 0)` | <i style="background:magenta">&nbsp; &nbsp; &nbsp;</i>
| Yellow    | `cmy(0, 0, 1, 0)` | <i style="background:yellow">&nbsp; &nbsp; &nbsp;</i>
| Red       | `cmy(0, 1, 1, 0)` | <i style="background:red">&nbsp; &nbsp; &nbsp;</i>
| Green     | `cmy(1, 0, 1, 0)` | <i style="background:lime">&nbsp; &nbsp; &nbsp;</i>
| Blue      | `cmy(1, 1, 0, 0)` | <i style="background:blue">&nbsp; &nbsp; &nbsp;</i>
| Black     | `cmy(0, 0, 0, 1)` | <i style="background:black">&nbsp; &nbsp; &nbsp;</i>

## HSL

[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV) is a color space designed to be intuitive to understand. It consists of **hue**, **saturation** and **lightness**.

The hue is a value on a radial slice, represented in degrees (0 to 360), where 0 corresponds to red, 120 corresponds to green and 240 to blue. It is equivalent to the hue in the HSV color space.

Saturation and lightness are values between 0 and 1 (or 100%). When the lightness is 0, the color is full black. When it is 1, it is full white. The most vibrant colors are at a lightness of 0.5, or 50%.

### Examples

|           |                   |                      |
| --------- | ------------------|--------------------- |
| Black     | `hsl(  ?, ?,  0)` | <i style="background:black">&nbsp; &nbsp; &nbsp;</i>
| White     | `hsl(  ?, ?,  1)` | <i style="background:white">&nbsp; &nbsp; &nbsp;</i>
| Red       | `hsl(  0, 1, .5)` | <i style="background:red">&nbsp; &nbsp; &nbsp;</i>
| Yellow    | `hsl( 60, 1, .5)` | <i style="background:yellow">&nbsp; &nbsp; &nbsp;</i>
| Green     | `hsl(120, 1, .5)` | <i style="background:lime">&nbsp; &nbsp; &nbsp;</i>
| Cyan      | `hsl(180, 1, .5)` | <i style="background:cyan">&nbsp; &nbsp; &nbsp;</i>
| Blue      | `hsl(240, 1, .5)` | <i style="background:blue">&nbsp; &nbsp; &nbsp;</i>
| Magenta   | `hsl(300, 1, .5)` | <i style="background:magenta">&nbsp; &nbsp; &nbsp;</i>

## HSV

[HSV](https://en.wikipedia.org/wiki/HSL_and_HSV) is also a color space designed to be intuitive to understand. It consists of **hue**, **saturation** and **value**. The hue is the same as in the HSL model.

Saturation and value are again values between 0 and 1. When the value is 0, the color is full black. When the value is 1, the color depends on the hue and saturation.

|           |                  |                      |
| --------- | -----------------|--------------------- |
| Black     | `hsv(  ?, ?, 0)` | <i style="background:black">&nbsp; &nbsp; &nbsp;</i>
| White     | `hsv(  ?, 0, 1)` | <i style="background:white">&nbsp; &nbsp; &nbsp;</i>
| Red       | `hsv(  0, 1, 1)` | <i style="background:red">&nbsp; &nbsp; &nbsp;</i>
| Yellow    | `hsv( 60, 1, 1)` | <i style="background:yellow">&nbsp; &nbsp; &nbsp;</i>
| Green     | `hsv(120, 1, 1)` | <i style="background:lime">&nbsp; &nbsp; &nbsp;</i>
| Cyan      | `hsv(180, 1, 1)` | <i style="background:cyan">&nbsp; &nbsp; &nbsp;</i>
| Blue      | `hsv(240, 1, 1)` | <i style="background:blue">&nbsp; &nbsp; &nbsp;</i>
| Magenta   | `hsv(300, 1, 1)` | <i style="background:magenta">&nbsp; &nbsp; &nbsp;</i>

## LCH

## LUV

## LAB

## Hunter Lab

## XYZ

## YXY
