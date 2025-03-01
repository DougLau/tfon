// common.rs
//
use std::iter::repeat;

/// Font parse/write error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O {0}")]
    Io(#[from] std::io::Error),

    #[error("Expected property '{0}'")]
    Expected(&'static str),

    #[error("Unknown font format")]
    UnknownFormat(),
}

/// Result type
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Bitmap of pixels
pub struct Bitmap {
    /// Height in pixels
    pub(crate) height: u8,
    /// Width in pixels
    pub(crate) width: u8,
    /// Vec of pixels
    bmap: Vec<u8>,
}

/// Pixel iterator for bitmaps
pub(crate) struct PixIter<'a> {
    bmap: &'a Bitmap,
    pos: usize,
}

/// Font property
pub enum Prop<'a> {
    /// Unknown property
    Unknown(&'a str),
    /// Font name
    FontName(&'a str),
    /// Font number
    FontNumber(u8),
    /// Font height (pixels)
    FontHeight(u8),
    /// Font width (pixels)
    FontWidth(u8),
    /// Pixel spacing between characters
    CharSpacing(u8),
    /// Pixel spacing between lines
    LineSpacing(u8),
    /// Baseline of characters
    Baseline(u8),
    /// Maximum character number
    MaxCharNumber(u16),
    /// Character code point
    CodePoint(u16),
    /// Character bitmap
    Bitmap(Bitmap),
}

impl Iterator for PixIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let pos = self.pos;
        let len = usize::from(self.bmap.height) * usize::from(self.bmap.width);
        if pos < len {
            self.pos += 1;
            let off = pos >> 3;
            let bit = 7 - (pos & 0b111);
            Some((self.bmap.bmap[off] >> bit) & 1 != 0)
        } else {
            None
        }
    }
}

impl Bitmap {
    /// Create a new bitmap
    pub(crate) fn new(width: u8) -> Self {
        Bitmap {
            height: 0,
            width,
            bmap: Vec::with_capacity(32),
        }
    }

    /// Create a bitmap from bits
    pub fn from_bits(height: u8, width: u8, bmap: Vec<u8>) -> Option<Self> {
        let len = usize::from(height) * usize::from(width);
        if bmap.len() == (len + 7) / 8 {
            Some(Bitmap {
                height,
                width,
                bmap,
            })
        } else {
            None
        }
    }

    /// Get pixel height
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Get pixel width
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Push a row of pixels to the bitmap
    pub(crate) fn push_row(&mut self, row: impl Iterator<Item = bool>) {
        let width = usize::from(self.width);
        let mut pos = usize::from(self.height) * width;
        for pix in row.chain(repeat(false)).take(width) {
            if pos & 0b111 == 0 {
                self.bmap.push(0);
            }
            if pix {
                let off = pos >> 3;
                let bit = 7 - (pos & 0b111);
                self.bmap[off] |= 1 << bit;
            }
            pos += 1;
        }
        self.height += 1;
    }

    /// Get an iterator of all pixels
    pub(crate) fn pixels(&self) -> impl Iterator<Item = bool> + '_ {
        PixIter { bmap: self, pos: 0 }
    }

    /// Convert into a Vec of packed bits
    pub fn into_bits(self) -> Vec<u8> {
        self.bmap
    }
}

impl<'a> Prop<'a> {
    /// Get font name
    pub fn font_name(&self) -> Option<&'a str> {
        match self {
            Prop::FontName(nm) => Some(nm),
            _ => None,
        }
    }

    /// Get font number
    pub fn font_number(&self) -> Option<u8> {
        match self {
            Prop::FontNumber(num) => Some(*num),
            _ => None,
        }
    }

    /// Get character spacing
    pub fn char_spacing(&self) -> Option<u8> {
        match self {
            Prop::CharSpacing(cs) => Some(*cs),
            _ => None,
        }
    }

    /// Get line spacing
    pub fn line_spacing(&self) -> Option<u8> {
        match self {
            Prop::LineSpacing(ls) => Some(*ls),
            _ => None,
        }
    }

    /// Get font height
    pub fn font_height(&self) -> Option<u8> {
        match self {
            Prop::FontHeight(fh) => Some(*fh),
            Prop::Bitmap(bmap) => Some(bmap.height),
            _ => None,
        }
    }

    /// Get code point
    pub fn code_point(&self) -> Option<u16> {
        match self {
            Prop::CodePoint(cp) => Some(*cp),
            _ => None,
        }
    }
}
