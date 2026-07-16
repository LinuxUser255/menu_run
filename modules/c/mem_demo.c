#include <stdlib.h>
#include "mem_demo.h"
#include <stdio.h>
#include <string.h>
#include <limits.h>

/**
 * ask_name_malloc function - Allocates memory and reads a name from the user
 *
 * @size: Maximum number of characters to read (including the null terminator)
 *
 * Returns:
 *   - Pointer to the allocated string on success
 *   - NULL if memory allocation fails or input fails
 *
 * Note: The caller is responsible for freeing the returned memory
 *       using free_name().
 */
char *ask_name_malloc(size_t size) {
    // refuse zero-sized request
    if (size == 0) {
        return NULL;
    }

    // allocate memory on the heap
    char *name = malloc(size);
    if (name == NULL) {
        fprintf(stderr, "Error: Failed to allocate memory\n");
        return NULL;
    }

    printf("Enter your name: ");

    // cap the value at INT_MAX
    int max_read = (size > INT_MAX) ? INT_MAX : (int)size;

    // Read input, and free the memory before returning NULL
    if (fgets(name, max_read, stdin) == NULL) {
        free(name);
        return NULL;
    }

    // Use indexing to find the newline and replace it with a 0
    name[strcspn(name, "\n")] = '\0';

    // Reject empty input (check if nothing at index 0)
    if (name[0] == '\0') {
        free(name);
        return NULL;
    }

    printf("Hello, %s.\n", name);
    return name;
}

// free name function - Safely frees memory allocated by the ask_name_malloc function
void free_name(char *name) {
    free(name); // free(NULL) is a no-op, safe to call
}









