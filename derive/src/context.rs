use std::{cell::RefCell, fmt::Display, thread::panicking};

use quote::ToTokens;
use syn::{Error, Result};

pub struct Context {
    errors: RefCell<Option<Vec<Error>>>,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.borrow().is_some()
    }

    pub fn error_spanned_by<I: ToTokens, T: Display>(&self, item: I, message: T) {
        self.error(Error::new_spanned(item, message));
    }

    pub fn error(&self, error: Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(error);
    }

    pub fn check(self) -> Result<()> {
        let mut errors = self.errors.borrow_mut().take().unwrap().into_iter();

        let Some(mut combined) = errors.next() else {
            return Ok(());
        };

        for error in errors {
            combined.combine(error);
        }

        Err(combined)
    }
}

pub const FORGOT: &str = "forgot to check for errors";

impl Drop for Context {
    fn drop(&mut self) {
        #[allow(clippy::manual_assert)]
        if !panicking() && self.has_errors() {
            panic!("{FORGOT}");
        }
    }
}
