// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::sksl::util::ShaderCaps;
use crate::sksl::version::Version;

#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    sksl_version: Version,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            sksl_version: Version::V100,
        }
    }
}

impl Capabilities {
    #[must_use]
    #[inline]
    pub fn raster_backend() -> Self {
        Self::default()
    }

    #[must_use]
    #[inline]
    pub fn sksl_version(&self) -> Version {
        self.sksl_version
    }

    fn init_caps(&mut self, shader_caps: &ShaderCaps) {
        self.sksl_version = shader_caps.supported_sksl_verion();
    }
}
