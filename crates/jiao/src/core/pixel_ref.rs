// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::base::id_change_listener::{IdChangeListener, IdChangeListenerList};
use crate::core::image_info::ImageInfo;
use crate::core::size::ISize;

/// `PixelRef` is the smart container for pixel memory, and is used with Bitmap.
///
/// This class can be shared/accessed between multiple threads.
pub struct PixelRef {
    width: i32,
    height: i32,
    row_bytes: usize,
    pixels: Vec<u8>,
    mutability: Mutability,

    tagged_gen_id: AtomicU32,
    gen_id_change_listeners: IdChangeListenerList,

    /// Set true by caches when they cache content that's derived from the current pixels.
    added_to_cache: AtomicBool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Mutability {
    /// PixelRefs begin mutable.
    Mutable,

    /// Considered immutable, but can revert to mutable.
    TemporarilyImmutable,

    /// Once set to this state, it never leaves.
    Immutable,
}

impl PixelRef {
    #[must_use]
    pub fn new(width: i32, height: i32, row_bytes: usize, pixels: &[u8]) -> Self {
        //unimplemented!();
        Self {
            width,
            height,
            row_bytes,
            pixels: pixels.to_vec(),
            mutability: Mutability::Mutable,

            tagged_gen_id: AtomicU32::new(0),
            gen_id_change_listeners: IdChangeListenerList::new(),
            added_to_cache: AtomicBool::new(false),
        }
    }

    /// Return a new `PixelRef`, automatically allocating storage for the pixels.
    ///
    /// - If `row_bytes` are 0, an optimal value will be chosen automatically.
    /// - If `row_bytes` is > 0, then it will be respected.
    /// - None will be returned if `row_bytes` is invalid for the specified info.
    ///
    /// All pixel bytes are zeroed.
    ///
    /// Returns None on failure.
    pub fn with_image_info(_info: &ImageInfo, _row_bytes: usize) -> Option<Self> {
        unimplemented!()
    }

    /// Return a new `PixelRef` that will use the provided Data and `row_bytes` as pixel storage.
    ///
    /// The Data will be refed and on destruction of the `PixelRef`, the Data will be unrefed.
    ///
    /// Returns None on failure.
    pub fn with_data(_info: &ImageInfo, _row_bytes: usize /*_data: &Data*/) -> Option<Self> {
        unimplemented!()
    }

    #[must_use]
    pub const fn dimensions(&self) -> ISize {
        ISize::from_wh(self.width, self.height)
    }

    #[must_use]
    pub const fn width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.height
    }

    #[must_use]
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    #[must_use]
    pub const fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    /// Returns a non-zero, unique value corresponding to the pixels in this pixelref.
    ///
    /// Each time the pixels are changed (and `notify_pixels_changed()` is called),
    /// a different generation ID will be returned.
    #[must_use]
    pub fn get_generation_id(&self) -> u32 {
        //unimplemented!();
        self.tagged_gen_id.load(Ordering::Relaxed)
    }

    /// Call this if you have changed the contents of the pixels.
    ///
    /// This will inturn cause a different generation ID value to be returned from
    /// `get_generation_id()`.
    pub fn notify_pixels_changed(&mut self) {
        unimplemented!()
    }

    /// Returns true if this pixelref is marked as immutable, meaning that the
    /// contents of its pixels will not change for the lifetime of the pixelref.
    #[must_use]
    pub fn is_immutable(&self) -> bool {
        self.mutability == Mutability::Immutable
    }

    /// Marks this pixelref is immutable, meaning that the contents of its
    /// pixels will not change for the lifetime of the pixelref.
    ///
    /// This state can be set on a pixelref, but it cannot be cleared once it is set.
    pub fn set_immutable(&mut self) {
        self.mutability = Mutability::Immutable;
        unimplemented!()
    }

    /// Register a listener that may be called the next time our generation ID changes.
    ///
    /// We'll only call the listener if we're confident that we are the only PixelRef with this
    /// generation ID.
    /// If our generation ID changes and we decide not to call the listener, we'll
    /// never call it: you must add a new listener for each generation ID change.
    /// We also won't call the listener when we're certain no one knows what our generation ID is.
    ///
    /// This can be used to invalidate caches keyed by PixelRef generation ID.
    /// Takes ownership of listener.
    /// Threadsafe.
    pub fn add_gen_id_change_listener(&mut self, listener: IdChangeListener) {
        self.gen_id_change_listeners.push(listener);
    }

    /// Call when this pixelref is part of the key to a resourcecache entry.
    ///
    /// This allows the cache to know automatically those entries can be purged
    /// when this pixelref is changed or deleted.
    pub fn notify_added_to_cache(&mut self) {
        self.added_to_cache.store(true, Ordering::Relaxed);
    }

    // Bottom bit indicates the Gen ID is unique.
    fn gen_id_is_unique(&self) -> bool {
        self.tagged_gen_id.load(Ordering::Relaxed) & 1 == 1
    }

    fn needs_new_gen_id(&mut self) {
        unimplemented!()
    }

    fn call_gen_id_change_listeners(&mut self) {
        unimplemented!()
    }

    fn set_temporarily_immutable(&mut self) {
        self.mutability = Mutability::TemporarilyImmutable;
        unimplemented!()
    }

    fn restore_mutability(&mut self) {
        self.mutability = Mutability::Mutable;
        unimplemented!()
    }

    fn set_immutable_with_id(&mut self, _gen_id: u32) {
        self.mutability = Mutability::Immutable;
        unimplemented!()
    }

    //friend void BitmapCache_setImmutableWithID(SkPixelRef*, uint32_t);
}
