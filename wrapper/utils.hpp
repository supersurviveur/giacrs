#include <ostream>

#define SAFE_VOID_CALL(res_code)                                               \
    try {                                                                      \
        res_code return NULL;                                                  \
    } catch (std::runtime_error & e) {                                         \
        size_t len = strlen(e.what());                                         \
        char *str = (char *)malloc((len + 1) * sizeof(char));                  \
        std::strcpy(str, e.what());                                            \
        return str;                                                            \
    }

#define SAFE_CALL(res_code) SAFE_VOID_CALL(*res = res_code;)

typedef const char *result;

// This is a stream which does not output anything
class NullStream : public std::ostream {
    // streambuffer doing nothing
    class NullBuffer : public std::streambuf {
      public:
        int overflow(int c) noexcept override { return c; }
    } nullBuffer;

  public:
    NullStream() : std::ostream(&nullBuffer) {}
    NullStream(const NullStream &) = delete;
    NullStream &operator=(const NullStream &) = delete;
};

static NullStream nullStream = NullStream();
