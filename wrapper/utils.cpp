#include "utils.hpp"
#include <cstdlib>

extern "C" void giacrs_free_str(char *s) { free(s); }
