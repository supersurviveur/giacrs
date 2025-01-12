//! A giac `Context` to keep variables

use std::sync::LazyLock;

use crate::{
    ffi::{self, giacrs_init_global_context},
    gen::Gen,
    support::GiacError,
};

pub(crate) enum GiacContext {}
pub(crate) type GiacContextRef = *const GiacContext;

/// A giac context used to manage multiple sessions, with differents variables.
pub struct Context(GiacContextRef);

unsafe impl Sync for Context {}
unsafe impl Send for Context {}

/// Pointer to the global giac context
#[allow(unused)]
pub static GLOBAL_CONTEXT: LazyLock<Context> = LazyLock::new(|| {
    unsafe { giacrs_init_global_context() };
    Context(unsafe { ffi::giacrs_global_context })
});

impl Context {
    /// Internal giac pointer
    pub(crate) unsafe fn as_context_ref(&self) -> GiacContextRef {
        self.0
    }

    /// Init a new giac context.
    ///
    /// Each context has it owns variables, it's usefull to manage multiple sessions at the same time, like in XCAS.
    pub fn new() -> Self {
        Self(unsafe { ffi::giacrs_new_context() })
    }

    /// Evaluates a string to an expression in the current context.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    ///
    /// // Compute the determinant of a 2x2 matrix
    /// assert_eq!(-2, ctx.eval("det([[1,2],[3,4]])")?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn eval(&self, str: &str) -> Result<Gen, GiacError> {
        Gen::from_str(str, &self)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::giacrs_free_context(self.as_context_ref()) };
    }
}

/// This function release all globals variables defined by giac to avoid some memory leaks. This must be called at the end of your program, if you want to make valgrind happy.
pub fn release_globals() {
    unsafe { ffi::giacrs_release_globals() };
}
