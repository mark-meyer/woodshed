#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct Rand {
    uint64_t modulus;
    uint64_t multiplier;
    uint64_t increment;
    uint64_t x0;
    int (*next)(struct Rand *self);

} Rand;

extern Rand *_new(uint64_t modulus, uint64_t multiplier, uint64_t increment, uint64_t x0);
extern int _delete(struct Rand *self);

int main() {
    struct Rand *r = _new(10, 7, 7, 7);

    printf("value: %d\n", r->next(r));
    printf("next: %d\n", r->next(r));
    printf("next: %d\n", r->next(r));
    printf("next: %d\n", r->next(r));
    printf("next: %d\n", r->next(r));

    _delete(r);

};