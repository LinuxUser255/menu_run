// Custom memory pool example is more conceptual and would typically involve
// complex management of memory. Here is a simplified example:

#include "custom_mempool.h"
#include <stdio.h>
#include <stdlib.h>

// Function to allocate a large block of memory
void *create_memory_pool(size_t size) {
    void *pool = malloc(size);
    if (pool == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(1);
    }
    return pool;
}

// Function to allocate memory from the pool
void *allocate_from_simple_pool(void **pool, size_t size) {
    void *allocation = *pool;
    *pool = (char *)(*pool) + size; // Move the pointer forward
    return allocation;
}

int call_the_custom_mem_pool(void) {
    // Create a memory pool of 1 KB
    void *memory_pool_start = create_memory_pool(1024);
    void *memory_pool_cursor = memory_pool_start;

    // Allocate 256 bytes from the pool
    void *block1 = allocate_from_simple_pool(&memory_pool_cursor, 256);

    // Allocate another 128 bytes from the pool
    void *block2 = allocate_from_simple_pool(&memory_pool_cursor, 128);

    // Prevent unused-variable warnings in this simple demo.
    (void)block1;
    (void)block2;

    free(memory_pool_start);

    return 0;
}