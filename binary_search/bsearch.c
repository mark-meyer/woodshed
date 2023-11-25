/**
 * Simple c wrapper to demonstrate the assembly function binary_search and print the index 
 * where the target is found, or -1 if not found
 * 
 * Usage: 
 * > bsearch 20 1 4 10 20 54 109
 * Found 20 at index 3 
 * 
 * Will search for the index of 20 in the sorted array {1, 4, 10, 20, 54, 109 } and print the result
 * It will report an index of -1 if the target is not found in the lis
 * 
*/

#include <stdio.h>
#include <stdlib.h>

extern int binary_search(int a[], int len, int target);


int main(int argc, char *argv[]) {
    
    if (argc < 3) {
        printf("Usage: %s target_number number 1 [number2 ...]\n", argv[0]);
        return 1;
    }
    int size = argc - 2;
    int *array = malloc(size * sizeof(int));

    if (array == NULL) {
        perror("Failed to allocate memory\n");
        return 1;
    }

    int n = atoi(argv[1]);

    for (int i = 0; i < size; i++) {
        array[i] = atoi(argv[i + 2]);
    }

    int found_index = binary_search(array, size, n);
    printf("Found %d at index %d\n", n, found_index);
    return 0;
}