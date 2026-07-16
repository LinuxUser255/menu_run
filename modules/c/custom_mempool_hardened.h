//
// Created by linux on 7/16/26.
//

#ifndef MENU_RUN_CUSTOM_MEMPOOL_HARDEND_H
#define MENU_RUN_CUSTOM_MEMPOOL_HARDEND_H

#include <stddef.h>
#include <stdbool.h>

/**
 * MemPool - A fixed-size-slot memory pool.
 *
 * One large heap block is reserved up front (region_start) and divided
 * into slot_count slots, each exactly slot_size bytes wide. Allocation
 * and deallocation are just bookkeeping (the used[] bitmap) — no calls
 * to malloc/free happen after pool creation.
 */
typedef struct {
    void *region_start;   // base of the single reserved block
    size_t slot_size;     // fixed width of every slot, in bytes
    size_t slot_count;    // total number of slots in the pool
    bool *used;           // used[i] == true means slot i is allocated
} MemPool;

MemPool create_pool(size_t slot_size, size_t slot_count);
void *allocate_from_pool(MemPool *pool);
void free_to_pool(MemPool *pool, void *ptr);
void destroy_pool(MemPool *pool);

#endif //MENU_RUN_CUSTOM_MEMPOOL_HARDEND_H