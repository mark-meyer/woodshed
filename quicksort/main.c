#include <stdio.h>

extern void _quicksort(int array[], int size ) asm("_quicksort");


int main() {
    int array[] = {100, 20, -1, 20, 17, 99, 20, 1};
    int n = 8;

    _quicksort(array, n);

    for (int i=0; i < n; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
}