#include <stdio.h>
#include "platform.h"

int main(void) {
    printf("[*] multiarch running on %s\n", platform_name());
    printf("[*] pointer size: %zu bytes\n", sizeof(void *));
    return 0;
}
