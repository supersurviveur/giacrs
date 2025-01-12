#include "utils.hpp"
#include <giac/global.h>

extern "C" void giacrs_options_set_epsilon(double e,
                                             const giac::context *ctx) {
    giac::epsilon(e, ctx);
}
