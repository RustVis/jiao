// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::font_style::FontStyle;

pub trait FontStyleSet {
    fn count(&self) -> i32;
    fn get_style(&self, index: i32, style: &mut FontStyle, style_str: &mut String);
    fn create_type_face(&self, index: i32) -> Typeface;
    fn match_style(&self, pattern: &FontStyle) -> Option<Typeface>;
}

pub(crate) fn match_style_css3(_pattern: &FontStyle) -> Option<Typeface> {
    unimplemented!()
}

pub struct FontManager {}

impl FontManager {
    #[must_use]
    pub const fn count_families(&self) -> i32 {
        unimplemented!()
    }

    #[must_use]
    pub fn get_family_name(&self, _index: i32) -> String {
        unimplemented!()
    }

    #[must_use]
    pub fn create_style_sheet(&self, _index: i32) -> Box<dyn FontStyleSet> {
        unimplemented!()
    }

    /// The caller must call unref() on the returned object.
    ///
    /// Never returns NULL; will return an empty set if the name is not found.
    /// Passing nullptr as the parameter will return the default system family.
    ///
    /// Note that most systems don't have a default system family, so passing nullptr will often
    /// result in the empty set.
    ///
    /// It is possible that this will return a style set not accessible from
    /// `create_style_set(int)` due to hidden or auto-activated fonts.
    #[must_use]
    pub fn match_family(&self, _family_name: &str) -> Box<dyn FontStyleSet> {
        unimplemented!()
    }

    /// Find the closest matching typeface to the specified familyName and style
    /// and return a ref to it.
    ///
    /// The caller must call unref() on the returned object.
    /// Will return nullptr if no 'good' match is found.
    ///
    /// Passing |nullptr| as the parameter for `family_name` will return the
    /// default system font.
    ///
    /// It is possible that this will return a style set not accessible from
    /// `create_style_set(int)` or matchFamily(const char[]) due to hidden or
    /// auto-activated fonts.
    #[must_use]
    pub fn match_family_style(&self, _family_name: &str, _style: &FontStyle) -> Option<Typeface> {
        unimplemented!()
    }

    /// Use the system fallback to find a typeface for the given character.
    ///
    /// Note that bcp47 is a combination of ISO 639, 15924, and 3166-1 codes,
    /// so it is fine to just pass a ISO 639 here.
    ///
    /// Will return NULL if no family can be found for the character
    /// in the system fallback.
    ///
    /// Passing |nullptr| as the parameter for |familyName| will return the
    /// default system font.
    ///
    /// bcp47[0] is the least significant fallback, bcp47[bcp47Count-1] is the
    /// most significant. If no specified bcp47 codes match, any font with the
    /// requested character will be matched.
    #[must_use]
    pub fn match_family_style_character(
        &self,
        _family_name: &str,
        _style: &FontStyle,
        _bcp47: &str,
        _character: char,
    ) -> Option<Typeface> {
        unimplemented!()
    }
}
