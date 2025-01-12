#include "context.hpp"
#include <giac/global.h>

const giac::context *giacrs_global_context = giac::context0;

extern "C" void giacrs_init_global_context() {
    giac::logptr(&nullStream, giac::context0);
}

extern "C" const giac::context *giacrs_new_context() {
    giac::context *c = new giac::context();
    giac::logptr(&nullStream, c);
    return c;
}
extern "C" void giacrs_free_context(const giac::context *ctx) { delete ctx; }

extern "C" void giacrs_release_globals() { giac::release_globals(); }
