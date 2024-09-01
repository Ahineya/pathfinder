use rustybuzz::{BufferClusterLevel, Direction, Face, Feature, Language};
use pathfinder_geometry::vector::Vector2F;
use crate::collection::{FontCollection, FontRef};
use crate::TextStyle;
use web_sys::console;

#[derive(Debug)]
pub struct TextLayout {
    pub size: f32,
    pub glyphs: Vec<Glyph>,
    pub advance: Vector2F,
    pub debug: Option<String>,
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
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
            debug: Some("NEW".to_string()),
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
        self.debug = other.debug.clone();
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
        debug: Some("MAKE_LAYOUT".to_string()),
    }
}

pub fn layout_run(style: &TextStyle, font: &FontRef, text: &str) -> TextLayout {
    // HB_THREAD_DATA.with(|hb_thread_data| {
    //     let mut hb_thread_data = hb_thread_data.borrow_mut();
    //     let mut b = Buffer::new();
    //     install_unicode_funcs(&mut b);
    //     b.add_str(text);
    //     b.set_direction(Direction::LTR);
    //     // TODO: set this based on detected script
    //     b.set_script(HB_SCRIPT_DEVANAGARI);
    //     b.set_language(Language::from_string("en_US"));
    //     let hb_face = hb_thread_data.create_hb_face_for_font(font);
    //     unsafe {
    //         let hb_font = hb_font_create(hb_face.hb_face);
    //         hb_shape(hb_font, b.as_ptr(), std::ptr::null(), 0);
    //         hb_font_destroy(hb_font);
    //         let mut n_glyph = 0;
    //         let glyph_infos = hb_buffer_get_glyph_infos(b.as_ptr(), &mut n_glyph);
    //         debug!("number of glyphs: {}", n_glyph);
    //         let glyph_infos = std::slice::from_raw_parts(glyph_infos, n_glyph as usize);
    //         let mut n_glyph_pos = 0;
    //         let glyph_positions = hb_buffer_get_glyph_positions(b.as_ptr(), &mut n_glyph_pos);
    //         let glyph_positions = std::slice::from_raw_parts(glyph_positions, n_glyph_pos as usize);
    //         let mut total_adv = Vector2F::zero();
    //         let mut glyphs = Vec::new();
    //         let scale = style.size / (font.font.metrics().units_per_em as f32);
    //         for (glyph, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
    //             debug!("{:?} {:?}", glyph.codepoint, (pos.x_offset, pos.y_offset));
    //             let adv = vec2i(pos.x_advance, pos.y_advance);
    //             let adv_f = adv.to_f32() * scale;
    //             let offset = vec2i(pos.x_offset, pos.y_offset).to_f32() * scale;
    //             let g = Glyph {
    //                 font: font.clone(),
    //                 glyph_id: glyph.codepoint,
    //                 offset: total_adv + offset,
    //             };
    //             total_adv += adv_f;
    //             glyphs.push(g);
    //         }
    //
    //         Layout {
    //             size: style.size,
    //             glyphs: glyphs,
    //             advance: total_adv,
    //         }
    //     }
    // })

    // Above is commented out because it uses HarfBuzz, which is not available in this project.
    // Instead, we will use RustyBuzz, which is a Rust implementation of HarfBuzz:

    let mut face = Face::from_slice(&font.font.font_data, 0).unwrap();
    // face.set_points_per_em(args.font_ptem);

        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(&text);

        buffer.set_direction(Direction::LeftToRight);

        buffer.set_language("en_US".parse().unwrap());
    
    

        // if let Some(script) = args.script {
        //     buffer.set_script(script);
        // }

        buffer.set_cluster_level(BufferClusterLevel::MonotoneGraphemes);
        buffer.reset_clusters();

        let features = [];

        let glyph_buffer = rustybuzz::shape(&face, &features, buffer);

        let mut format_flags = rustybuzz::SerializeFlags::default();
        // if args.no_glyph_names {
        //     format_flags |= rustybuzz::SerializeFlags::NO_GLYPH_NAMES;
        // }

        // if args.no_clusters || args.ned {
        //     format_flags |= rustybuzz::SerializeFlags::NO_CLUSTERS;
        // }

        // if args.no_positions {
        //     format_flags |= rustybuzz::SerializeFlags::NO_POSITIONS;
        // }

        // if args.no_advances || args.ned {
        //     format_flags |= rustybuzz::SerializeFlags::NO_ADVANCES;
        // }

        // if args.show_extents {
        //     format_flags |= rustybuzz::SerializeFlags::GLYPH_EXTENTS;
        // }

        // if args.show_flags {
        //     format_flags |= rustybuzz::SerializeFlags::GLYPH_FLAGS;
        // }

        // println!("{}", glyph_buffer.serialize(&face, format_flags))

    let mut glyphs: Vec<Glyph> = Vec::new();
    let mut posi = Vector2F::default();
    
    glyph_buffer.glyph_infos().iter().zip(glyph_buffer.glyph_positions()).for_each(|(info, pos)| {
        let adv = Vector2F::new(pos.x_advance as f32, pos.y_advance as f32);
        let offset = Vector2F::new(pos.x_offset as f32, pos.y_offset as f32);
        let glyph = Glyph {
            font: font.clone(),
            glyph_id: info.glyph_id,
            offset
        };
        glyphs.push(glyph);
        
        posi += adv;
    });

    TextLayout {
        size: style.size,
        glyphs,
        advance: posi,
        debug: Some( glyph_buffer.serialize(&face, format_flags)),
    }
}

pub fn layout(style: &TextStyle, collection: &FontCollection, text: &str) -> TextLayout {
    let mut result = TextLayout::new();
    result.debug = Option::from(text.to_string());
    
    for (range, font) in collection.itemize(text) {
        result.push_layout(&layout_run(style, font, &text[range]));
    }

    result
}
