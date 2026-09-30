use std::{cell::RefCell, fmt::Display, thread::panicking};

use quote::ToTokens;
use syn::{Error, Result};

pub const CHECKED: &str = "already checked for errors";
pub const FORGOT: &str = "forgot to check for errors";

pub struct Context {
    errors: RefCell<Option<Vec<Error>>>,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn checked(&self) -> bool {
        self.errors.borrow().is_none()
    }

    pub fn error_spanned_by<T: ToTokens, M: Display>(&self, tokens: T, message: M) {
        self.error(Error::new_spanned(tokens, message));
    }

    pub fn error(&self, error: Error) {
        self.errors
            .borrow_mut()
            .as_mut()
            .expect(CHECKED)
            .push(error);
    }

    pub fn check_error(self) -> Option<Error> {
        self.errors
            .borrow_mut()
            .take()
            .expect(CHECKED)
            .into_iter()
            .reduce(|mut combined, error| {
                combined.combine(error);
                combined
            })
    }

    pub fn check(self) -> Result<()> {
        self.check_error().map_or(Ok(()), Err)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        assert!(panicking() || self.checked(), "{FORGOT}");
    }
}
