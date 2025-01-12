#include "gen.hpp"
#include "utils.hpp"
#include <cstdint>
#include <cstring>
#include <giac/config.h>
#include <giac/gausspol.h>
#include <giac/gen.h>
#include <giac/giac.h>
#include <giac/global.h>
#include <giac/ifactor.h>
#include <giac/intgab.h>
#include <giac/misc.h>
#include <giac/modpoly.h>
#include <giac/pari.h>
#include <giac/subst.h>
#include <giac/sym2poly.h>
#include <giac/usual.h>
#include <giac/vecteur.h>
#include <stdexcept>

char *string_to_c(std::string s) {
    char *str = (char *)malloc((s.length() + 1) * sizeof(char));
    std::strcpy(str, s.c_str());
    return str;
}

// ALLOCATION

extern "C" giac::gen *giacrs_gen_allocate() { return new giac::gen(); }
extern "C" result giacrs_gen_from_str(char *s, const giac::context *ctx,
                                      giac::gen *res) {
    *res = giac::gen(s, ctx);
    if (giac::first_error_line(ctx) != 0) {
        return string_to_c(giac::parser_error(ctx));
    }
    // Evaluate expression
    SAFE_CALL(giac::eval(*res, ctx))
}

extern "C" giac::gen *giacrs_gen_from_int(int i) { return new giac::gen(i); }
extern "C" giac::gen *giacrs_gen_from_float(float i) {
    return new giac::gen(i);
}
extern "C" giac::gen *giacrs_gen_from_double(double i) {
    return new giac::gen(i);
}

extern "C" giac::gen *giacrs_gen_factorial(unsigned long i) {
    return new giac::gen(giac::factorial(i));
}

extern "C" void giacrs_free_gen(giac::gen *e) { delete e; }

extern "C" giac::gen *giacrs_gen_clone(giac::gen *e) {
    return new giac::gen(*e);
}

// DATA

extern "C" unsigned char giacrs_gen_type(giac::gen *e) { return e->type; }
extern "C" result giacrs_gen_is_zero(giac::gen *e, bool *res,
                                     giac::context *ctx) {
    SAFE_CALL(giac::is_zero(*e, ctx));
}

// CONVERSION

extern "C" const char *giacrs_gen_to_str(giac::gen *e) {
    return string_to_c(e->print());
}

extern "C" result giacrs_gen_to_int(giac::gen *e, int *res) {
    SAFE_CALL(e->to_int());
}

// METHODS

extern "C" result giacrs_gen_gcd(giac::gen *a, giac::gen *b, giac::gen *res,
                                 const giac::context *ctx) {
    SAFE_CALL(giac::gcd(*a, *b, ctx));
}

extern "C" result giacrs_gen_lcm(giac::gen *a, giac::gen *b, giac::gen *res) {
    SAFE_CALL(giac::lcm(*a, *b));
}

extern "C" result giacrs_gen_ifactor(giac::gen *e, giac::gen *res,
                                     const giac::context *ctx) {
    SAFE_CALL(giac::_ifactor(*e, ctx));
}

extern "C" result giacrs_gen_ifactors(giac::gen *e, giac::gen *res,
                                      const giac::context *ctx) {
    SAFE_CALL(giac::_ifactors(*e, ctx));
}

extern "C" result giacrs_gen_maple_ifactors(giac::gen *e, giac::gen *res,
                                            const giac::context *ctx) {
    SAFE_CALL(giac::_maple_ifactors(*e, ctx));
}

extern "C" result giacrs_gen_divisors(giac::gen *e, giac::gen *res,
                                      const giac::context *ctx) {
    SAFE_CALL(giac::idivis(*e, ctx));
}

extern "C" result giacrs_gen_iquo(giac::gen *a, giac::gen *b, giac::gen *res) {
    SAFE_CALL(giac::iquo(*a, *b));
}

extern "C" result giacrs_gen_iquorem(giac::gen *a, giac::gen *b, giac::gen *q,
                                     giac::gen *res) {
    SAFE_CALL(giac::irem(*a, *b, *q));
}

extern "C" result giacrs_gen_irem(giac::gen *a, giac::gen *b, giac::gen *res) {
    giac::gen q;
    return giacrs_gen_iquorem(a, b, &q, res);
}

extern "C" result giacrs_gen_even(giac::gen *a, bool *res, giac::context *ctx) {
    SAFE_CALL(!giac::is_zero(giac::_even(*a, ctx)));
}

extern "C" result giacrs_gen_odd(giac::gen *a, bool *res, giac::context *ctx) {
    SAFE_CALL(!giac::is_zero(giac::_odd(*a, ctx)));
}

extern "C" result giacrs_gen_is_pseudoprime(giac::gen *a, int8_t *res) {
    SAFE_CALL(giac::is_probab_prime_p(*a));
}

extern "C" result giacrs_gen_nextprime(giac::gen *a, giac::gen *res) {
    SAFE_CALL(giac::nextprime(*a));
}

extern "C" result giacrs_gen_prevprime(giac::gen *a, giac::gen *res) {
    SAFE_CALL(giac::prevprime(*a));
}

extern "C" result giacrs_gen_nthprime(giac::gen *a, giac::gen *res,
                                      giac::context *ctx) {
    try {
        *res = giac::_ithprime(*a, ctx);
        if (giac::is_undef(*res)) {
            throw std::runtime_error(
                "Failed to compute nthprime, argument is too big");
        }
        return NULL;
    } catch (std::runtime_error &e) {
        size_t len = strlen(e.what());
        char *str = (char *)malloc((len + 1) * sizeof(char));
        std::strcpy(str, e.what());
        return str;
    }
}

extern "C" result giacrs_gen_iegcd(giac::gen *a, giac::gen *b, giac::gen *u,
                                   giac::gen *v, giac::gen *d) {
    SAFE_VOID_CALL(giac::egcd(*a, *b, *u, *v, *d););
}

extern "C" result giacrs_gen_iabcuv(giac::gen *a, giac::gen *b, giac::gen *c,
                                    giac::gen *u, giac::gen *v,
                                    giac::context *ctx) {
    SAFE_VOID_CALL({
        giac::gen vec = giac::iabcuv(*a, *b, *c, ctx);
        *u = vec[0];
        *v = vec[1];
    });
}

extern "C" result giacrs_gen_ichinrem(giac::gen *a, giac::gen *amod,
                                      giac::gen *b, giac::gen *bmod,
                                      giac::gen *res) {
    SAFE_CALL(giac::ichinrem(*a, *b, *amod, *bmod));
}

extern "C" result giacrs_gen_pa2b2(giac::gen *p, giac::gen *a, giac::gen *b,
                                   giac::context *ctx) {
    SAFE_VOID_CALL({
        giac::gen vec = giac::pa2b2(*p, ctx);
        *a = vec[0];
        *b = vec[1];
    });
}

extern "C" result giacrs_gen_euler(giac::gen *a, giac::gen *res,
                                   giac::context *ctx) {
    SAFE_CALL(giac::euler(*a, ctx));
}

extern "C" result giacrs_gen_legendre(giac::gen *a, giac::gen *b, int8_t *res) {
    SAFE_CALL(giac::legendre(*a, *b));
}

extern "C" result giacrs_gen_jacobi(giac::gen *a, giac::gen *b, int8_t *res) {
    SAFE_CALL(giac::jacobi(*a, *b));
}

extern "C" result giacrs_gen_comb(giac::gen *n, giac::gen *k, giac::gen *res,
                                  giac::context *ctx) {
    SAFE_CALL(giac::comb(*n, *k, ctx));
}

extern "C" result giacrs_gen_perm(giac::gen *n, giac::gen *k, giac::gen *res,
                                  giac::context *ctx) {
    giac::vecteur vec(2);
    vec[0] = *n;
    vec[1] = *k;
    SAFE_CALL(giac::_perm(giac::gen(vec), ctx));
}

extern "C" result giacrs_gen_rand(giac::gen *n, giac::gen *res,
                                  giac::context *ctx) {
    SAFE_CALL(giac::_rand(*n, ctx));
}

extern "C" result giacrs_gen_float2rational(giac::gen *n, giac::gen *res,
                                            giac::context *ctx) {
    SAFE_CALL(giac::_float2rational(*n, ctx));
}

// OPERATOR

extern "C" result giacrs_gen_add(giac::gen *res, giac::gen *f) {
    SAFE_CALL(*res + *f)
}
extern "C" result giacrs_gen_sub(giac::gen *res, giac::gen *f) {
    SAFE_CALL(*res - *f)
}
extern "C" result giacrs_gen_mul(giac::gen *res, giac::gen *f) {
    SAFE_CALL(*res * *f)
}
extern "C" result giacrs_gen_div(giac::gen *res, giac::gen *f) {
    SAFE_CALL(*res / *f)
}

// OTHERS

extern "C" result giacrs_gen_factor(giac::gen *e, giac::gen *res,
                                    const giac::context *ctx) {
    SAFE_CALL(giac::_factor(*e, ctx));
}

extern "C" result giacrs_gen_simplify(giac::gen *e, giac::gen *res,
                                      const giac::context *ctx) {
    SAFE_CALL(giac::simplify(*e, ctx));
}

extern "C" result giacrs_gen_det(giac::gen *e, giac::gen *res,
                                 const giac::context *ctx) {
    SAFE_CALL(giac::_det(*e, ctx));
}
