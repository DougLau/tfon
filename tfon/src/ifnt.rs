//! Parse and write fonts in `ifnt` format
use crate::common::{Bitmap, Error, Prop, Result};
use std::io::Write;
use std::str::{FromStr, Lines};

/// Parser for `ifnt` format
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
    /// Create a new `ifnt` parser
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
        if let Some(end) = line.strip_prefix("[Char_") {
            if let Some(cp) = end.strip_suffix("]") {
                return u16::from_str(cp)
                    .ok()
                    .and_then(|cp| Some(Prop::CodePoint(cp)));
            }
        }
        match line.split_once("=") {
            Some(("FontName", val)) => Some(Prop::FontName(val)),
            Some(("FontHeight", val)) => {
                u8::from_str(val).ok().map(Prop::FontHeight)
            }
            Some(("CharSpacing", val)) => {
                u8::from_str(val).ok().map(Prop::CharSpacing)
            }
            Some(("LineSpacing", val)) => {
                u8::from_str(val).ok().map(Prop::LineSpacing)
            }
            Some(("MaxCharNumber", val)) => {
                u16::from_str(val).ok().map(Prop::MaxCharNumber)
            }
            Some(("Character", _val)) => self.character(line),
            Some((key, _val)) => Some(Prop::Unknown(key)),
            _ => Some(Prop::Unknown(line)),
        }
    }

    /// Parse a bitmap character property
    fn character(&mut self, _line: &'p str) -> Option<Prop<'p>> {
        let line = self.next_line()?;
        let pix: Vec<_> = parse_row(line).collect();
        let width = u8::try_from(pix.len()).unwrap_or(0);
        if width == 0 {
            return None;
        }
        let mut bitmap = Bitmap::new(width);
        bitmap.push_row(parse_row(line));
        let width = usize::from(width);
        while let Some(line) = self.next_line() {
            let pix: Vec<_> = parse_row(line).collect();
            if pix.len() == width {
                bitmap.push_row(parse_row(line));
            } else {
                self.push_line(line);
                break;
            }
        }
        Some(Prop::Bitmap(bitmap))
    }
}

/// Parse a bitmap row
fn parse_row(line: &str) -> impl Iterator<Item = bool> + '_ {
    if line.starts_with("row") {
        if let Some((_key, val)) = line.split_once("=") {
            return val.chars().filter_map(pixel_filter_map);
        }
    }
    "".chars().filter_map(pixel_filter_map)
}

/// Filter/map a pixel to a bool
fn pixel_filter_map(c: char) -> Option<bool> {
    if c == '.' {
        Some(false)
    } else if c == 'X' {
        Some(true)
    } else {
        None
    }
}

/// Write a font in `ifnt` format
pub fn write<'a, W: Write>(
    mut writer: W,
    props: impl Iterator<Item = Prop<'a>>,
) -> Result<()> {
    let props: Vec<_> = props.collect();
    let font_name = props
        .iter()
        .find_map(|v| v.font_name())
        .ok_or(Error::Expected("FontName"))?;
    let font_height = props
        .iter()
        .find_map(|v| v.font_height())
        .ok_or(Error::Expected("FontHeight"))?;
    let char_spacing = props
        .iter()
        .find_map(|v| v.char_spacing())
        .ok_or(Error::Expected("CharSpacing"))?;
    let line_spacing = props
        .iter()
        .find_map(|v| v.line_spacing())
        .ok_or(Error::Expected("LineSpacing"))?;
    let max_char_num = props
        .iter()
        .filter_map(|v| v.code_point())
        .last()
        .ok_or(Error::Expected("MaxCharNumber"))?;
    writeln!(writer, "[FontInfo]")?;
    writeln!(writer, "FontName={font_name:64}")?;
    writeln!(writer, "FontHeight={font_height}")?;
    writeln!(writer, "CharSpacing={char_spacing}")?;
    writeln!(writer, "LineSpacing={line_spacing}")?;
    writeln!(writer, "MaxCharNumber={max_char_num}")?;
    let mut ch = true;
    for prop in props {
        match prop {
            Prop::CodePoint(cp) => {
                ch = false;
                writeln!(writer)?;
                writeln!(writer, "[Char_{cp}]")?;
                if cp >= 32 && cp < 127 {
                    let c = char::from_u32(u32::from(cp)).unwrap();
                    writeln!(writer, "Character='{c}'")?;
                } else {
                    writeln!(writer, "Character=0x{cp:x}")?;
                }
            }
            Prop::Bitmap(bmap) => {
                if ch {
                    return Err(Error::Expected("Character"));
                }
                ch = true;
                let mut col = 0;
                let mut row = 1;
                for pix in bmap.pixels() {
                    if col == 0 {
                        write!(writer, "row{row:02}=")?;
                    }
                    if pix {
                        write!(writer, " X")?;
                    } else {
                        write!(writer, " .")?;
                    }
                    col += 1;
                    if col >= bmap.width {
                        writeln!(writer)?;
                        col = 0;
                        row += 1;
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
