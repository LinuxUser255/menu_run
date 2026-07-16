#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include "mem_demo.h"
#include "custom_mempool.h"
#include "custom_mempool_hardened.h"
#include "custom_mempool_canary.h"

int main(void) {
    puts("HELLO FROM THE C MODULE");

    // --- mem_demo: heap-allocated string with manual ownership ---
    char *name = ask_name_malloc(100);
    if (name == NULL) {
        fprintf(stderr, "Nothing entered, or allocation failed.\n");
        return 1;
    }
    free_name(name);

    // --- custom_mempool: basic bump-pointer pool ---
    call_the_custom_mem_pool();

    // --- custom_mempool_hardened: fixed-size-slot pool with reuse ---
    MemPool pool = create_pool(100, 10);

    void *ptr = allocate_from_pool(&pool);
    if (ptr == NULL) {
        fprintf(stderr, "Error: pool exhausted\n");
    } else {
        free_to_pool(&pool, ptr);
    }

    destroy_pool(&pool);

    // --- custom_mempool_canary: fixed-size-slot pool with overflow detection ---
    puts("Running hardened (canary) pool demo...");
    MemPoolHardEnd pool_hardend = create_pool_hardend(100, 10);

    void *ptr2 = allocate_from_pool_hardend(&pool_hardend);
    if (ptr2 == NULL) {
        fprintf(stderr, "Error: hardened pool exhausted\n");
    } else {
        memset(ptr2, 'A', 101); // usable_size is 100 — this stomps the canary byte
        bool intact = free_to_pool_hardend(&pool_hardend, ptr2);
        if (intact) {
            puts("Canary check passed - no overflow detected");
        }
    }

    destroy_pool_hardend(&pool_hardend);

    return 0;
}