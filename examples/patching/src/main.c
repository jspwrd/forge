#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "config.h"

int main(void) {
    const char *ip   = CALLBACK_IP;
    const char *port = CALLBACK_PORT;
    const char *interval = SLEEP_SECONDS;

    printf("[*] agent v1.0.0\n");
    printf("[*] target: %s:%s\n", ip, port);
    printf("[*] sleep interval: %s\n", interval);

    /* Simulate a beacon loop */
    for (int i = 0; i < 3; i++) {
        printf("[*] beacon %d -> %s:%s\n", i + 1, ip, port);
        sleep(1);
    }

    printf("[+] done\n");
    return 0;
}
