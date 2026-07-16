// custom_mempool_canary.c
#include "custom_mempool_canary.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define CANARY_SIZE 1

/**
 * create_pool_hardend - Reserves a block and carves it into slots, each
 * with one extra byte reserved for a canary.
 *
 * @usable_size: Bytes the caller may actually write per slot.
 * @slot_count:  Number of slots to reserve.
 *
 * Returns: An initialized MemPoolHardEnd. Exits the program on allocation
 *          failure or invalid arguments, matching create_pool()'s
 *          fail-fast convention.
 *
 * Note: The canary value is seeded from rand()/time(), which is NOT
 *       cryptographically secure. Real hardened_malloc uses a proper CSPRNG
 *       per slab (see random.h / get_random_u64() in h_malloc.c). This is
 *       simplified for teaching the *mechanism*, not production security.
 */
MemPoolHardEnd create_pool_hardend(size_t usable_size, size_t slot_count) {
    if (usable_size == 0 || slot_count == 0) {
        fprintf(stderr, "Error: pool requires nonzero usable_size and slot_count\n");
        exit(1);
    }

    MemPoolHardEnd pool;
    pool.slot_size = usable_size + CANARY_SIZE;
    pool.usable_size = usable_size;
    pool.slot_count = slot_count;

    pool.region_start = malloc(pool.slot_size * slot_count);
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

    // seed rand() once per program run, not once per pool
    static bool rng_seeded = false;
    if (!rng_seeded) {
        srand((unsigned)time(NULL));
        rng_seeded = true;
    }
    pool.canary_value = (unsigned char)(rand() & 0xFF);

    return pool;
}

/**
 * allocate_from_pool_hardend - Claims the first free slot and stamps its
 * canary byte.
 *
 * @pool: The pool to allocate from.
 *
 * Returns: Pointer to a usable_size-byte slot, or NULL if the pool is full.
 *          The returned pointer only grants access to usable_size bytes —
 *          the canary byte immediately after it is off-limits to the caller.
 */
void *allocate_from_pool_hardend(MemPoolHardEnd *pool) {
    for (size_t i = 0; i < pool->slot_count; i++) {
        if (!pool->used[i]) {
            pool->used[i] = true;

            char *slot = (char *)pool->region_start + (i * pool->slot_size);
            slot[pool->usable_size] = (char)pool->canary_value;

            return slot;
        }
    }

    return NULL; // pool exhausted
}

/**
 * free_to_pool_hardend - Validates the canary, then returns a slot to the
 * pool for reuse.
 *
 * @pool: The pool that ptr was allocated from.
 * @ptr:  A pointer previously returned by allocate_from_pool_hardend().
 *
 * Note: A corrupted canary means the caller wrote past usable_size bytes
 *       into the guard byte. This is reported and the slot is still
 *       reclaimed — a real allocator (see h_malloc.c's fatal_error() in
 *       check_canary()) aborts the whole process instead, since a
 *       corrupted heap is no longer trustworthy to keep running on.
 */
//
bool free_to_pool_hardend(MemPoolHardEnd *pool, void *ptr) {
    if (ptr == NULL) {
        return true; // freeing NULL is a no-op, trivially "no corruption"
    }

    size_t offset = (char *)ptr - (char *)pool->region_start;
    size_t index = offset / pool->slot_size;

    if (index >= pool->slot_count || offset % pool->slot_size != 0) {
        fprintf(stderr, "Error: invalid pointer passed to free_to_pool_hardend\n");
        return false;
    }

    if (!pool->used[index]) {
        fprintf(stderr, "Error: double free detected\n");
        return false;
    }

    char *slot = (char *)ptr;
    bool canary_ok = (unsigned char)slot[pool->usable_size] == pool->canary_value;
    if (!canary_ok) {
        fprintf(stderr, "Error: canary corrupted — buffer overflow detected\n");
    }

    pool->used[index] = false;
    return canary_ok;
}

//

/**
 * destroy_pool_hardend - Releases the pool's two backing allocations.
 *
 * @pool: The pool to tear down. Not usable after this call.
 */
void destroy_pool_hardend(MemPoolHardEnd *pool) {
    free(pool->region_start);
    free(pool->used);
    pool->region_start = NULL;
    pool->used = NULL;
}