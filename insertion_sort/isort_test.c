#include <stdio.h>
#include <assert.h>
#include <stdbool.h>

extern void isort(int array[], int size );

bool isArrayEqual(int array1[], int array2[], size_t size) {
    for (size_t i = 0; i < size; i++) {
        if (array1[i] != array2[i]) {
            return false;
        }
    }
    return true;
}

int main() {
    int sorted[] =  {-1, 1, 17, 20, 99, 100};
    int array[] = {100, 20, -1, 17, 99, 1};
    int n = 6;

    isort(array, n);
    assert(isArrayEqual(array, sorted, n));

    printf("passed!\n");
}