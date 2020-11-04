# Color spaces

Color spaces are reproducible representations of color. Most color spaces have three dimensions that describe properties of the color (such as color hue, saturation, luminance), or their composition of primary colors, using a certain [color model](https://en.wikipedia.org/wiki/Color_model).


## RGB (sRGB)

[sRGB](https://en.wikipedia.org/wiki/SRGB) (often simply called _RGB_) is a color space of the three primary colors **red**, **green** and **blue**. It uses additive color mixing, which is how colored light works. The sRGB color space is therefore used by computer screens, for example.

Colors in the sRGB color space are represented as three values between 0 and 255. 0 means, the primary color is not present, whereas 255 means that the primary color is at full intensity.

### Examples

|                         |                      |
| ----------------------- |--------------------- |
| `rgb(0, 0, 0)`          | ![color](docs/colors/black.png)
| `rgb(255, 0, 0)`        | ![color](docs/colors/red.png)
| `rgb(0, 255, 0)`        | ![color](docs/colors/green.png)
| `rgb(0, 0, 255)`        | ![color](docs/colors/blue.png)
| `rgb(255, 255, 0)`      | ![color](docs/colors/yellow.png)
| `rgb(0, 255, 255)`      | ![color](docs/colors/cyan.png)
| `rgb(255, 0, 255)`      | ![color](docs/colors/magenta.png)
| `rgb(255, 255, 255)`    | ![color](docs/colors/white.png)

## CMY

[CMY](https://en.wikipedia.org/wiki/CMY_color_model) is the counterpart of RGB: It consists of the primary colors **cyan**, **magenta** and **yellow**, and uses subtractive color mixing, so it behaves like color pigments or dyes.

CMY colors are represented as three values between 0 and 1, or 100%.

### Examples

|                |                      |
| ---------------|--------------------- |
| `cmy(0, 0, 0)` | ![color](docs/colors/white.png)
| `cmy(1, 0, 0)` | ![color](docs/colors/cyan.png)
| `cmy(0, 1, 0)` | ![color](docs/colors/magenta.png)
| `cmy(0, 0, 1)` | ![color](docs/colors/yellow.png)
| `cmy(0, 1, 1)` | ![color](docs/colors/red.png)
| `cmy(1, 0, 1)` | ![color](docs/colors/green.png)
| `cmy(1, 1, 0)` | ![color](docs/colors/blue.png)
| `cmy(1, 1, 1)` | ![color](docs/colors/black.png)

## CMYK

CMYK is a variation of CMY that adds a fourth component, the **key**. This is used primarily by color printers: The CMY color components correspond to the color cartridges, whereas the _key_ corresponds to the black cartridge.

### Examples

|                   |                      |
| ------------------|--------------------- |
| `cmy(0, 0, 0, 0)` | ![color](docs/colors/white.png)
| `cmy(1, 0, 0, 0)` | ![color](docs/colors/cyan.png)
| `cmy(0, 1, 0, 0)` | ![color](docs/colors/magenta.png)
| `cmy(0, 0, 1, 0)` | ![color](docs/colors/yellow.png)
| `cmy(0, 1, 1, 0)` | ![color](docs/colors/red.png)
| `cmy(1, 0, 1, 0)` | ![color](docs/colors/green.png)
| `cmy(1, 1, 0, 0)` | ![color](docs/colors/blue.png)
| `cmy(0, 0, 0, 1)` | ![color](docs/colors/black.png)

## HSL

[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV) is a color space designed to be intuitive to understand. It consists of **hue**, **saturation** and **lightness**.

The hue is a value on a radial slice, represented in degrees (0 to 360), where 0 corresponds to red, 120 corresponds to green and 240 to blue. It is equivalent to the hue in the HSV color space.

Saturation and lightness are values between 0 and 1 (or 100%). When the lightness is 0, the color is full black. When it is 1, it is full white. The most vibrant colors are at a lightness of 0.5, or 50%.

### Examples

|                   |                      |
| ------------------|--------------------- |
| <code>hsl(&nbsp; ?, ?,  0)</code> | ![color](docs/colors/black.png)
| <code>hsl(&nbsp; ?, ?,  1)</code> | ![color](docs/colors/white.png)
| <code>hsl(&nbsp; 0, 1, .5)</code> | ![color](docs/colors/red.png)
| `hsl( 60, 1, .5)` | ![color](docs/colors/yellow.png)
| `hsl(120, 1, .5)` | ![color](docs/colors/green.png)
| `hsl(180, 1, .5)` | ![color](docs/colors/cyan.png)
| `hsl(240, 1, .5)` | ![color](docs/colors/blue.png)
| `hsl(300, 1, .5)` | ![color](docs/colors/magenta.png)

## HSV

[HSV](https://en.wikipedia.org/wiki/HSL_and_HSV) is also a color space designed to be intuitive to understand. It consists of **hue**, **saturation** and **value**. The hue is the same as in the HSL model. Hue and saturation together describe the [chromaticity](https://en.wikipedia.org/wiki/Chromaticity).

Saturation and value are again values between 0 and 1. When the value is 0, the color is full black. When the value is 1, the color depends on the hue and saturation.

|                  |                      |
| -----------------|--------------------- |
| <code>hsv(&nbsp; ?, ?, 0)</code> | ![color](docs/colors/black.png)
| <code>hsv(&nbsp; ?, 0, 1)</code> | ![color](docs/colors/white.png)
| <code>hsv(&nbsp; 0, 1, 1)</code> | ![color](docs/colors/red.png)
| `hsv( 60, 1, 1)` | ![color](docs/colors/yellow.png)
| `hsv(120, 1, 1)` | ![color](docs/colors/green.png)
| `hsv(180, 1, 1)` | ![color](docs/colors/cyan.png)
| `hsv(240, 1, 1)` | ![color](docs/colors/blue.png)
| `hsv(300, 1, 1)` | ![color](docs/colors/magenta.png)


## LAB (CIELAB)

[CIELAB](https://en.wikipedia.org/wiki/CIELAB_color_space) is a color space consisting of three values: __L\*__ for the lightness from black (0) to white (100), __a\*__ from green (−) to red (+), and __b\*__ from blue (−) to yellow (+). It was designed so that the same amount of numerical change in these values corresponds to roughly the same amount of visually perceived change. __a\*__ and __b\*__ together describe the [chromaticity](https://en.wikipedia.org/wiki/Chromaticity) (hue and colorfulness).

CIELAB is designed to approximate human vision. It's __L\*__ component closely matches human perception of lightness.

## LCH (CIELCh)

[CIELCh](https://en.wikipedia.org/wiki/CIELAB_color_space#Cylindrical_representation:_CIELCh_or_CIEHLC)

## LUV (CIELUV)

[CIELUV](https://en.wikipedia.org/wiki/CIELUV)

## Hunter Lab

## XYZ

## YXY
