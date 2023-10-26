//! Parse fonts in `ifnt` (X) format
//!
//! This is an obsolete font format which is sort of half way between `ifnt`
//! and `tfon`.  Writing is not supported.
use crate::common::{Bitmap, Prop};
use std::str::{FromStr, Lines};

/// Parser for `ifnt` (X) format
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
    /// Create a new `ifnt` (X) parser
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
            Some(("name", val)) => Some(Prop::FontName(val)),
            Some(("font_number", val)) => {
                u8::from_str(val).ok().map(Prop::FontNumber)
            }
            Some(("height", val)) => {
                u8::from_str(val).ok().map(Prop::FontHeight)
            }
            Some(("width", val)) => u8::from_str(val).ok().map(Prop::FontWidth),
            Some(("char_spacing", val)) => {
                u8::from_str(val).ok().map(Prop::CharSpacing)
            }
            Some(("line_spacing", val)) => {
                u8::from_str(val).ok().map(Prop::LineSpacing)
            }
            Some(("codepoint", val)) => {
                let cp = val.split_ascii_whitespace().next().unwrap_or(val);
                u16::from_str(cp).ok().map(Prop::CodePoint)
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
    line.chars().all(|c| c == '.' || c == 'X')
}

/// Parse a row into on/off pixels
fn row_pixels(line: &str) -> impl Iterator<Item = bool> + '_ {
    line.chars().map(|c| c == 'X')
}
