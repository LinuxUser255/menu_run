// custom_mempool_canary.h
#ifndef MENU_RUN_CUSTOM_MEMPOOL_CANARY_H
#define MENU_RUN_CUSTOM_MEMPOOL_CANARY_H

#include <stddef.h>
#include <stdbool.h>

/**
 * MemPoolHardEnd - A fixed-size-slot pool with overflow detection.
 *
 * Identical in structure to MemPool (custom_mempool_hardened.h), but each
 * slot reserves one extra byte past what the caller can use. That byte is
 * stamped with a per-pool random canary value on allocation and checked on
 * free. If the canary doesn't match, the caller wrote past the end of
 * their slot — this is the same technique as set_canary()/check_canary()
 * in hardened_malloc's h_malloc.c, simplified to one canary per pool
 * instead of one per slab.
 */
typedef struct {
    void *region_start;         // base of the single reserved block
    size_t slot_size;           // usable_size + 1 (the extra byte is the canary)
    size_t usable_size;         // bytes the caller is actually allowed to write
    size_t slot_count;          // total number of slots in the pool
    bool *used;                 // used[i] == true means slot i is allocated
    unsigned char canary_value; // stamped at the end of every live slot
} MemPoolHardEnd;

MemPoolHardEnd create_pool_hardend(size_t usable_size, size_t slot_count);
void *allocate_from_pool_hardend(MemPoolHardEnd *pool);
bool free_to_pool_hardend(MemPoolHardEnd *pool, void *ptr);
void destroy_pool_hardend(MemPoolHardEnd *pool);

#endif //MENU_RUN_CUSTOM_MEMPOOL_CANARY_H