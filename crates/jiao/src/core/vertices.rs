// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use bitflags::bitflags;
use std::mem::size_of;

use crate::core::color::Color;
use crate::core::point::Point;
use crate::core::rect::Rect;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum VertexMode {
    Triangles,
    TriangleStrip,
    TriangleFan,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Desc {
    pub mode: VertexMode,
    pub vertex_count: i32,
    pub index_count: i32,
    pub has_tex_coords: bool,
    pub has_colors: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Sizes {
    /// size of entire Vertices allocation (obj + arrays)
    pub total: usize,
    /// size of all the data arrays (V + D + T + C + I)
    pub arrays: usize,
    pub vex_size: usize,
    pub texs_size: usize,
    pub color_size: usize,
    pub indices_size: usize,

    /// For indexed tri-fans this is the number of amount of space fo indices
    /// needed in the builder before conversion to indexed triangles
    /// (or zero if not indexed or not a triangle fan).
    pub builder_tri_fan_indices_size: usize,
}

impl Sizes {
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_lossless)]
    pub fn from_desc(desc: &Desc) -> Option<Self> {
        // TODO(Shaohua): Check overflow.
        let vex_size: usize = desc.vertex_count as usize * size_of::<Point>();
        let texs_size: usize = if desc.has_tex_coords {
            desc.vertex_count as usize * size_of::<Point>()
        } else {
            0
        };
        let color_size: usize = if desc.has_colors {
            desc.vertex_count as usize * size_of::<Color>()
        } else {
            0
        };

        let mut builder_tri_fan_indices_size: usize = 0;
        let mut indices_size: usize = desc.index_count as usize * size_of::<u16>();
        if desc.mode == VertexMode::TriangleFan {
            let num_fan_tris;
            if desc.index_count > 0 {
                builder_tri_fan_indices_size = indices_size;
                num_fan_tris = desc.index_count - 2;
            } else {
                num_fan_tris = desc.vertex_count - 2;
                // By forcing this to become indexed we are adding a constraint to the maximum
                // number of vertices.
                if desc.vertex_count > (u16::MAX as i32 + 1) {
                    return None;
                }
            }
            if num_fan_tris <= 0 {
                return None;
            }

            indices_size = num_fan_tris as usize * (3 * size_of::<u16>());
        }

        let total = size_of::<Vertices>() + vex_size + texs_size + color_size + indices_size;

        // just the sum of the arrays
        let arrays = vex_size + texs_size + color_size + indices_size;

        Some(Self {
            total,
            arrays,
            vex_size,
            texs_size,
            color_size,
            indices_size,
            builder_tri_fan_indices_size,
        })
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.total != 0
    }
}

/// An immutable set of vertex data that can be used with `Canvas::draw_vertices()`.
pub struct Vertices {
    unique_id: u32,

    // these point inside our allocation, so none of these can be "freed"
    positions: Vec<Point>, // vertex_count
    indices: Vec<u16>,     // index_count or empty
    texs: Vec<Point>,      // vertex_count or empty
    colors: Vec<Color>,    // vertex_count or empty

    // computed to be the union of the positions[]
    bounds: Rect,

    vertex_count: i32,
    index_count: i32,

    mode: VertexMode,
}

impl Vertices {
    #[must_use]
    fn new() -> Self {
        unimplemented!()
    }

    /// Create a vertices by copying the specified arrays.
    ///
    /// texs, colors may be empty.
    #[must_use]
    pub fn from(
        _mode: VertexMode,
        _vertex_count: i32,
        _points: &[Point],
        _texs: &[Point],
        _colors: &[Color],
        _indices: &[u16],
    ) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn unique_id(&self) -> u32 {
        self.unique_id
    }

    #[must_use]
    pub const fn bounds(&self) -> &Rect {
        &self.bounds
    }

    /// returns approximate byte size of the vertices object
    #[must_use]
    pub const fn approximate_size(&self) -> usize {
        unimplemented!()
    }

    #[must_use]
    pub(crate) const fn get_sizes() -> Sizes {
        unimplemented!()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.vertex_count == 0
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct BuilderFlags : u8 {
        const HasTexCoords = 1 << 0;
        const HasColors = 1 << 1;
    }
}

pub struct Builder {
    /// holds a partially complete object. only completed in detach()
    vertices: Vertices,

    /// Extra storage for intermediate vertices in the case where the client specifies indexed
    /// triangle fans.
    ///
    /// These get converted to indexed triangles when the Builder is finalized.
    intermediate_fan_indices: Vec<u8>,
}

impl Builder {
    #[must_use]
    pub fn new(
        _mode: VertexMode,
        _vertex_count: i32,
        _index_count: i32,
        _flags: BuilderFlags,
    ) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.vertices.is_empty()
    }

    pub fn positions(&mut self) -> &mut [Point] {
        unimplemented!()
    }

    /// Returns empty slice if there are no indices
    pub fn indices(&mut self) -> &mut [u16] {
        unimplemented!()
    }

    /// Returns empty slice if there are no `tex_coords`
    ///
    /// If we have custom attributes, it will always be empty.
    pub fn tex_coords(&mut self) -> &mut [Point] {
        unimplemented!()
    }

    /// Returns empty slice if there are no colors.
    ///
    /// If we have custom attributes, it will always be empty.
    pub fn colors(&mut self) -> &mut [Color] {
        unimplemented!()
    }

    /// Detach the built vertices object.
    ///
    /// After the first call, this will always return null.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn finish(self) -> Vertices {
        self.vertices
    }

    pub(crate) fn from_desc(desc: &Desc) -> Self {
        let mut flags = BuilderFlags::empty();
        if desc.has_tex_coords {
            flags |= BuilderFlags::HasTexCoords;
        }
        if desc.has_colors {
            flags |= BuilderFlags::HasColors;
        }
        Self::new(desc.mode, desc.vertex_count, desc.index_count, flags)
    }
}
