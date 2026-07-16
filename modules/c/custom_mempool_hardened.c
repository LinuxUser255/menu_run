#include "custom_mempool_hardened.h"
#include <stdio.h>
#include <stdlib.h>

/**
 * create_pool - Reserves one large block and carves it into fixed-size slots.
 *
 * @slot_size:  Size in bytes of each slot (this pool's "size class").
 * @slot_count: Number of slots to reserve.
 *
 * Returns: An initialized MemPool. Exits the program on allocation failure,
 *          since a pool that fails to construct has nothing usable to return.
 */
MemPool create_pool(size_t slot_size, size_t slot_count) {
    MemPool pool;

    pool.region_start = malloc(slot_size * slot_count);
    if (pool.region_start == NULL) {
        fprintf(stderr, "Error: pool region allocation failed\n");
        exit(1);
    }

    pool.used = calloc(slot_count, sizeof(bool));
    if (pool.used == NULL) {
        free(pool.region_start);
        fprintf(stderr, "Error: pool bookkeeping allocation failed\n");
        exit(1);
    }

    pool.slot_size = slot_size;
    pool.slot_count = slot_count;

    return pool;
}

/**
 * allocate_from_pool - Claims the first free slot in the pool.
 *
 * @pool: The pool to allocate from.
 *
 * Returns: Pointer to a slot_size-byte slot, or NULL if the pool is full.
 */
void *allocate_from_pool(MemPool *pool) {
    for (size_t i = 0; i < pool->slot_count; i++) {
        if (!pool->used[i]) {
            pool->used[i] = true;
            return (char *)pool->region_start + (i * pool->slot_size);
        }
    }

    return NULL; // pool exhausted
}

/**
 * free_to_pool - Returns a slot to the pool for reuse.
 *
 * @pool: The pool that ptr was allocated from.
 * @ptr:  A pointer previously returned by allocate_from_pool() on this pool.
 */
void free_to_pool(MemPool *pool, void *ptr) {
    size_t offset = (char *)ptr - (char *)pool->region_start;
    size_t index = offset / pool->slot_size;

    if (index >= pool->slot_count || offset % pool->slot_size != 0) {
        fprintf(stderr, "Error: invalid pointer passed to free_to_pool\n");
        return;
    }

    if (!pool->used[index]) {
        fprintf(stderr, "Error: double free detected\n");
        return;
    }

    pool->used[index] = false;
}

/**
 * destroy_pool - Releases the pool's two backing allocations.
 *
 * @pool: The pool to tear down. Not usable after this call.
 */
void destroy_pool(MemPool *pool) {
    free(pool->region_start);
    free(pool->used);
    pool->region_start = NULL;
    pool->used = NULL;
}