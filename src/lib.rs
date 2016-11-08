extern crate regex;
#[macro_use]
extern crate lazy_static;

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
    /// returns an instance of Colorize, with default color mappings.
    pub fn new() -> Colorize {
        Colorize {
            colors: DEFAULT_COLORS.clone(),
            disable: false,
            reset: true,
        }
    }

    /// colorizes a string `v` according to the settings setup in the struct.
    pub fn color(&self, v: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)\[[a-z0-9_-]+\]").unwrap();
        }
        let matches: Vec<(usize, usize)> = RE.find_iter(v).collect();
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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?i)(\[[a-z0-9_-]+\])+").unwrap();
        }
        RE.captures(v).and_then(|cap| cap.at(1).map(ToOwned::to_owned))
    }
}

lazy_static! {
    static ref DEFAULT_COLORS: HashMap<&'static str, &'static str>  = {
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

        default_colors
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ref c = Colorize::new();
        expect(c, "foo", "foo");
        expect(c, "[blue]foo", "\x1B[34mfoo\x1B[0m");
        expect(c, "foo[blue]foo", "foo\x1B[34mfoo\x1B[0m");
        expect(c, "foo[what]foo", "foo[what]foo");
        expect(c, "foo[_blue_]foo", "foo\x1B[44mfoo\x1B[0m");
        expect(c, "foo[bold]foo", "foo\x1B[1mfoo\x1B[0m");
        expect(c, "[blue]foo[bold]bar", "\x1B[34mfoo\x1B[1mbar\x1B[0m");
        expect(c, "[underline]foo[reset]bar", "\x1B[4mfoo\x1B[0mbar\x1B[0m");
    }

    fn expect(c: &Colorize, src: &str, expected: &str) {
        let colored = c.color(src);
        assert_eq!(expected,
                   colored,
                   "{:?}, {:?}, {:?}",
                   expected,
                   colored,
                   src);
    }
}
