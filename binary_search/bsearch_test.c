// Call code from c

#include <stdio.h>
#include <assert.h>

extern int binary_search(int a[], int n, int len);

int main(){
	int a[] = {-9, -5, 0, 1, 2, 4, 9, 22, 55, 105};
	int len = 10;
	
	assert(5 == binary_search(a, len, 4));
	assert(0 == binary_search(a, len, -9));
	assert(9 == binary_search(a, len, 105));
	assert(-1 == binary_search(a, len, 99));

	// empty array
	int b[] = {};
	len = 0;
	assert(-1 == binary_search(b, len, 99));

	printf("Passed!\n");
}
