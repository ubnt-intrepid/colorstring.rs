#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate regex;

use std::collections::HashMap;
use regex::Regex;

/// colorizes the strings, giving you the ability to customize
/// some of the colorization process.
pub struct Colorize {
    /// maps a color string to the code for that color.
    colors: HashMap<&'static str, &'static str>,
    /// whether color attributes are ignored.
    disable: bool,
    /// whether it reset the color after each colorization,
    /// by adding a reset code at the end.
    reset: bool,
}

impl Colorize {
    /// colorizes a string `v` according to the settings setup in the struct.
    pub fn color(&self, v: &str) -> String {
        let re: Regex = Regex::new(r"(?i)\[[a-z0-9_-]+\]").unwrap();
        let matches: Vec<(usize, usize)> = re.find_iter(v).collect();
        if matches.len() == 0 {
            return v.to_owned();
        }

        let mut result = String::new();
        let mut colored = false;
        let mut m = (0, 0);
        for nm in matches {
            result += &v[(m.1)..(nm.0)];
            m = nm;

            let mut replace = String::new();
            if let Some(ref code) = self.colors.get(&v[(m.0 + 1)..(m.1 - 1)]) {
                colored = true;

                if !self.disable {
                    replace = format!("\x1B[{}m", code);
                }
            } else {
                replace = String::from(&v[(m.0)..(m.1)]);
            }

            result += &replace;
        }

        result += &v[(m.1)..];

        if colored && self.reset && !self.disable {
            result += "\x1B[0m";
        }

        result
    }

    /// returns the first color sequence that exists in this string.
    pub fn color_prefix(&self, v: &str) -> Option<String> {
        let re = Regex::new(r"^(?i)(\[[a-z0-9_-]+\])+").unwrap();
        re.captures(v).and_then(|cap| cap.at(1).map(ToOwned::to_owned))
    }
}

pub struct Context {
    /// the default colors used when colorizing.
    default_colors: HashMap<&'static str, &'static str>,

    /// [undocumented]
    def: Colorize,
}

pub fn init() -> Context {
    let mut default_colors = HashMap::new();
    default_colors.insert("default", "39");
    default_colors.insert("_default_", "49");
    // foreground colors
    default_colors.insert("black", "30");
    default_colors.insert("red", "31");
    default_colors.insert("green", "32");
    default_colors.insert("yellow", "33");
    default_colors.insert("blue", "34");
    default_colors.insert("magenta", "35");
    default_colors.insert("cyan", "36");
    default_colors.insert("light_gray", "37");
    default_colors.insert("dark_gray", "90");
    default_colors.insert("light_red", "91");
    default_colors.insert("light_green", "92");
    default_colors.insert("light_yellow", "93");
    default_colors.insert("light_blue", "94");
    default_colors.insert("light_magenta", "95");
    default_colors.insert("light_cyan", "96");
    default_colors.insert("white", "97");

    // background colors
    default_colors.insert("_black_", "40");
    default_colors.insert("_red_", "41");
    default_colors.insert("_green_", "42");
    default_colors.insert("_yellow_", "43");
    default_colors.insert("_blue_", "44");
    default_colors.insert("_magenta_", "45");
    default_colors.insert("_cyan_", "46");
    default_colors.insert("_light_gray_", "47");
    default_colors.insert("_dark_gray_", "100");
    default_colors.insert("_light_red_", "101");
    default_colors.insert("_light_green_", "102");
    default_colors.insert("_light_yellow_", "103");
    default_colors.insert("_light_blue_", "104");
    default_colors.insert("_light_magenta_", "105");
    default_colors.insert("_light_cyan_", "106");
    default_colors.insert("_white_", "107");

    // attributes
    default_colors.insert("bold", "1");
    default_colors.insert("dim", "2");
    default_colors.insert("underline", "4");
    default_colors.insert("blink_slow", "5");
    default_colors.insert("blink_fast", "6");
    default_colors.insert("invert", "7");
    default_colors.insert("hidden", "8");

    // Reset to reset everything to their defaults
    default_colors.insert("reset", "0");
    default_colors.insert("reset_bold", "21");

    let def = Colorize {
        colors: default_colors.clone(),
        disable: false,
        reset: true,
    };

    Context {
        default_colors: default_colors,
        def: def,
    }
}

impl Context {
    /// colorizes the string using the default settings.
    pub fn color(&self, v: &str) -> String {
        self.def.color(v)
    }

    /// returns the color sequence that prefixes the given text.
    pub fn color_prefix(&self, v: &str) -> Option<String> {
        self.def.color_prefix(v)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use super::*;

    #[test]
    fn it_works() {
        let ctx = init();

        let s = "[red]hoge [green][blink_slow]hoee";
        let colored = ctx.color(s);
        let mut w = io::stdout();
        w.write_all(colored.as_bytes()).unwrap();
        w.write_all(b"\n").unwrap();
    }
}
