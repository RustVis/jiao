// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::{PainterTrait, PathTrait};

use crate::platforms::Path;
use crate::ShapeTrait;

#[derive(Debug, Clone)]
pub struct PolygonShape {
    points: Vec<PointF>,
    is_closed: bool,

    path: Path,
    path_is_dirty: bool,
}

impl Default for PolygonShape {
    fn default() -> Self {
        Self::new()
    }
}

impl PolygonShape {
    #[must_use]
    pub fn new() -> Self {
        Self::from_points(&[], true)
    }

    #[must_use]
    pub fn from_points(points: &[PointF], is_closed: bool) -> Self {
        let points = points.to_vec();
        let path = Path::new();
        Self {
            points,
            path,
            is_closed,
            path_is_dirty: true,
        }
    }

    #[must_use]
    pub fn points(&self) -> &[PointF] {
        &self.points
    }

    #[must_use]
    pub fn points_mut(&mut self) -> &mut Vec<PointF> {
        self.path_is_dirty = true;
        &mut self.points
    }

    pub fn add_point(&mut self, point: PointF) {
        self.points.push(point);
        self.path_is_dirty = true;
    }

    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn set_closed(&mut self, is_closed: bool) {
        self.is_closed = is_closed;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();

        if self.points.len() > 1 {
            let first_point = self.points[0];
            self.path.move_to(first_point);
            for point in &self.points[1..] {
                self.path.line_to(*point);
            }

            if self.is_closed {
                self.path.close_path();
            }
        }

        self.path_is_dirty = true;
    }
}

impl ShapeTrait for PolygonShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
