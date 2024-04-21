// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::matrix::Matrix;
use crate::core::paint::Paint;
use crate::core::path::Path;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

/// Returns the filled equivalent of the stroked path.
///
/// # Parameters
/// - `src` - Path read to create a filled version
/// - `paint` - Paint, from which attributes such as stroke cap, width, miter, and join,
///             as well as path effect will be used.
/// - `dst` - resulting Path; may be the same as src
/// - `cull_rect` - optional limit passed to `PathEffect`
/// - `res_scale` - if > 1.0, increase precision, else if (`0.0 < res_scale < 1.0`)
///                 reduce precision to favor speed and size
///
/// Returns true if the dst path was updated, false if it was not
/// (e.g. if the path represents hairline and cannot be filled).
pub fn fill_path_with_paint_and_scale(
    _src: &Path,
    _paint: &Paint,
    _dst: &mut Path,
    _cull_rect: &Rect,
    _res_scale: Scalar,
) {
    unimplemented!()
}

pub fn fill_path_with_paint_and_cull_rect(
    _src: &Path,
    _paint: &Paint,
    _dst: &mut Path,
    _cull_rect: &Rect,
    _ctm: &Matrix,
) {
    unimplemented!()
}

pub fn fill_path_with_paint(_src: &Path, _paint: &Paint, _dst: &mut Path) {
    unimplemented!()
}

/*
fn fill_path_with_paint_impl(src: &Path, paint: &Paint, dst: &mut Path,
                       cull_rect: &mut Rect, ctm: &Matrix) -> bool {
    if !src.is_finite() {
        dst.reset();
        return false;
    }

    let res_scale = ctm.compute_res_scale_for_stroking();
    StrokeRec rec(paint, res_scale);

#if defined(SK_BUILD_FOR_FUZZER)
    // Prevent lines with small widths from timing out.
    if (rec.getStyle() == SkStrokeRec::Style::kStroke_Style && rec.getWidth() < 0.001) {
        return false;
    }
#endif

    const SkPath* srcPtr = &src;
    SkPath tmpPath;

    SkPathEffect* pe = paint.getPathEffect();
    if (pe && pe->filterPath(&tmpPath, src, &rec, cullRect, ctm)) {
        srcPtr = &tmpPath;
    }

    if (!rec.applyToPath(dst, *srcPtr)) {
        if (srcPtr == &tmpPath) {
            // If path's were copy-on-write, this trick would not be needed.
            // As it is, we want to save making a deep-copy from tmpPath -> dst
            // since we know we're just going to delete tmpPath when we return,
            // so the swap saves that copy.
            dst->swap(tmpPath);
        } else {
            *dst = *srcPtr;
        }
    }

    if (!dst->isFinite()) {
        dst->reset();
        return false;
    }
    return !rec.isHairlineStyle();
}
*/
