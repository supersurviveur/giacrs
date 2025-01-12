//! A `Gen` object representing an expression

use std::{
    ffi::CString,
    fmt::Display,
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use giacrs_internals::{ffi_safe_call, ffi_safe_panic_inplace_call};

use crate::{context::Context, ffi, types::GenType, GiacError, GiacString};

pub(crate) enum GiacGen {}
pub(crate) type GiacGenRef = *mut GiacGen;

/// A giac expression.
/// All methods which computes using an expression should return a `Result<_, GiacError>`, since giac may always throw an exception if some arguments are not correct.
#[derive(Debug)]
pub struct Gen(pub(crate) GiacGenRef);

impl Gen {
    /// Create a new empty `Gen` object
    pub fn new() -> Self {
        Self(unsafe { ffi::giacrs_gen_allocate() })
    }

    /// Returns the internal reference to the Cpp structure
    pub(crate) unsafe fn as_gen_ref(&self) -> GiacGenRef {
        self.0
    }

    /// Creates a `Gen` object from a string
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from_str("12", &ctx)?;
    ///
    /// assert_eq!(12, a.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node29.html>
    pub fn from_str(str: &str, ctx: &Context) -> Result<Self, GiacError> {
        let s = CString::new(str).unwrap();
        ffi_safe_call! { ffi::giacrs_gen_from_str(s.as_ptr(), ctx.as_context_ref(), result.as_gen_ref()) }
    }

    /// Creates a `Gen` object from a factorial
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// let value = Gen::factorial(24);
    /// assert_eq!("620448401733239439360000", value.to_string());
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node28.html>
    pub fn factorial(i: u64) -> Self {
        Self(unsafe { ffi::giacrs_gen_factorial(i) })
    }

    /// Returns the string representation of an expression.
    /// This returns a [GiacString] object.
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// assert_eq!("42", Gen::from(42).print_to_string().to_string());
    /// assert_eq!("24", Gen::factorial(4).print_to_string().to_string());
    /// ```
    pub fn print_to_string(&self) -> GiacString {
        let str = unsafe { ffi::giacrs_gen_to_str(self.as_gen_ref()) };
        unsafe { GiacString::new(str) }
    }

    /// Converts an expression to a primitive integer
    /// ```
    /// use giacrs::gen::Gen;
    ///
    /// assert_eq!(42, Gen::from(42).to_int()?);
    /// assert_eq!(24, Gen::factorial(4).to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn to_int(&self) -> Result<i32, GiacError> {
        let mut result = 0;
        let error = unsafe { ffi::giacrs_gen_to_int(self.as_gen_ref(), &mut result) };
        if error == std::ptr::null() {
            Ok(result)
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Returns the giac type of the expression. See [GenType]
    /// ```
    /// use giacrs::gen::Gen;
    /// use giacrs::types::GenType;
    ///
    /// assert_eq!(GenType::Int, Gen::from(42).get_type());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn get_type(&self) -> GenType {
        unsafe { ffi::giacrs_gen_type(self.as_gen_ref()) }
    }

    /// Checks if the expression equals zero
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    ///
    /// assert!(Gen::from(0).is_zero(&ctx)?);
    /// assert!(!Gen::from(10).is_zero(&ctx)?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn is_zero(&self, ctx: &Context) -> Result<bool, GiacError> {
        let mut result = false;
        let error = unsafe {
            ffi::giacrs_gen_is_zero(self.as_gen_ref(), &mut result, ctx.as_context_ref())
        };
        if error == std::ptr::null() {
            Ok(result)
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    }

    /// Factorizes the expression.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let poly = Gen::from_str("x^2-1", &ctx)?;
    ///
    /// assert_eq!("(x-1)*(x+1)", poly.factor(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn factor(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_factor(self.as_gen_ref(),result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Simplifies the expression.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let poly = Gen::from_str("(x-1)*(x+1)", &ctx)?;
    ///
    /// assert_eq!("x^2-1", poly.simplify(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    pub fn simplify(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_simplify(self.as_gen_ref(),result.as_gen_ref(), ctx.as_context_ref()) }
    }

    /// Computes the determinant of the matrix.
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let mat = Gen::from_str("[[1,2],[3,4]]", &ctx)?;
    ///
    /// assert_eq!(-2, mat.det(&ctx)?.to_int()?);
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```

    pub fn det(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! { ffi::giacrs_gen_det(self.as_gen_ref(),result.as_gen_ref(), ctx.as_context_ref()) }
    }

    // GIAC METHODS

    /// Converts a floating point number d to a rational number q approaching d such that `abs(q-d) < epsilon` ([Context::set_epsilon])
    /// ```
    /// use giacrs::context::Context;
    /// use giacrs::gen::Gen;
    ///
    /// let ctx = Context::new();
    /// let a = Gen::from(0.125);
    ///
    /// assert_eq!("1/8", a.float_to_rational(&ctx)?.to_string());
    /// # use giacrs::GiacError;
    /// # Ok::<(), GiacError>(())
    /// ```
    /// <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/node61.html>
    pub fn float_to_rational(&self, ctx: &Context) -> Result<Self, GiacError> {
        ffi_safe_call! {ffi::giacrs_gen_float2rational(self.as_gen_ref(), result.as_gen_ref(), ctx.as_context_ref()) }
    }
}

impl TryFrom<u64> for Gen {
    type Error = TryFromIntError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self(unsafe { ffi::giacrs_gen_from_int(value.try_into()?) }))
    }
}

impl TryFrom<u32> for Gen {
    type Error = TryFromIntError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(unsafe { ffi::giacrs_gen_from_int(value.try_into()?) }))
    }
}

impl From<u16> for Gen {
    fn from(value: u16) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_int(value.into()) })
    }
}

impl From<u8> for Gen {
    fn from(value: u8) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_int(value.into()) })
    }
}

impl TryFrom<i64> for Gen {
    type Error = TryFromIntError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(unsafe { ffi::giacrs_gen_from_int(value.try_into()?) }))
    }
}

impl From<i32> for Gen {
    fn from(value: i32) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_int(value.into()) })
    }
}

impl From<i16> for Gen {
    fn from(value: i16) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_int(value.into()) })
    }
}

impl From<i8> for Gen {
    fn from(value: i8) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_int(value.into()) })
    }
}

impl From<f32> for Gen {
    fn from(value: f32) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_float(value.into()) })
    }
}

impl From<f64> for Gen {
    fn from(value: f64) -> Self {
        Self(unsafe { ffi::giacrs_gen_from_double(value.into()) })
    }
}

impl TryFrom<Gen> for i32 {
    type Error = GiacError;

    fn try_from(value: Gen) -> Result<Self, Self::Error> {
        value.to_int()
    }
}

impl Display for Gen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_to_string())
    }
}

impl Clone for Gen {
    fn clone(&self) -> Self {
        Self(unsafe { ffi::giacrs_gen_clone(self.as_gen_ref()) })
    }
}

impl Add<&Self> for Gen {
    type Output = Gen;

    fn add(mut self, rhs: &Gen) -> Self::Output {
        self += rhs;
        self
    }
}
impl AddAssign<&Self> for Gen {
    fn add_assign(&mut self, rhs: &Self) {
        ffi_safe_panic_inplace_call! { ffi::giacrs_gen_add(self.as_gen_ref(), rhs.as_gen_ref()) };
    }
}

impl Sub<&Self> for Gen {
    type Output = Gen;

    fn sub(mut self, rhs: &Gen) -> Self::Output {
        self -= rhs;
        self
    }
}
impl SubAssign<&Self> for Gen {
    fn sub_assign(&mut self, rhs: &Self) {
        ffi_safe_panic_inplace_call! { ffi::giacrs_gen_sub(self.as_gen_ref(), rhs.as_gen_ref()) };
    }
}

impl Mul<&Self> for Gen {
    type Output = Gen;

    fn mul(mut self, rhs: &Gen) -> Self::Output {
        self *= rhs;
        self
    }
}
impl MulAssign<&Self> for Gen {
    fn mul_assign(&mut self, rhs: &Self) {
        ffi_safe_panic_inplace_call! { ffi::giacrs_gen_mul(self.as_gen_ref(), rhs.as_gen_ref()) };
    }
}

impl Div<&Self> for Gen {
    type Output = Gen;

    fn div(mut self, rhs: &Gen) -> Self::Output {
        self /= rhs;
        self
    }
}
impl DivAssign<&Self> for Gen {
    fn div_assign(&mut self, rhs: &Self) {
        ffi_safe_panic_inplace_call! { ffi::giacrs_gen_div(self.as_gen_ref(), rhs.as_gen_ref()) };
    }
}

// TODO: implement remainder, but it's not the same as irem in giac
// impl Rem<&Self> for Gen {
//     type Output = Gen;

//     fn rem(mut self, rhs: &Gen) -> Self::Output {
//         self %= rhs;
//         self
//     }
// }
// impl RemAssign<&Self> for Gen {
//     fn rem_assign(&mut self, rhs: &Self) {
//         let r = self.irem(rhs).unwrap_or_else(|err| panic!("{:?}", err));
//         *self = r;
//     }
// }

impl Drop for Gen {
    fn drop(&mut self) {
        unsafe { ffi::giacrs_free_gen(self.as_gen_ref()) };
    }
}
