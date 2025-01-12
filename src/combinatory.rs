//! Combinatory analysis
//! <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node55.html> 

use giacrs_internals::ffi_safe_call;

use crate::{context::Context, ffi, gen::Gen, GiacError};

impl Gen {
    /// Computes the Pascal binom
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(5);
    ///
    /// assert_eq!(10, a.binomial(&2.into(), &ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node57.html>
    pub fn binomial(&self, k: &Gen, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! {ffi::giacrs_gen_comb(self.as_gen_ref(), k.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Computes arrangements
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(5);
    ///
    /// assert_eq!(20, a.permutation(&2.into(), &ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node58.html>
    pub fn permutation(&self, k: &Gen, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! {ffi::giacrs_gen_perm(self.as_gen_ref(), k.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Returns a random integer p such that `0 <= p < n`
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(10);
    /// let rand = a.rand(&ctx)?.to_int()?;
    ///
    /// assert!(0 <= rand && rand < a.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node59.html>
    pub fn rand(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! {ffi::giacrs_gen_rand(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }
}
