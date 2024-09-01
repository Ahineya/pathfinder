use pathfinder_geometry::vector::Vector2F;
use crate::collection::{FontCollection, FontRef};
use crate::TextStyle;

#[derive(Debug)]
pub struct TextLayout {
    pub size: f32,
    pub glyphs: Vec<Glyph>,
    pub advance: Vector2F,
}

#[derive(Debug)]
pub struct Glyph {
    pub font: FontRef,
    pub glyph_id: u32,
    pub offset: Vector2F,
    // TODO: more fields for advance, clusters, etc.
}

impl TextLayout {
    pub(crate) fn new() -> TextLayout {
        TextLayout {
            size: 0.0,
            glyphs: Vec::new(),
            advance: Vector2F::default(),
        }
    }

    pub(crate) fn push_layout(&mut self, other: &TextLayout) {
        self.size = other.size;
        for glyph in &other.glyphs {
            self.glyphs.push(Glyph {
                font: glyph.font.clone(),
                glyph_id: glyph.glyph_id,
                offset: self.advance + glyph.offset,
            });
        }
        self.advance += other.advance;
    }
}

// This implementation just uses advances and doesn't do fallback.
pub fn make_layout(style: &TextStyle, font: &FontRef, text: &str) -> TextLayout {
    // let scale = style.size / (font.font.metrics().units_per_em as f32);
    let mut pos = Vector2F::default();
    let mut glyphs = Vec::new();
    // for c in text.chars() {
    //     if let Some(glyph_id) = font.font.glyph_for_char(c) {
    //         if let Ok(adv) = font.font.advance(glyph_id) {
    //             // TODO(font-kit): this doesn't get hinted advance (hdmx) table
    //             let adv_f = adv * scale;
    //             debug!("{:?}", adv);
    //             let glyph = Glyph {
    //                 font: font.clone(),
    //                 glyph_id,
    //                 offset: pos,
    //             };
    //             glyphs.push(glyph);
    //             pos += adv_f;
    //         }
    //     }
    // }
    TextLayout {
        size: style.size,
        glyphs,
        advance: pos,
    }
}

pub fn layout(style: &TextStyle, collection: &FontCollection, text: &str) -> TextLayout {
    let mut result = TextLayout::new();
    for (range, font) in collection.itemize(text) {
        // result.push_layout(&layout_run(style, font, &text[range]));
        !unimplemented!();
    }
    result
}
