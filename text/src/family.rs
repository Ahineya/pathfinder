// font-kit/src/family.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Defines a set of faces that vary in weight, width or slope.

use crate::collection::Font;
use crate::error::FontLoadingError;
use crate::family_handle::FamilyHandle;
use crate::handle::Handle;

/// Defines a set of faces that vary in weight, width or slope.
// #[derive(Debug)]
pub struct Family
{
    fonts: Vec<Font>,
}

impl Family
{
    pub(crate) fn from_font_handles<'a, I>(font_handles: I) -> Result<Family, FontLoadingError>
    where
        I: Iterator<Item = &'a Handle>,
    {
        let mut fonts = vec![];
        for font_handle in font_handles {
            fonts.push(Font::from_handle(font_handle)?)
        }
        Ok(Family { fonts })
    }

    #[inline]
    pub(crate) fn from_handle(family_handle: &FamilyHandle) -> Result<Family, FontLoadingError> {
        Family::from_font_handles(family_handle.fonts.iter())
    }

    /// Returns the individual fonts in this family.
    #[inline]
    pub fn fonts(&self) -> &[Font] {
        &self.fonts
    }

    /// Returns true if and only if this family is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }
}
