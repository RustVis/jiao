// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::mem::size_of;

use crate::core::matrix::Matrix;
use crate::core::point::Vector;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

pub const SIZE_IN_MEMORY: usize = 12 * size_of::<Scalar>();

pub type Radii = [Vector; 4];

/// `RRect` describes a rounded rectangle with a bounds and a pair of radii for each corner.
///
/// The bounds and radii can be set so that `RRect` describes: a rectangle with sharp corners;
/// a circle; an oval; or a rectangle with one or more rounded corners.
///
/// `RRect` may have up to eight different radii, one for each axis on each of its four
/// corners.
///
/// `RRect` may modify the provided parameters when initializing bounds and radii.
/// If either axis radii is zero or less: radii are stored as zero; corner is square.
/// If corner curves overlap, radii are proportionally reduced to fit within bounds.
#[derive(Debug, Clone)]
pub struct RRect {
    rect: Rect,

    /// Radii order is UL, UR, LR, LL.
    ///
    /// Use Corner enum to index into radii[]
    radii: Radii,

    /// use an explicitly sized type so we're sure the class is dense (no uninitialized bytes)
    kind: Type,
}

impl Default for RRect {
    fn default() -> Self {
        Self::new()
    }
}

/// Type describes possible specializations of `RRect`.
///
/// Each Type is exclusive; a `RRect` may only have one type.
///
/// Type members become progressively less restrictive; larger values of
/// Type have more degrees of freedom than smaller values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type {
    /// zero width or height
    Empty,

    /// non-zero width and height, and zeroed radii
    Rect,

    /// non-zero width and height filled with radii
    Oval,

    /// non-zero width and height with equal radii
    Simple,

    /// non-zero width and height with axis-aligned radii
    NinePatch,

    /// non-zero width and height with arbitrary radii
    Complex,
}

impl Default for Type {
    fn default() -> Self {
        Self::Empty
    }
}

/// Corner type of `RRect`.
///
/// The radii are stored: top-left, top-right, bottom-right, bottom-left.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Corner {
    /// index of top-left corner radii
    UpperLeft,

    /// index of top-right corner radii
    UpperRight,

    /// index of bottom-right corner radii
    LowerRight,

    /// index of bottom-left corner radii
    LowerLeft,
}

impl RRect {
    /// Initializes bounds at (0, 0), the origin, with zero width and height.
    /// Initializes corner radii to (0, 0), and sets type of `Empty`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rect: Rect::new(),
            radii: [Vector::new(), Vector::new(), Vector::new(), Vector::new()],
            kind: Type::Empty,
        }
    }

    #[must_use]
    pub const fn get_type(&self) -> Type {
        debug_assert!(self.is_valid());
        self.kind
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kind == Type::Empty
    }

    #[must_use]
    pub fn is_rect(&self) -> bool {
        self.kind == Type::Rect
    }

    #[must_use]
    pub fn is_oval(&self) -> bool {
        self.kind == Type::Oval
    }

    #[must_use]
    pub fn is_simple(&self) -> bool {
        self.kind == Type::Simple
    }

    #[must_use]
    pub fn is_nine_patch(&self) -> bool {
        self.kind == Type::NinePatch
    }

    #[must_use]
    pub fn is_complex(&self) -> bool {
        self.kind == Type::Complex
    }

    /// Returns span on the x-axis.
    ///
    /// This does not check if result fits in 32-bit float; result may be infinity.
    #[must_use]
    pub fn width(&self) -> Scalar {
        self.rect.width()
    }

    /// Returns span on the y-axis.
    ///
    /// This does not check if result fits in 32-bit float; result may be infinity.
    #[must_use]
    pub fn height(&self) -> Scalar {
        self.rect.height()
    }

    /// Returns top-left corner radii.
    ///
    /// If `type()` returns Empty, Rect, Oval, or Simple, returns a value
    /// representative of all corner radii.
    /// If `type()` returns `NinePatch` or Complex, at least one of the
    /// remaining three corners has a different value.
    #[must_use]
    pub const fn get_simple_radii(&self) -> Vector {
        self.radii[0]
    }

    /// Sets bounds to zero width and height at (0, 0), the origin.
    ///
    /// Sets corner radii to zero and sets type to Empty.
    pub fn set_empty(&mut self) {
        *self = Self::new();
    }

    /// Sets bounds to sorted rect, and sets corner radii to zero.
    ///
    /// If set bounds has width and height, and sets type to Rect;
    /// otherwise, sets type to Empty.
    pub fn set_rect(&mut self, rect: &Rect) {
        if !self.initialize_rect(rect) {
            return;
        }

        //memset(fRadii, 0, sizeof(fRadii));
        self.radii.fill(Vector::new());
        self.kind = Type::Rect;

        debug_assert!(self.is_valid());
    }

    /// Initializes to copy of r bounds and zeroes corner radii.
    ///
    /// # Parameters
    /// - `r` - bounds to copy
    #[must_use]
    pub fn from_rect(r: &Rect) -> Self {
        let mut rr = Self::new();
        rr.set_rect(r);
        rr
    }

    /// Sets bounds to oval, x-axis radii to half `oval.width()`, and all y-axis radii
    /// to half `oval.height()`.
    ///
    /// If oval bounds is empty, sets to Empty. Otherwise, sets to Oval.
    ///
    /// # Parameters
    /// - `oval` - bounds of oval
    #[must_use]
    pub fn from_oval(oval: &Rect) -> Self {
        let mut rr = Self::new();
        rr.set_oval(oval);
        rr
    }

    /// Sets to rounded rectangle with the same radii for all four corners.
    ///
    /// If rect is empty, sets to Empty.
    /// Otherwise, if xRad and yRad are zero, sets to Rect.
    /// Otherwise, if xRad is at least half `rect.width()` and yRad is at least half
    /// `rect.height()`, sets to oval.
    /// Otherwise, sets to Simple.
    ///
    /// # Parameters
    /// - `rect` - bounds of rounded rectangle
    /// - `x_rad` - x-axis radius of corners
    /// - `y_rad` - y-axis radius of corners
    #[must_use]
    pub fn from_rect_xy(rect: &Rect, x_rad: Scalar, y_rad: Scalar) -> Self {
        let mut rr = Self::new();
        rr.set_rect_xy(rect, x_rad, y_rad);
        rr
    }

    /// Sets bounds to oval, x-axis radii to half `oval.width()`, and all y-axis radii
    /// to half `oval.height()`.
    ///
    /// If oval bounds is empty, sets to Empty. Otherwise, sets to Oval.
    pub fn set_oval(&mut self, _oval: &Rect) {
        unimplemented!()
    }

    /// Sets to rounded rectangle with the same radii for all four corners.
    ///
    /// If rect is empty, sets to Empty.
    /// Otherwise, if `x_rad` or yRad is zero, sets to Rect.
    /// Otherwise, if `x_rad` is at least half `rect.width()` and yRad is at least half
    /// `rect.height()`, sets to Oval.
    /// Otherwise, sets to Simple.
    ///
    /// # Parameters
    /// - `rect`  bounds of rounded rectangle
    /// - `x_rad` - x-axis radius of corners
    /// - `y_rad` - y-axis radius of corners
    pub fn set_rect_xy(&mut self, _rect: &Rect, _x_rad: Scalar, _y_rad: Scalar) {
        unimplemented!()
    }

    /// Sets bounds to rect.
    ///
    /// Sets radii to `(left_rad, top_rad), (right_rad, top_rad),
    /// (right_rad, bottom_rad), (left_rad, bottom_rad)`.
    ///
    /// - if rect is empty, sets to Empty.
    /// - if `left_rad` and `right_rad` are zero, sets to Rect.
    /// - if `top_rad` and `bottom_rad` are zero, sets to Rect.
    /// - if `left_rad` and `right_rad` are equal and at least half `rect.width()`,
    /// and `top_rad` and `bottom_rad` are equal at least half `rect.height()`, sets to Oval.
    /// - if `left_rad` and `right_rad` are equal, and `top_rad` and `bottom_rad` are equal,
    /// sets to Simple.
    /// - otherwise, sets to `NinePatch`.
    ///
    /// Nine patch refers to the nine parts defined by the radii: one center rectangle,
    /// four edge patches, and four corner patches.
    ///
    /// # Parameters
    /// - `rect` - bounds of rounded rectangle
    /// - `left_rad` - left-top and left-bottom x-axis radius
    /// - `top_rad` - left-top and right-top y-axis radius
    /// - `right_rad` - right-top and right-bottom x-axis radius
    /// - `bottom_rad` - left-bottom and right-bottom y-axis radius
    pub fn set_nine_patch(
        &mut self,
        _rect: &Rect,
        _left_rad: Scalar,
        _top_rad: Scalar,
        _right_rad: Scalar,
        _bottom_rad: Scalar,
    ) {
        unimplemented!()
    }

    /// Sets bounds to rect.
    ///
    /// Sets radii array for individual control of all for corners.
    ///
    /// If rect is empty, sets to Empty.
    /// Otherwise, if one of each corner radii are zero, sets to Rect.
    /// Otherwise, if all x-axis radii are equal and at least half `rect.width()`, and
    /// all y-axis radii are equal at least half `rect.height()`, sets to Oval.
    /// Otherwise, if all x-axis radii are equal, and all y-axis radii are equal,
    /// sets to Simple.
    /// Otherwise, sets to `NinePatch`.
    ///
    /// # Parameters
    /// - `rect` - bounds of rounded rectangle
    /// - `radii` - corner x-axis and y-axis radii
    pub fn set_rect_radii(&mut self, _rect: &Rect, _radii: &Radii) {
        unimplemented!()
    }

    /// Returns bounding box.
    ///
    /// Bounds may have zero width or zero height. Bounds right is greater than
    /// or equal to left; bounds bottom is greater than or equal to top.
    /// Result is identical to `get_bounds()`.
    #[must_use]
    pub const fn rect(&self) -> &Rect {
        &self.rect
    }

    /// Returns scalar pair for radius of curve on x-axis and y-axis for one corner.
    ///
    /// Both radii may be zero. If not zero, both are positive and finite.
    #[must_use]
    pub const fn radii(&self, corner: Corner) -> Vector {
        self.radii[corner as usize]
    }

    /// Returns bounding box.
    ///
    /// Bounds may have zero width or zero height. Bounds right is greater than
    /// or equal to left; bounds bottom is greater than or equal to top.
    /// Result is identical to `rect()`.
    #[must_use]
    pub const fn get_bounds(&self) -> &Rect {
        &self.rect
    }

    /// Copies `RRect` to dst, then insets dst bounds by dx and dy, and adjusts dst
    /// radii by dx and dy.
    ///
    /// dx and dy may be positive, negative, or zero. dst may be `RRect`.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged.
    /// Otherwise, if adjusted radius becomes negative, pins radius to zero.
    /// If dx exceeds half dst bounds width, dst bounds left and right are set to
    /// bounds x-axis center. If dy exceeds half dst bounds height, dst bounds top and
    /// bottom are set to bounds y-axis center.
    ///
    /// If dx or dy cause the bounds to become infinite, dst bounds is zeroed.
    ///
    /// # Parameters
    /// - `dx` - added to rect().left, and subtracted from rect().right
    /// - `dy` - added to rect().top, and subtracted from rect().bottom
    /// - `dst` - insets bounds and radii
    pub fn inset(&self, _dx: Scalar, _dy: Scalar, _dst: &mut Self) {
        unimplemented!()
    }

    /// Insets bounds by dx and dy, and adjusts radii by dx and dy. dx and dy may be
    /// positive, negative, or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged.
    /// Otherwise, if adjusted radius becomes negative, pins radius to zero.
    /// If dx exceeds half bounds width, bounds left and right are set to
    /// bounds x-axis center. If dy exceeds half bounds height, bounds top and
    /// bottom are set to bounds y-axis center.
    ///
    /// If dx or dy cause the bounds to become infinite, bounds is zeroed.
    ///
    /// # Parameters
    /// - `dx` - added to rect().left, and subtracted from rect().right
    /// - `dy` - added to rect().top, and subtracted from rect().bottom
    pub fn inset_in_place(&mut self, _dx: Scalar, _dy: Scalar) {
        unimplemented!()
        //this->inset(dx, dy, this);
    }

    /// Outsets dst bounds by dx and dy, and adjusts radii by dx and dy. dx and dy may be
    /// positive, negative, or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged.
    /// Otherwise, if adjusted radius becomes negative, pins radius to zero.
    /// If dx exceeds half dst bounds width, dst bounds left and right are set to
    /// bounds x-axis center. If dy exceeds half dst bounds height, dst bounds top and
    /// bottom are set to bounds y-axis center.
    ///
    /// If dx or dy cause the bounds to become infinite, dst bounds is zeroed.
    ///
    /// # Parameters
    /// - `dx` - subtracted from rect().left, and added to rect().right
    /// - `dy` - subtracted from rect().top, and added to rect().bottom
    /// - `dst` - outset bounds and radii
    pub fn outset(&mut self, dx: Scalar, dy: Scalar, dst: &mut Self) {
        self.inset(-dx, -dy, dst);
    }

    /// Outsets bounds by dx and dy, and adjusts radii by dx and dy. dx and dy may be
    /// positive, negative, or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged.
    /// Otherwise, if adjusted radius becomes negative, pins radius to zero.
    /// If dx exceeds half bounds width, bounds left and right are set to
    /// bounds x-axis center. If dy exceeds half bounds height, bounds top and
    /// bottom are set to bounds y-axis center.
    ///
    /// If dx or dy cause the bounds to become infinite, bounds is zeroed.
    ///
    /// # Parameters
    /// - `dx` - subtracted from rect().left, and added to rect().right
    /// - `dy` - subtracted from rect().top, and added to rect().bottom
    pub fn outset_in_place(&mut self, _dx: Scalar, _dy: Scalar) {
        unimplemented!()
        //this->inset(-dx, -dy, this);
    }

    /// Translates `RRect` by (dx, dy).
    ///
    /// # Parameters
    /// - `dx` - offset added to rect().left and rect().right
    /// - `dy` - offset added to rect().top and rect().bottom
    pub fn offset(&mut self, dx: Scalar, dy: Scalar) {
        self.rect.offset(dx, dy);
    }

    /// Returns `RRect` translated by (dx, dy).
    ///
    /// # Parameters
    /// - `dx` - offset added to rect().left and rect().right
    /// - `dy` - offset added to rect().top and rect().bottom
    ///
    /// Returns `RRect` bounds offset by (dx, dy), with unchanged corner radii
    #[must_use]
    pub fn from_offset(&self, dx: Scalar, dy: Scalar) -> Self {
        Self {
            rect: self.rect.from_offset(dx, dy),
            radii: self.radii,
            kind: self.kind,
        }
    }

    /// Returns true if rect is inside the bounds and corner radii, and if
    /// `RRect` and rect are not empty.
    ///
    /// # Parameters
    /// - `rect` - area tested for containment
    ///
    /// Returns true if `RRect` contains rect
    #[must_use]
    pub const fn contains(&self, _rect: &Rect) -> bool {
        unimplemented!()
    }

    /// Returns true if bounds and radii values are finite and describe a `RRect`
    /// type that matches `get_type()`.
    ///
    /// All `RRect` methods construct valid types, even if the input values are not valid.
    /// Invalid `RRect` data can only be generated by corrupting memory.
    ///
    /// Returns true if bounds and radii match `type()`
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        unimplemented!()
    }

    /// Writes `RRect` to buffer.
    ///
    /// Writes `SIZE_IN_MEMORY` bytes, and returns `SIZE_IN_MEMORY`,
    /// the number of bytes written.
    pub fn write_to_memory(&self, _buffer: &mut [u8]) -> usize {
        unimplemented!()
    }

    /// Reads `RRect` from buffer, reading `SIZE_IN_MEMORY` bytes.
    ///
    /// Returns `SIZE_IN_MEMORY`, bytes read if length is at least `SIZE_IN_MEMORY`.
    /// Otherwise, returns zero.
    ///
    /// # Parameters
    /// - `buffer` - memory to read from
    /// - `length` - size of buffer
    ///
    /// Returns bytes read, or 0 if length is less than `SIZE_IN_MEMORY`
    pub fn read_from_memory(&mut self, _buffer: &[u8]) -> usize {
        unimplemented!()
    }

    /// Transforms by `RRect` by matrix, storing result in dst.
    ///
    /// Returns true if `RRect` transformed can be represented by another `RRect`.
    /// Returns false if matrix contains transformations that are not axis aligned.
    ///
    /// # Parameters
    /// - `matrix` - Matrix specifying the transform
    /// - `dst` - `RRect` to store the result
    ///
    /// Returns true if transformation succeeded.
    pub fn transform(&self, _matrix: &Matrix, _dst: &mut Self) -> bool {
        unimplemented!()
    }

    /// Writes text representation of `RRect` to standard output.
    ///
    /// Set `as_hex` true to generate exact binary representations of floating point numbers.
    ///
    /// # Parameters
    /// - `as_hex` - true if Scalar values are written as hexadecimal
    pub fn dump(&self, _as_hex: bool) {
        unimplemented!()
    }

    #[must_use]
    pub fn dump_to_string(&self, _as_hex: bool) -> String {
        unimplemented!()
    }

    /// Writes text representation of `RRect` to standard output.
    ///
    /// Floating point values are written with limited precision;
    /// it may not be possible to reconstruct original `RRect` from output.
    pub fn dump_text(&self) {
        self.dump(false);
    }

    /// Writes text representation of `RRect` to standard output.
    ///
    /// Floating point values are written in hexadecimal to preserve their exact bit pattern.
    /// The output reconstructs the original `RRect`.
    pub fn dump_hex(&self) {
        self.dump(true);
    }

    #[must_use]
    fn are_rect_and_radii_valid(_rect: &Rect, _radii: &Radii) -> bool {
        unimplemented!()
    }

    /// Initializes rect.
    ///
    /// If the passed in rect is not finite or empty the rrect will be fully initialized
    /// and false is returned.
    /// Otherwise, just fRect is initialized and true is returned.
    fn initialize_rect(&mut self, rect: &Rect) -> bool {
        self.rect = rect.clone();
        unimplemented!()
    }

    fn compute_type(/*&mut self*/) {
        unimplemented!()
    }

    #[must_use]
    const fn check_corner_containment(_x: Scalar, _y: Scalar) -> bool {
        unimplemented!()
    }

    /// Returns true if the radii had to be scaled to fit rect
    #[must_use]
    fn scale_radii(/*&mut self*/) -> bool {
        unimplemented!()
    }
}

impl PartialEq<Self> for RRect {
    /// Returns true if bounds and radii in self are equal to bounds and radii in other.
    ///
    /// self and other are not equal if either contain NaN. a and b are equal if members
    /// contain zeroes with different signs.
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect && self.radii == other.radii
    }
}
