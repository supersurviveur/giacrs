//! Integers and gaussians integers
//! <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node27.html>

use giacrs_internals::ffi_safe_call;

use crate::{context::Context, ffi, gen::Gen, GiacError};

/// Return value of the [Gen::is_pseudoprime] method
#[derive(Debug, PartialEq, Eq)]
pub enum PseudoPrime {
    /// Is not a prime number
    NotPrime,
    /// Is a pseudo prime (most probably prime)
    PseudoPrime,
    /// Is a prime number
    Prime,
}

impl Gen {
    /// Returns the Greatest Common Divisor of `a` and `b`
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(18);
    /// let b = Gen::from(12);
    ///
    /// assert_eq!(6, a.gcd(&b, &ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node29.html>
    pub fn gcd(&self, b: &Gen, ctx: &Context) -> Result<Gen, GiacError> {
        ffi_safe_call! {
            ffi::giacrs_gen_gcd(
                self.as_gen_ref(),
                b.as_gen_ref(),
                result.as_gen_ref(),
                ctx.as_context_ref(),
            )
        }
    }

    /// Returns the Least Common Multiple of `a` and `b`
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(18);
    ///
    /// assert_eq!(90, a.lcm(&15.into())?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node32.html>
    pub fn lcm(&self, b: &Gen) -> Result<Gen, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_lcm(self.as_gen_ref(), b.as_gen_ref(), result.as_gen_ref()) }
    }

    /// Returns the decomposition into prime factors as a symbolic expression
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(90);
    ///
    /// assert_eq!("2*3^2*5", a.ifactor(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node33.html>
    pub fn ifactor(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_ifactor(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Returns the decomposition into prime factors as a vector
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(90);
    ///
    /// assert_eq!("[2,1,3,2,5,1]", a.ifactors(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node34.html>
    pub fn ifactors(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_ifactors(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Returns the decomposition into prime factors, output follow maple syntax
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(90);
    ///
    /// assert_eq!("[1,[[2,1],[3,2],[5,1]]]", a.maple_ifactors(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node35.html>
    pub fn maple_ifactors(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! {
            ffi::giacrs_gen_maple_ifactors(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref())
        }
    }

    /// Returns the list of divisors of an expression
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(36);
    ///
    /// assert_eq!("[1,2,4,3,6,12,9,18,36]", a.divisors(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node36.html>
    pub fn divisors(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_divisors(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Returns the integer quotient q of the euclidian division between a and b (a=b*q+r)
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(148);
    /// let b = Gen::from(5);
    ///
    /// // 148 = 29*5 + 3
    /// assert_eq!(29, a.iquo(&b)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node37.html>
    pub fn iquo(&self, b: &Self) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_iquo(self.as_gen_ref(), b.as_gen_ref(), result.as_gen_ref()) }
    }

    /// Returns the integer remainder r of the euclidian division between a and b (a=b*q+r)
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(148);
    /// let b = Gen::from(5);
    ///
    /// // 148 = 29*5 + 3
    /// assert_eq!(3, a.irem(&b)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node38.html>
    pub fn irem(&self, b: &Self) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_irem(self.as_gen_ref(), b.as_gen_ref(), result.as_gen_ref()) }
    }

    /// Returns the integer quotient q and remainder r of the euclidian division between a and b (a=b*q+r)
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(148);
    /// let b = Gen::from(5);
    ///
    /// // 148 = 29*5 + 3
    /// assert_eq!(29, a.iquorem(&b)?.0.to_int()?);
    /// assert_eq!(3, a.iquorem(&b)?.1.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node39.html>
    pub fn iquorem(&self, b: &Self) -> Result<(Self, Self), GiacError> {
        let r = Self::new();
        let result = Self::new();
        let error = unsafe {
            ffi::giacrs_gen_iquorem(
                self.as_gen_ref(),
                b.as_gen_ref(),
                result.as_gen_ref(),
                r.as_gen_ref(),
            )
        };
        if error == std::ptr::null() {
            Ok((result, r))
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Returns true if the number is even, false otherwise
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(148);
    /// let b = Gen::from(149);
    ///
    /// assert!(a.is_even(&ctx)?);
    /// assert!(!b.is_even(&ctx)?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node40.html>
    pub fn is_even(&self, ctx: &Context) -> Result<bool, GiacError> {
        let mut result = false;
        let error =
            unsafe { ffi::giacrs_gen_even(self.as_gen_ref(), &mut result, ctx.as_context_ref()) };
        if error == std::ptr::null() {
            Ok(result)
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Returns true if the number is odd, false otherwise
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(148);
    /// let b = Gen::from(149);
    ///
    /// assert!(!a.is_odd(&ctx)?);
    /// assert!(b.is_odd(&ctx)?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node41.html>
    pub fn is_odd(&self, ctx: &Context) -> Result<bool, GiacError> {
        let mut result = false;
        let error =
            unsafe { ffi::giacrs_gen_odd(self.as_gen_ref(), &mut result, ctx.as_context_ref()) };
        if error == std::ptr::null() {
            Ok(result)
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Check if a number is pseudoprime.
    /// See [PseudoPrime] enum for more infos.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    /// use giacrs::integers::PseudoPrime;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(100003);
    /// let b = Gen::from(14);
    /// let c = Gen::from_str("9856989898997789789", &ctx)?;
    ///
    /// assert_eq!(PseudoPrime::Prime, a.is_pseudoprime()?);
    /// assert_eq!(PseudoPrime::NotPrime, b.is_pseudoprime()?);
    /// assert_eq!(PseudoPrime::PseudoPrime, c.is_pseudoprime()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node42.html>
    pub fn is_pseudoprime(&self) -> Result<PseudoPrime, GiacError> {
        let mut result: u8 = 0;
        let error = unsafe { ffi::giacrs_gen_is_pseudoprime(self.as_gen_ref(), &mut result) };
        if error == std::ptr::null() {
            Ok(match result {
                0 => PseudoPrime::NotPrime,
                1 => PseudoPrime::PseudoPrime,
                2 => PseudoPrime::Prime,
                _ => unreachable!(),
            })
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Returns the smallest pseudoprime ([PseudoPrime]) number greater than self.
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(75);
    ///
    /// assert_eq!(79, a.next_prime()?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node44.html>
    pub fn next_prime(&self) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_nextprime(self.as_gen_ref(), result.as_gen_ref()) }
    }

    /// Returns the greatest pseudoprime ([PseudoPrime]) number smaller than self.
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(75);
    ///
    /// assert_eq!(73, a.previous_prime()?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node45.html>
    pub fn previous_prime(&self) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_prevprime(self.as_gen_ref(), result.as_gen_ref()) }
    }

    /// Returns the nth prime number.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(75);
    ///
    /// assert_eq!(379, a.nth_prime(&ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node46.html>
    pub fn nth_prime(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_nthprime(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Returns the coefficients of the Bézout's identity.
    /// Returns `(u, v, d)` where `u*a + v*b = d` and `d=gcd(a,b)`
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let a = Gen::from(48);
    ///
    /// let (u, v, d) = a.iegcd(&30.into())?;
    /// // 48*2 + 30*(-3) = 6
    /// assert_eq!(2, u.to_int()?);
    /// assert_eq!(-3, v.to_int()?);
    /// assert_eq!(6, d.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node47.html>
    pub fn iegcd(&self, b: &Gen) -> Result<(Self, Self, Self), GiacError> {
        let u = Self::new();
        let v = Self::new();
        let d = Self::new();
        let error = unsafe {
            ffi::giacrs_gen_iegcd(
                self.as_gen_ref(),
                b.as_gen_ref(),
                u.as_gen_ref(),
                v.as_gen_ref(),
                d.as_gen_ref(),
            )
        };
        if error == std::ptr::null() {
            Ok((u, v, d))
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Solves `a² + b² = p`. `p` must be congruent to 1 modulo 4.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let p = Gen::from(17);
    ///
    /// let (a, b) = p.pa2b2(&ctx)?;
    /// assert_eq!(4, a.to_int()?);
    /// assert_eq!(1, b.to_int()?);
    ///
    /// let q = Gen::from(18);
    /// // 18 != 1 mod 4
    /// assert!(q.pa2b2(&ctx).is_err());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node51.html>
    pub fn pa2b2(&self, ctx: &Context) -> Result<(Self, Self), GiacError> {
        if !(self.irem(&4.into())? - &1.into()).is_zero(ctx)? {
            return Err(GiacError::NoSolution("p must be congruent to 1 modulo 4"));
        }
        let a = Self::new();
        let b = Self::new();
        let error = unsafe {
            ffi::giacrs_gen_pa2b2(
                self.as_gen_ref(),
                a.as_gen_ref(),
                b.as_gen_ref(),
                ctx.as_context_ref(),
            )
        };
        if error == std::ptr::null() {
            Ok((a, b))
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Returns Euler indicatrix for an integer.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(21);
    ///
    /// assert_eq!(12, a.euler(&ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node52.html>
    pub fn euler(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_euler(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }
}

/// Solve the equation `u*a + v*b = c`.
/// c must be a multiple of `gcd(a, b)` for the existence of a solution.
/// ```
/// use giacrs::context::Context;
/// use giacrs::gen::Gen;
/// use giacrs::integers::iabcuv;
///
/// let ctx = Context::new();
///
/// let (u, v) = iabcuv(&48.into(), &30.into(), &18.into(), &ctx)?;
/// // 48*6 + 30*(-9) = 18
/// assert_eq!(6, u.to_int()?);
/// assert_eq!(-9, v.to_int()?);
///
/// // 19 is not a multiple of 6 = gcd(48, 30)
/// assert!(iabcuv(&48.into(), &30.into(), &19.into(), &ctx).is_err());
/// # use giacrs::GiacError;
/// # Ok::<(), GiacError>(())
/// ```
/// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node48.html>
pub fn iabcuv(a: &Gen, b: &Gen, c: &Gen, ctx: &Context) -> Result<(Gen, Gen), GiacError> {
    // TODO avoid making solution check in rust AND in cpp
    let d = a.gcd(b, ctx)?;
    if !(c.irem(&d)?).is_zero(ctx)? {
        return Err(GiacError::NoSolution("c must be a multiple of `gcd(a,b)`"));
    }
    let u = Gen::new();
    let v = Gen::new();
    let error = unsafe {
        ffi::giacrs_gen_iabcuv(
            a.as_gen_ref(),
            b.as_gen_ref(),
            c.as_gen_ref(),
            u.as_gen_ref(),
            v.as_gen_ref(),
            ctx.as_context_ref(),
        )
    };
    if error == std::ptr::null() {
        Ok((u, v))
    } else {
        Err(GiacError::InternalError(error.into()))
    }
}

/// Computes the chinese remainders.
/// Returns `(c, lcm(amod, bmod))` such that `∀k ∈ ℤ, d = c + k*lcm(amod, bmod)` has properties `d=a[amod]` and `d=b[bmod]`
/// ```
/// use giacrs::gen::Gen;
/// use giacrs::integers::ichinrem;
///
/// let (c, cmod) = ichinrem(&3.into(), &5.into(), &9.into(), &13.into())?;
/// assert_eq!(-17, c.to_int()?);
/// assert_eq!(65, cmod.to_int()?);
///
/// # use giacrs::GiacError;
/// # Ok::<(), GiacError>(())
/// ```
/// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node49.html>
pub fn ichinrem(a: &Gen, amod: &Gen, b: &Gen, bmod: &Gen) -> Result<(Gen, Gen), GiacError> {
    let c = Gen::new();
    let error = unsafe {
        ffi::giacrs_gen_ichinrem(
            a.as_gen_ref(),
            amod.as_gen_ref(),
            b.as_gen_ref(),
            bmod.as_gen_ref(),
            c.as_gen_ref(),
        )
    };
    if error == std::ptr::null() {
        Ok((c, amod.lcm(&bmod)?))
    } else {
        Err(GiacError::InternalError(error.into()))
    }
}

/// Computes the Legendre symbol
/// ```
/// use giacrs::gen::Gen;
/// use giacrs::integers::legendre_symbol;
///
/// assert_eq!(1, legendre_symbol(&26.into(), &17.into())?);
/// assert_eq!(-1, legendre_symbol(&27.into(), &17.into())?);
/// assert_eq!(0, legendre_symbol(&34.into(), &17.into())?);
/// # use giacrs::GiacError;
/// # Ok::<(), GiacError>(())
/// ```
/// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node53.html>
pub fn legendre_symbol(a: &Gen, n: &Gen) -> Result<i8, GiacError> {
    let mut result = 0;
    let error = unsafe { ffi::giacrs_gen_legendre(a.as_gen_ref(), n.as_gen_ref(), &mut result) };
    if error == std::ptr::null() {
        Ok(result)
    } else {
        Err(GiacError::InternalError(error.into()))
    }
}

/// Computes the Jacobi symbol
/// ```
/// use giacrs::gen::Gen;
/// use giacrs::integers::jacobi_symbol;
///
/// assert_eq!(1, jacobi_symbol(&25.into(), &12.into())?);
/// assert_eq!(-1, jacobi_symbol(&35.into(), &12.into())?);
/// assert_eq!(0, jacobi_symbol(&33.into(), &12.into())?);
/// # use giacrs::GiacError;
/// # Ok::<(), GiacError>(())
/// ```
/// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node54.html>
pub fn jacobi_symbol(a: &Gen, n: &Gen) -> Result<i8, GiacError> {
    let mut result = 0;
    let error = unsafe { ffi::giacrs_gen_jacobi(a.as_gen_ref(), n.as_gen_ref(), &mut result) };
    if error == std::ptr::null() {
        Ok(result)
    } else {
        Err(GiacError::InternalError(error.into()))
    }
}
