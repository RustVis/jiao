// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use super::position::Position;

/// Class which is notified in the event of an error.
pub struct ErrorReporter {
    source: String,
    error_count: i32,
    error_handler: Box<dyn ErrorHandler>,
}

impl ErrorReporter {
    #[must_use]
    pub fn new(error_handler: Box<dyn ErrorHandler>) -> Self {
        Self {
            source: String::new(),
            error_count: 0,
            error_handler,
        }
    }

    pub fn error(&mut self, position: Position, msg: &str) {
        //if (skstd::contains(msg, Compiler::POISON_TAG)) {
        // Don't report errors on poison values.
        //    return;
        //}
        self.error_count += 1;
        self.handle_error(msg, position);
    }

    #[must_use]
    #[inline]
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = source.to_owned();
    }

    #[must_use]
    #[inline]
    pub const fn error_count(&self) -> i32 {
        self.error_count
    }

    pub fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

    fn position(&mut self, _offset: i32) -> Position {
        todo!()
    }

    /// Called when an error is reported.
    #[inline]
    fn handle_error(&mut self, msg: &str, position: Position) {
        self.error_handler.handle_error(msg, position);
    }
}

pub trait ErrorHandler {
    fn handle_error(&mut self, msg: &str, position: Position);
}

/// Error reporter for tests that need an `SkSL` context; aborts immediately if an error is reported.
pub struct TestingOnlyAbortErrorReporter {}

impl ErrorHandler for TestingOnlyAbortErrorReporter {
    fn handle_error(&mut self, msg: &str, position: Position) {
        panic!("pos: {position:?}, {msg}");
    }
}
