
#ifndef MENU_RUN_CUSTOM_MEMPOOL_H
#define MENU_RUN_CUSTOM_MEMPOOL_H

#include <stddef.h>

void *create_memory_pool(size_t size);
void *allocate_from_simple_pool(void **pool, size_t size);
int call_the_custom_mem_pool(void);

#endif //MENU_RUN_CUSTOM_MEMPOOL_H