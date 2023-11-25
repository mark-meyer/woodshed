#include <stdio.h>
#include <assert.h>
#include <stdbool.h>

extern void _quicksort(int array[], int size );

bool isArrayEqual(int array1[], int array2[], size_t size) {
    for (size_t i = 0; i < size; i++) {
        if (array1[i] != array2[i]) {
            return false;
        }
    }
    return true;
}

int main() {
    int sorted[] =  {-1, 1, 17, 20, 20, 99, 100};
    int array[] = {100, 20, -1, 17, 99, 20, 1};
    int n = 7;

    _quicksort(array, n);
    assert(isArrayEqual(array, sorted, n));

    int empty[] = {};
    int empty_sorted[] = {};
    n = 0;
    _quicksort(array, n);
    assert(isArrayEqual(empty, empty_sorted, n));

    printf("passed!\n");
}