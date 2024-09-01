//! The font collection type.

use std::{fmt, slice};
use std::ops::Range;
use std::sync::Arc;
use rustybuzz::Face;
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::transform2d::Transform2F;
use crate::error::{FontLoadingError, GlyphLoadingError};
use crate::handle::Handle;
use crate::hinting::HintingOptions;
use crate::metrics::Metrics;
use crate::outline::{Outline, OutlineSink};
use crate::properties::Properties;
use crate::rasterization::RasterizationOptions;

#[derive(Clone)]
pub struct Font {
    pub face: Arc<Face<'static>>,
    pub font_data: Arc<Vec<u8>>,
}

impl Font {
    pub fn postscript_name(&self) -> Option<String> {
        Some("Sevillana-Regular".to_string())
    }
    
    pub fn full_name(&self) -> String {
        "Ainanenane".to_string()
    }
    
    pub fn glyph_for_char(&self, c: char) -> Option<u32> {
        None
    }
    
    pub fn metrics(&self) -> Metrics {
        Metrics {
            units_per_em: 1000,
            ascent: 800.0,
            descent: -200.0,
            line_gap: 0.0,
            underline_position: -100.0,
            underline_thickness: 50.0,
            cap_height: 700.0,
            x_height: 500.0,
            bounding_box: RectF::default(),
        }
    }
    
    pub fn outline<S>(
        &self,
        glyph_id: u32,
        _: HintingOptions,
        sink: &mut S,
    ) -> Result<(), GlyphLoadingError>
    where
        S: OutlineSink,
    {
        let mut outline = Outline::new();
        // let glyph = self.glyph_for_char(c).unwrap();
        // self.font.outline(glyph, hinting, sink)
        // outline.copy_to(sink);
        Ok(())
    }

    pub fn from_handle(handle: &Handle) -> Result<Self, FontLoadingError> {
        match *handle {
            Handle::Memory {
                ref bytes,
                font_index,
            } => Self::from_bytes(Arc::clone(bytes), font_index, Arc::clone(bytes)),
            _ => unimplemented!(),
        }
    }

    fn from_bytes(font_data: Arc<Vec<u8>>, font_index: u32, data: Arc<Vec<u8>>) -> Result<Self, FontLoadingError> {
        let boxed_data: Box<[u8]> = font_data.as_slice().into();
        let leaked_data: &'static [u8] = Box::leak(boxed_data);
        let face = Face::from_slice(&leaked_data, font_index).unwrap();

        Ok(Self {
            face: Arc::new(face),
            font_data: data
        })
    }

    pub fn properties(&self) -> Properties {
        let mut properties = Properties::default();
        
        properties
    }
    
    pub fn family_name(&self) -> String {
        "Sevillana".to_string()
    }
    
    pub fn raster_bounds(
        &self,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        _: HintingOptions,
        _: RasterizationOptions,
    ) -> Result<RectI, GlyphLoadingError> {
        Ok(RectI::default())
    }
}


/// A collection of fonts
pub struct FontCollection {
    pub(crate) families: Vec<FontFamily>,
}

pub struct FontFamily {
    // TODO: multiple weights etc
    pub(crate) fonts: Vec<FontRef>,
}

// Design question: deref to Font?
#[derive(Clone)]
pub struct FontRef {
    pub font: Arc<Font>,
}

impl<'a> fmt::Debug for FontRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontRef({})", self.font.full_name())
    }
}

pub struct Itemizer<'a> {
    text: &'a str,
    collection: &'a FontCollection,
    ix: usize,
}

impl<'a> FontRef {
    pub fn new(font: Font) -> FontRef {
        FontRef {
            font: Arc::new(font),
        }
    }
}

impl<'a> FontFamily {
    pub fn new() -> FontFamily {
        FontFamily { fonts: Vec::new() }
    }

    pub fn add_font(&mut self, font: FontRef) {
        self.fonts.push(font);
    }

    /// Create a collection consisting of a single font
    pub fn new_from_font(font: Font) -> FontFamily {
        let mut result = FontFamily::new();
        result.add_font(FontRef::new(font));
        result
    }

    pub fn supports_codepoint(&self, c: char) -> bool {
        if let Some(font) = self.fonts.first() {
            let glyph_id = font.font.glyph_for_char(c);
            // TODO(font-kit): We're getting Some(0) for unsupported glyphs on CoreText
            // and DirectWrite
            glyph_id.unwrap_or(0) != 0
        } else {
            false
        }
    }
}

impl<'a> FontCollection {
    pub fn new() -> FontCollection {
        FontCollection {
            families: Vec::new(),
        }
    }

    pub fn add_family(&mut self, family: FontFamily) {
        self.families.push(family);
    }

    pub fn itemize<'b>(&'b self, text: &'b str) -> Itemizer<'b> {
        Itemizer {
            text,
            collection: self,
            ix: 0,
        }
    }

    // TODO: other style params, including locale list
    fn choose_font(&self, c: char) -> usize {
        self.families
            .iter()
            .position(|family| family.supports_codepoint(c))
            .unwrap_or(0)
    }
}

// This is the PostScript name of the font. Eventually this should be a unique ID.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct FontId {
    postscript_name: String,
}

impl FontId {
    pub(crate) fn from_font(font: &FontRef) -> FontId {
        FontId { postscript_name: font.font.postscript_name().unwrap_or_default() }
    }
}

impl<'a> Iterator for Itemizer<'a> {
    type Item = (Range<usize>, &'a FontRef);

    fn next(&mut self) -> Option<(Range<usize>, &'a FontRef)> {
        let start = self.ix;
        let mut chars_iter = self.text[start..].chars();
        if let Some(c) = chars_iter.next() {
            let mut end = start + c.len_utf8();
            let font_ix = self.collection.choose_font(c);
            while let Some(c) = chars_iter.next() {
                if font_ix != self.collection.choose_font(c) {
                    break;
                }
                end += c.len_utf8();
            }
            self.ix = end;

            if &self.collection.families.len() >= &1 {
                Some((start..end, &self.collection.families[font_ix].fonts[0]))
            }
            else {
                None
            }
        } else {
            None
        }
    }
}
