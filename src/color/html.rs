use std::io::{stdout, Write};

use super::{
    hex,
    space::{self, Rgb},
};
use anyhow::Result;
use colored::{Color::TrueColor, Colorize};

/// List of HTML color, taken from
/// https://www.w3schools.com/colors/colors_groups.asp
const HTML_COLOR_NAMES: &[(&str, u32)] = &[
    ("pink", 0xffc0cb),
    ("lightpink", 0xffb6c1),
    ("hotpink", 0xff69b4),
    ("deeppink", 0xff1493),
    ("palevioletred", 0xdb7093),
    ("mediumvioletred", 0xc71585),
    ("lavender", 0xe6e6fa),
    ("thistle", 0xd8bfd8),
    ("plum", 0xdda0dd),
    ("orchid", 0xda70d6),
    ("violet", 0xee82ee),
    ("fuchsia", 0xff00ff),
    ("magenta", 0xff00ff),
    ("mediumorchid", 0xba55d3),
    ("darkorchid", 0x9932cc),
    ("darkviolet", 0x9400d3),
    ("blueviolet", 0x8a2be2),
    ("darkmagenta", 0x8b008b),
    ("purple", 0x800080),
    ("mediumpurple", 0x9370db),
    ("mediumslateblue", 0x7b68ee),
    ("slateblue", 0x6a5acd),
    ("darkslateblue", 0x483d8b),
    ("rebeccapurple", 0x663399),
    ("indigo", 0x4b0082),
    ("lightsalmon", 0xffa07a),
    ("salmon", 0xfa8072),
    ("darksalmon", 0xe9967a),
    ("lightcoral", 0xf08080),
    ("indianred", 0xcd5c5c),
    ("crimson", 0xdc143c),
    ("red", 0xff0000),
    ("firebrick", 0xb22222),
    ("darkred", 0x8b0000),
    ("orange", 0xffa500),
    ("darkorange", 0xff8c00),
    ("coral", 0xff7f50),
    ("tomato", 0xff6347),
    ("orangered", 0xff4500),
    ("gold", 0xffd700),
    ("yellow", 0xffff00),
    ("lightyellow", 0xffffe0),
    ("lemonchiffon", 0xfffacd),
    ("lightgoldenrodyellow", 0xfafad2),
    ("papayawhip", 0xffefd5),
    ("moccasin", 0xffe4b5),
    ("peachpuff", 0xffdab9),
    ("palegoldenrod", 0xeee8aa),
    ("khaki", 0xf0e68c),
    ("darkkhaki", 0xbdb76b),
    ("greenyellow", 0xadff2f),
    ("chartreuse", 0x7fff00),
    ("lawngreen", 0x7cfc00),
    ("lime", 0x00ff00),
    ("limegreen", 0x32cd32),
    ("palegreen", 0x98fb98),
    ("lightgreen", 0x90ee90),
    ("mediumspringgreen", 0x00fa9a),
    ("springgreen", 0x00ff7f),
    ("mediumseagreen", 0x3cb371),
    ("seagreen", 0x2e8b57),
    ("forestgreen", 0x228b22),
    ("green", 0x008000),
    ("darkgreen", 0x006400),
    ("yellowgreen", 0x9acd32),
    ("olivedrab", 0x6b8e23),
    ("darkolivegreen", 0x556b2f),
    ("mediumaquamarine", 0x66cdaa),
    ("darkseagreen", 0x8fbc8f),
    ("lightseagreen", 0x20b2aa),
    ("darkcyan", 0x008b8b),
    ("teal", 0x008080),
    ("aqua", 0x00ffff),
    ("cyan", 0x00ffff),
    ("lightcyan", 0xe0ffff),
    ("paleturquoise", 0xafeeee),
    ("aquamarine", 0x7fffd4),
    ("turquoise", 0x40e0d0),
    ("mediumturquoise", 0x48d1cc),
    ("darkturquoise", 0x00ced1),
    ("cadetblue", 0x5f9ea0),
    ("steelblue", 0x4682b4),
    ("lightsteelblue", 0xb0c4de),
    ("lightblue", 0xadd8e6),
    ("powderblue", 0xb0e0e6),
    ("lightskyblue", 0x87cefa),
    ("skyblue", 0x87ceeb),
    ("cornflowerblue", 0x6495ed),
    ("deepskyblue", 0x00bfff),
    ("dodgerblue", 0x1e90ff),
    ("royalblue", 0x4169e1),
    ("blue", 0x0000ff),
    ("mediumblue", 0x0000cd),
    ("darkblue", 0x00008b),
    ("navy", 0x000080),
    ("midnightblue", 0x191970),
    ("cornsilk", 0xfff8dc),
    ("blanchedalmond", 0xffebcd),
    ("bisque", 0xffe4c4),
    ("navajowhite", 0xffdead),
    ("wheat", 0xf5deb3),
    ("burlywood", 0xdeb887),
    ("tan", 0xd2b48c),
    ("rosybrown", 0xbc8f8f),
    ("sandybrown", 0xf4a460),
    ("goldenrod", 0xdaa520),
    ("darkgoldenrod", 0xb8860b),
    ("peru", 0xcd853f),
    ("chocolate", 0xd2691e),
    ("olive", 0x808000),
    ("saddlebrown", 0x8b4513),
    ("sienna", 0xa0522d),
    ("brown", 0xa52a2a),
    ("maroon", 0x800000),
    ("white", 0xffffff),
    ("snow", 0xfffafa),
    ("honeydew", 0xf0fff0),
    ("mintcream", 0xf5fffa),
    ("azure", 0xf0ffff),
    ("aliceblue", 0xf0f8ff),
    ("ghostwhite", 0xf8f8ff),
    ("whitesmoke", 0xf5f5f5),
    ("seashell", 0xfff5ee),
    ("beige", 0xf5f5dc),
    ("oldlace", 0xfdf5e6),
    ("floralwhite", 0xfffaf0),
    ("ivory", 0xfffff0),
    ("antiquewhite", 0xfaebd7),
    ("linen", 0xfaf0e6),
    ("lavenderblush", 0xfff0f5),
    ("mistyrose", 0xffe4e1),
    ("gainsboro", 0xdcdcdc),
    ("lightgray", 0xd3d3d3),
    ("silver", 0xc0c0c0),
    ("darkgray", 0xa9a9a9),
    ("dimgray", 0x696969),
    ("gray", 0x808080),
    ("lightslategray", 0x778899),
    ("slategray", 0x708090),
    ("darkslategray", 0x2f4f4f),
    ("lightgrey", 0xd3d3d3),
    ("darkgrey", 0xa9a9a9),
    ("dimgrey", 0x696969),
    ("grey", 0x808080),
    ("lightslategrey", 0x778899),
    ("slategrey", 0x708090),
    ("darkslategrey", 0x2f4f4f),
    ("black", 0x000000),
];

/// Gets an HTML color. The name (e.g. `Rebeccapurple`) is converted to
/// lowercase first. If this function is called many times, it's more efficient
/// to build a HashMap.
pub fn get(name: &str) -> Option<Rgb> {
    let name = name.to_lowercase();
    HTML_COLOR_NAMES
        .iter()
        .filter(|&&(k, _)| k == name)
        .map(|&(_, hex)| Rgb::from_hex(hex))
        .next()
}

pub fn get_similar(name: &str) -> Vec<(&'static str, f64)> {
    let name = name.to_lowercase();
    HTML_COLOR_NAMES
        .iter()
        .filter_map(|&(k, _)| {
            let score = strsim::jaro_winkler(k, &name);
            if score > 0.85 {
                Some((k, score))
            } else {
                None
            }
        })
        .collect()
}

/// Gets the name of a HTML color. The name (e.g. `Rebeccapurple`) is converted
/// to lowercase first. If this function is called many times, it's more
/// efficient to build a HashMap.
pub fn get_name(color: Rgb) -> Option<&'static str> {
    let hex = hex::rgb_to_u32(color);
    HTML_COLOR_NAMES
        .iter()
        .filter(|&&(_, v)| v == hex)
        .map(|&(name, _)| name)
        .next()
}

pub fn show_all() -> Result<()> {
    let mut stdout = stdout();

    let mut even = false;

    for &(name, color) in HTML_COLOR_NAMES {
        if name == "magenta" || name == "aqua" || name.ends_with("grey") {
            continue;
        }
        let color = Rgb::from_hex(color);
        let hsl: space::Hsl = color.into();
        let color = TrueColor {
            r: color.r as u8,
            g: color.g as u8,
            b: color.b as u8,
        };
        let text_color = if hsl.l < 0.5 {
            TrueColor {
                r: 255,
                g: 255,
                b: 255,
            }
        } else {
            TrueColor { r: 0, g: 0, b: 0 }
        };
        let name = format!("   {}{}", name, &"                     "[name.len()..]);
        write!(stdout, "{}", name.color(text_color).on_color(color))?;
        if even {
            writeln!(stdout)?;
        }
        even = !even;
    }
    writeln!(stdout)?;

    Ok(())
}
