// Print Serial string



use colored::{ColoredString, Colorize, Style};

//
// fn resolve_colour(code: String) -> Option<ColoredString> {
//
//     // Remove the leading [, if it doesn't have one it is invalid
//     if code.chars().next() != Some('[') {
//         return None;
//     }
//     // Remove leading [
//     let code = code.chars().skip(1).collect::<String>();
//     // Split the code by ;
//     let codes = code.split(';').collect::<Vec<&str>>();
//     // Check the first code against the styles enum
//
//
//
//
// }



enum LineStyles {
// Text Formatting
//
// 0: Reset / Normal (all attributes off)
// 1: Bold or increased intensity
// 2: Faint, decreased intensity
// 3: Italicized (not widely supported)
// 4: Underlined
// 5: Slow Blink
// 6: Rapid Blink (MS-DOS ANSI.SYS; 150+ per minute; not widely supported)
// 7: Reverse video (swap foreground and background colors)
// 8: Conceal (not widely supported)
// 9: Crossed-out (not widely supported)
//
// Text Colors
// Foreground Colors
//
// 30: Black
// 31: Red
// 32: Green
// 33: Yellow
// 34: Blue
// 35: Magenta
// 36: Cyan
// 37: White
// 90-97: Bright versions of the above colors (i.e., 90 is bright black (grey), 91 is bright red, etc.)
//
// Background Colors
//
// 40: Black
// 41: Red
// 42: Green
// 43: Yellow
// 44: Blue
// 45: Magenta
// 46: Cyan
// 47: White
// 100-107: Bright versions of the above colors
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    SlowBlink = 5,
    RapidBlink = 6,
    ReverseVideo = 7,
    Conceal = 8,
    CrossedOut = 9,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
    BlackBackground = 40,
    RedBackground = 41,
    GreenBackground = 42,
    YellowBackground = 43,
    BlueBackground = 44,
    MagentaBackground = 45,
    CyanBackground = 46,
    WhiteBackground = 47,
    BrightBlackBackground = 100,
    BrightRedBackground = 101,
    BrightGreenBackground = 102,
    BrightYellowBackground = 103,
    BrightBlueBackground = 104,
    BrightMagentaBackground = 105,
    BrightCyanBackground = 106,
    BrightWhiteBackground = 107,
}