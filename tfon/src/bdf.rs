//! Parse fonts in `bdf` format
//!
use crate::common::{Bitmap, Prop};
use std::str::{FromStr, Lines};

/// Parser for `bdf` format
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
    /// Create a new `bdf` parser
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

    /// Parse one property
    fn prop(&mut self) -> Option<Prop<'p>> {
        let mut line = self.next_line()?;
        let mut tok = line.split(' ');
        if let Some(key) = tok.next() {
            if key == "ENDFONT" {
                return None;
            } else if key == "ENDPROPERTIES" {
                line = self.next_line()?;
            }
        }
        let mut tok = line.split(' ');
        match tok.next() {
            Some("FONT") => tok.next().map(Prop::FontName),
            Some("SIZE") => { tok.next().and_then(|sz| {
                u8::from_str(sz).ok().map(Prop::FontHeight)
            })}
            Some("FONT_ASCENT") => { tok.next().and_then(|sz| {
                u8::from_str(sz).ok().map(Prop::Baseline)
            })}
            Some("ENCODING") => { tok.next().and_then(|sz| {
                u16::from_str(sz).ok().map(Prop::CodePoint)
            })}
            Some("DWIDTH") => { tok.next().and_then(|w| {
                match u8::from_str(w) {
                    Ok(width) => self.character(width),
                    _ => None,
                }
            })}
            _ => Some(Prop::Unknown(line)),
        }
    }

    /// Parse a bitmap character property
    fn character(&mut self, width: u8) -> Option<Prop<'p>> {
        if width == 0 {
            return None;
        }
        let mut line = self.next_line()?;
        if !line.starts_with("BBX") {
            return None;
        }
        line = self.next_line()?;
        if line != "BITMAP" {
            return None;
        }
        let mut bitmap = Bitmap::new(width);
        while let Some(line) = self.next_line() {
            if line == "ENDCHAR" {
                break;
            } else if is_pixel_row(line) {
                bitmap.push_row(HexBitIter::new(line));
            } else {
                return None;
            }
        }
        Some(Prop::Bitmap(bitmap))
    }
}

/// Check if a line is a pixel row
fn is_pixel_row(line: &str) -> bool {
    line.chars().all(|c| "1234567890ABCDEF".contains(c))
}

/// Hexadecimal bit iterator
struct HexBitIter<'a> {
    line: &'a [u8],
    nybble: u8,
    bit: u8,
}

impl<'a> Iterator for HexBitIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit > 0b0001 {
            self.bit >>= 1;
            Some((self.nybble & self.bit) != 0)
        } else if self.line.len() > 0 {
            self.bit = 0b1000;
            self.nybble = hex_nybble(self.line[0]);
            self.line = &self.line[1..];
            Some((self.nybble & self.bit) != 0)
        } else {
            None
        }
    }
}

impl<'a> HexBitIter<'a> {
    /// Create a new hexadecimal bit iterator
    fn new(line: &'a str) -> Self {
        let line = line.as_bytes();
        HexBitIter { line, nybble: 0, bit: 0 }
    }
}

/// Get byte value of a hexadecimal nybble (char)
fn hex_nybble(v: u8) -> u8 {
    if v >= 48 && v <= 57 {
        // '0' - '9'
        v - 48
    } else if v >= 65 && v <= 70 {
        // 'A' - 'F'
        v + 10 - 65
    } else {
        0
    }
}
