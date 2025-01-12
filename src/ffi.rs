//! FFI functions to communicate with giac.
//! All these functions start with `giacrs_` to avoid conflict with giac names on the cpp side.

use crate::{
    context::GiacContextRef,
    gen::GiacGenRef,
    types::GenType,
};

pub type GiacResult = *const std::os::raw::c_char;

extern "C" {
    pub fn giacrs_free_str(str: *const std::os::raw::c_char);

    pub fn giacrs_gen_allocate() -> GiacGenRef;
    pub fn giacrs_gen_from_str(
        str: *const std::os::raw::c_char,
        ctx: GiacContextRef,
        expr: GiacGenRef,
    ) -> GiacResult;

    pub fn giacrs_gen_from_int(i: std::os::raw::c_int) -> GiacGenRef;
    pub fn giacrs_gen_from_float(i: std::os::raw::c_float) -> GiacGenRef;
    pub fn giacrs_gen_from_double(i: std::os::raw::c_double) -> GiacGenRef;
    pub fn giacrs_gen_factorial(i: std::os::raw::c_ulong) -> GiacGenRef;
    pub fn giacrs_free_gen(expr: GiacGenRef);

    pub fn giacrs_gen_clone(expr: GiacGenRef) -> GiacGenRef;

    // DATA
    pub fn giacrs_gen_type(expr: GiacGenRef) -> GenType;
    pub fn giacrs_gen_is_zero(expr: GiacGenRef, res: *mut bool, ctx: GiacContextRef) -> GiacResult;
    // CONVERSION
    pub fn giacrs_gen_to_str(expr: GiacGenRef) -> *const std::os::raw::c_char;
    pub fn giacrs_gen_to_int(expr: GiacGenRef, res: *mut std::os::raw::c_int) -> GiacResult;

    // METHODS
    pub fn giacrs_gen_gcd(
        a: GiacGenRef,
        b: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_lcm(a: GiacGenRef, b: GiacGenRef, res: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_ifactor(expr: GiacGenRef, res: GiacGenRef, ctx: GiacContextRef)
        -> GiacResult;
    pub fn giacrs_gen_ifactors(
        expr: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_maple_ifactors(
        expr: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_divisors(
        expr: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_iquo(a: GiacGenRef, b: GiacGenRef, res: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_irem(a: GiacGenRef, b: GiacGenRef, res: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_iquorem(
        a: GiacGenRef,
        b: GiacGenRef,
        res: GiacGenRef,
        r: GiacGenRef,
    ) -> GiacResult;
    pub fn giacrs_gen_even(expr: GiacGenRef, res: *mut bool, ctx: GiacContextRef) -> GiacResult;
    pub fn giacrs_gen_odd(expr: GiacGenRef, res: *mut bool, ctx: GiacContextRef) -> GiacResult;
    pub fn giacrs_gen_is_pseudoprime(expr: GiacGenRef, res: *mut u8) -> GiacResult;
    pub fn giacrs_gen_nextprime(expr: GiacGenRef, res: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_prevprime(expr: GiacGenRef, res: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_nthprime(
        expr: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_iegcd(
        a: GiacGenRef,
        b: GiacGenRef,
        u: GiacGenRef,
        v: GiacGenRef,
        d: GiacGenRef,
    ) -> GiacResult;
    pub fn giacrs_gen_iabcuv(
        a: GiacGenRef,
        b: GiacGenRef,
        c: GiacGenRef,
        u: GiacGenRef,
        v: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_ichinrem(
        a: GiacGenRef,
        amod: GiacGenRef,
        b: GiacGenRef,
        bmod: GiacGenRef,
        c: GiacGenRef,
    ) -> GiacResult;
    pub fn giacrs_gen_pa2b2(
        p: GiacGenRef,
        a: GiacGenRef,
        b: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_euler(a: GiacGenRef, res: GiacGenRef, ctx: GiacContextRef) -> GiacResult;
    pub fn giacrs_gen_legendre(a: GiacGenRef, b: GiacGenRef, res: *mut i8) -> GiacResult;
    pub fn giacrs_gen_jacobi(a: GiacGenRef, b: GiacGenRef, res: *mut i8) -> GiacResult;
    pub fn giacrs_gen_comb(
        n: GiacGenRef,
        k: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_perm(
        n: GiacGenRef,
        k: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_rand(n: GiacGenRef, res: GiacGenRef, ctx: GiacContextRef) -> GiacResult;
    pub fn giacrs_gen_float2rational(
        f: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;

    // OPERATOR

    pub fn giacrs_gen_add(left: GiacGenRef, right: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_sub(left: GiacGenRef, right: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_mul(left: GiacGenRef, right: GiacGenRef) -> GiacResult;
    pub fn giacrs_gen_div(left: GiacGenRef, right: GiacGenRef) -> GiacResult;
}

extern "C" {
    pub fn giacrs_options_set_epsilon(epsilon: std::os::raw::c_double, ctx: GiacContextRef);
}

extern "C" {
    pub static giacrs_global_context: GiacContextRef;
    pub fn giacrs_init_global_context();
    pub fn giacrs_new_context() -> GiacContextRef;
    pub fn giacrs_free_context(ctx: GiacContextRef);

    pub fn giacrs_release_globals();

    pub fn giacrs_gen_factor(expr: GiacGenRef, res: GiacGenRef, ctx: GiacContextRef) -> GiacResult;
    pub fn giacrs_gen_simplify(
        expr: GiacGenRef,
        res: GiacGenRef,
        ctx: GiacContextRef,
    ) -> GiacResult;
    pub fn giacrs_gen_det(expr: GiacGenRef, res: GiacGenRef, ctx: GiacContextRef) -> GiacResult;
}
