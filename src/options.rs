//! Edit giac contexts options

use crate::{context::Context, ffi};

impl Context {
    /// Change epsilon value, used for choosing a precision in computations.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let mut ctx = Context::new();
    /// let a = Gen::from(12.9642857143);
    ///
    /// ctx.set_epsilon(1e-6);
    /// assert_eq!("363/28", a.float_to_rational(&ctx)?.to_string());
    ///
    /// ctx.set_epsilon(1e-15);
    /// assert_eq!("32433139246/2501729749", a.float_to_rational(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn set_epsilon(&mut self, epsilon: f64) {
        unsafe { ffi::giacrs_options_set_epsilon(epsilon, self.as_context_ref()) };
    }
}
