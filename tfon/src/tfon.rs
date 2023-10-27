//! Parse and write fonts in `tfon` format
//!
//! A `tfon` font can be created in any text editor:
//! # Example
//! ```text
//! font_name: tfon example
//! font_number: 2
//! char_spacing: 1
//! line_spacing: 3
//!
//! ch: 52 4
//! ...@@@.
//! ..@.@@.
//! .@..@@.
//! @...@@.
//! @@@@@@@
//! ....@@.
//! ....@@.
//!
//! ch: 65 A
//! .@@@@.
//! @@..@@
//! @@..@@
//! @@@@@@
//! @@..@@
//! @@..@@
//! @@..@@
//! ```
use crate::common::{Bitmap, Error, Prop, Result};
use std::io::Write;
use std::str::{FromStr, Lines};

/// Symbols for all ASCII + Latin 1 characters
const SYMBOL: &[&str] = &[
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF",
    "VT", "FF", "CR", "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK",
    "SYN", "ETB", "CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US", "SP", "!",
    "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?",
    "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
    "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]",
    "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{",
    "|", "}", "~", "DEL", "PAD", "HOP", "BPH", "NBH", "IND", "NEL", "SSA",
    "ESA", "HTS", "HTJ", "LTS", "PLD", "PLU", "RI", "SS2", "SS3", "DCS", "PU1",
    "PU2", "STS", "CCH", "MW", "SPA", "EPA", "SOS", "SGCI", "SCI", "CSI", "ST",
    "OSC", "PM", "APC", "NBSP", "¡", "¢", "£", "¤", "¥", "¦", "§", "¨", "©",
    "ª", "«", "¬", "SHY", "®", "¯", "°", "±", "²", "³", "´", "µ", "¶", "·",
    "¸", "¹", "º", "»", "¼", "½", "¾", "¿", "À", "Á", "Â", "Ã", "Ä", "Å", "Æ",
    "Ç", "È", "É", "Ê", "Ë", "Ì", "Í", "Î", "Ï", "Ð", "Ñ", "Ò", "Ó", "Ô", "Õ",
    "Ö", "×", "Ø", "Ù", "Ú", "Û", "Ü", "Ý", "Þ", "ß", "à", "á", "â", "ã", "ä",
    "å", "æ", "ç", "è", "é", "ê", "ë", "ì", "í", "î", "ï", "ð", "ñ", "ò", "ó",
    "ô", "õ", "ö", "÷", "ø", "ù", "ú", "û", "ü", "ý", "þ", "ÿ",
];

/// Parser for `tfon` format
pub struct Parser<'p> {
    /// Lines to parse
    lines: Lines<'p>,
    /// Pushed back line
    line: Option<&'p str>,
}

impl<'p> Iterator for Parser<'p> {
    type Item = Prop<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        self.prop()
    }
}

impl<'p> Parser<'p> {
    /// Create a new `tfon` parser
    pub fn new(buf: &'p str) -> Self {
        let lines = buf.lines();
        Parser { lines, line: None }
    }

    /// Get the next line
    fn next_line(&mut self) -> Option<&'p str> {
        if self.line.is_some() {
            self.line.take()
        } else {
            for line in self.lines.by_ref() {
                if !line.is_empty() {
                    return Some(line);
                }
            }
            None
        }
    }

    /// Push a line back
    fn push_line(&mut self, line: &'p str) {
        self.line = Some(line);
    }

    /// Parse one property
    fn prop(&mut self) -> Option<Prop<'p>> {
        let line = self.next_line()?;
        match line.split_once(": ") {
            Some(("font_name", val)) => Some(Prop::FontName(val)),
            Some(("font_number", val)) => {
                u8::from_str(val).ok().map(Prop::FontNumber)
            }
            Some(("char_spacing", val)) => {
                u8::from_str(val).ok().map(Prop::CharSpacing)
            }
            Some(("line_spacing", val)) => {
                u8::from_str(val).ok().map(Prop::LineSpacing)
            }
            Some(("ch", val)) => {
                val.split_once(' ').and_then(|(cp, symbol)| {
                    u16::from_str(cp).ok().and_then(|cp| {
                        if symbol == SYMBOL[usize::from(cp)] {
                            Some(Prop::CodePoint(cp))
                        } else {
                            None
                        }
                    })
                })
            }
            Some((key, _val)) => Some(Prop::Unknown(key)),
            _ => self.character(line),
        }
    }

    /// Parse a bitmap character property
    fn character(&mut self, line: &'p str) -> Option<Prop<'p>> {
        let width = u8::try_from(line.len()).unwrap_or(0);
        if width == 0 || !is_pixel_row(line) {
            return None;
        }
        let mut bitmap = Bitmap::new(width);
        bitmap.push_row(row_pixels(line));
        let width = usize::from(width);
        while let Some(line) = self.next_line() {
            if is_pixel_row(line) && line.len() == width {
                bitmap.push_row(row_pixels(line));
            } else {
                self.push_line(line);
                break;
            }
        }
        Some(Prop::Bitmap(bitmap))
    }
}

/// Check if a line is a pixel row
fn is_pixel_row(line: &str) -> bool {
    line.chars().all(|c| c == '.' || c == '@')
}

/// Parse a row into on/off pixels
fn row_pixels(line: &str) -> impl Iterator<Item = bool> + '_ {
    line.chars().map(|c| c == '@')
}

/// Write a font in `tfon` format
pub fn write<'a, W: Write>(
    mut writer: W,
    props: impl Iterator<Item = Prop<'a>>,
) -> Result<()> {
    let props: Vec<_> = props.collect();
    let font_name = props
        .iter()
        .find_map(|v| v.font_name())
        .ok_or(Error::Expected("font_name"))?;
    let font_number = props.iter().find_map(|v| v.font_number()).unwrap_or(1);
    let char_spacing = props.iter().find_map(|v| v.char_spacing()).unwrap_or(0);
    let line_spacing = props.iter().find_map(|v| v.line_spacing()).unwrap_or(0);
    writeln!(writer, "font_name: {font_name:64}")?;
    writeln!(writer, "font_number: {font_number}")?;
    writeln!(writer, "char_spacing: {char_spacing}")?;
    writeln!(writer, "line_spacing: {line_spacing}")?;
    let mut ch = true;
    for prop in props {
        match prop {
            Prop::CodePoint(cp) => match SYMBOL.get(usize::from(cp)) {
                Some(symbol) => {
                    ch = false;
                    writeln!(writer)?;
                    writeln!(writer, "ch: {cp} {symbol}")?;
                }
                _ => return Err(Error::Expected("ch")),
            },
            Prop::Bitmap(bmap) => {
                if ch {
                    return Err(Error::Expected("ch"));
                }
                ch = true;
                let mut col = 0;
                for pix in bmap.pixels() {
                    if pix {
                        write!(writer, "@")?;
                    } else {
                        write!(writer, ".")?;
                    }
                    col += 1;
                    if col >= bmap.width {
                        writeln!(writer)?;
                        col = 0;
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
