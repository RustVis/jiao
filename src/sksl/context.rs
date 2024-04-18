// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// Contains compiler-wide objects and state.
pub struct Context {
    /// The Context holds a reference to all of the built-in types.
    types: BuiltinTypes,

    /// The Context holds a pointer to the configuration of the program being compiled.
    config: Option<ProgramConfig>,

    /// The Context holds a pointer to our error reporter.
    errors: Box<ErrorReporter>,

    /// The Context holds a pointer to our module with built-in declarations.
    module: Option<Module>,

    /// This is the current symbol table of the code we are processing, and therefore changes during compilation.
    symbol_table: Option<SymbolTable>,
}

impl Context {
    pub fn new(types: BuiltinTypes, errors: Box<ErrorReporter>) -> Self {
        Self {
            types,
            errors,
            config: None,
            module: None,
            symbol_table: None,
        }
    }

    /*
    void setErrorReporter(ErrorReporter* e) {
        SkASSERT(e);
        fErrors = e;
    }
    */
}
