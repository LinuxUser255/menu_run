#include "mem_demo.h"
#include <stdio.h>

int main(void) {
    puts("HELLO FROM THE C MODULE");

    char *name = ask_name_malloc(100);
    if (name == NULL)  {
        fprintf(stderr, "Nothing entered, or allocation failed.\n");
        return 1;
    }

    free_name(name) ;
    return 0;
}

