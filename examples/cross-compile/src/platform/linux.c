#include "platform.h"

#if defined(__aarch64__)
    static const char *name = "linux-aarch64";
#elif defined(__arm__)
    static const char *name = "linux-armhf";
#elif defined(__mips__)
    static const char *name = "linux-mips";
#elif defined(__x86_64__)
    static const char *name = "linux-x86_64";
#else
    static const char *name = "linux-unknown";
#endif

const char *platform_name(void) {
    return name;
}
