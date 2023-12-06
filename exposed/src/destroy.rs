use std::ops::{Deref, DerefMut};

use exposed_macro::log_error;

/// Used for cleanup function that might return an error.
/// To achieve best out of it use implementing type with trait `Destroyable`.
pub trait Destroy {
    fn destroy(&mut self) -> Result<(), std::io::Error>;
}

/// Calls `Destroy::destroy` for `T` when it is dropped.
pub struct Destroyable<T: Destroy>(pub T);

impl<T: Destroy> Destroyable<T> {
    /// Returns inner object while consuming itself.
    pub unsafe fn into_inner(self) -> T {
        let inner: T = std::mem::transmute_copy(&self.0);
        std::mem::forget(self);
        inner
    }
}

impl<T: Destroy> Destroyable<T> {
    pub fn new(x: T) -> Destroyable<T> {
        Destroyable(x)
    }
}

impl<T: Destroy> Deref for Destroyable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Destroy> DerefMut for Destroyable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Destroy> Drop for Destroyable<T> {
    fn drop(&mut self) {
        if let Err(e) = self.0.destroy() {
            log_error!("Destroy", "{e}")
        }
    }
}
