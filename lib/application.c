// application.c
#include <stdio.h>
#include "container.h"

int main() {
    printf("Contents of stuff:\n");
    for (int i = 0; i < 3; ++i) {
        printf("%d ", stuff[i]);
    }
    printf("\n");
    return 0;
}