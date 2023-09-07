// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::rect::Rect;

// TODO(Shaohua): move everything below into the PathBuilder
/// The logical operations that can be performed when combining two paths.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PathOp {
    /// Subtract the op path from the first path
    Difference,

    /// Intersect the two paths
    Intersect,

    /// Union (inclusive-or) the two paths
    Union,

    /// exclusive-or the two paths
    Xor,

    /// Subtract the first path from the op path
    ReverseDifference,
}

/// Set this path to the result of applying the Op to this path and the
/// specified path: this = (this op operand).
///
/// The resulting path will be constructed from non-overlapping contours.
/// The curve order is reduced where possible so that cubics may be turned
/// into quadratics, and quadratics maybe turned into lines.
///
/// # Parameters
/// Returns true if operation was able to produce a result;
/// otherwise, result is unmodified.
///
/// # Parameters
/// - `one` - The first operand (for difference, the minuend)
/// - `two` - The second operand (for difference, the subtrahend)
/// - `op` - The operator to apply.
/// - `result` - The product of the operands. The result may be one of the inputs.
///
/// Returns true if the operation succeeded.
pub fn op(_one: &Path, _two: &Path, _op: PathOp, _result: &mut Path) -> bool {
    unimplemented!()
}

/// Set this path to a set of non-overlapping contours that describe the
/// same area as the original path.
///
/// The curve order is reduced where possible so that cubics may
/// be turned into quadratics, and quadratics maybe turned into lines.
///
/// Returns true if operation was able to produce a result;
/// otherwise, result is unmodified.
///
/// # Parameters
/// - `path` - The path to simplify.
/// - `result` The simplified path. The result may be the input.
///
/// Returns true if simplification succeeded.
pub fn simplify(_path: &Path, _result: &mut Path) -> bool {
    unimplemented!()
}

/// Set the resulting rectangle to the tight bounds of the path.
///
/// # Parameters
/// - `path` - The path measured.
/// - `result` - The tight bounds of the path.
///
/// Returns true if the bounds could be computed.
pub fn tight_bounds(_path: &Path, _result: &mut Rect) -> bool {
    unimplemented!()
}

/// Set the result with fill type winding to area equivalent to path.
///
/// Returns true if successful. Does not detect if path contains contours which
/// contain self-crossings or cross other contours; in these cases, may return
/// true even though result does not fill same area as path.
///
/// Returns true if operation was able to produce a result;
/// otherwise, result is unmodified. The result may be the input.
///
/// # Parameters
/// - `path` - The path typically with fill type set to even odd.
/// - `result` - The equivalent path with fill type set to winding.
///
/// Returns true if winding path was set.
pub fn as_winding(_path: &Path, _result: &mut Path) -> bool {
    unimplemented!()
}

/// Perform a series of path operations, optimized for unioning many paths together.
pub struct OpBuilder {
    // TODO(Shaohua): Replace Path with PathRef.
    path_refs: Vec<Path>,
    ops: Vec<PathOp>,
}

impl OpBuilder {
    /// Add one or more paths and their operand.
    ///
    /// The builder is empty before the first path is added, so the result
    /// of a single add is (`empty_path` OP path).
    ///
    /// # Parameters
    /// - `path` - The second operand.
    /// - `operator` - The operator to apply to the existing and supplied paths.
    pub fn add(&mut self, path: &Path, operator: PathOp) {
        self.path_refs.push(path.clone());
        self.ops.push(operator);
        unimplemented!()
    }

    /// Computes the sum of all paths and operands, and resets the builder to its
    /// initial state.
    ///
    /// # Parameters
    /// - `result` - The product of the operands.
    ///
    /// Returns true if the operation succeeded.
    pub fn resolve(&mut self, _result: &mut Path) -> bool {
        unimplemented!()
    }

    fn fix_winding(_path: &mut Path) -> bool {
        unimplemented!()
    }

    fn reverse_path(_path: &mut Path) {
        unimplemented!()
    }

    fn reset(&mut self) {
        self.path_refs.clear();
        self.ops.clear();
        unimplemented!()
    }
}
