// Implementation of Linear Congruential Methed Random Number Generator
// as descibed in Knuth TAOCP vol c
// exports _new and _delete functions


.data
.equ    s_m, 0                  // modulus 0 < m
.equ    s_a, 8                  // multiplier 0 <= a < m
.equ    s_c, 16                 // increment 0 <= c < m
.equ    s_x_0, 24               // the starting value 0 <= x_0 < m
.equ    s_next, 32              // next function pointer
.equ    s_size, 40

.global _new
.global _delete

.text
_new:
    // creates a rand structure in memory
    // arguments:
    // X0: m
    // X1: a
    // X2: c
    // X3: x_0

    STP X29, X30, [sp, #-16]!
    STP X0, X1, [sp, #-16]!     // store arguments on stack 
    STP X2, X3, [sp, #-16]!     // before calling malloc

    MOV X0, #s_size             // allocate memory for stuct
    bl malloc
    MOV X6, X0
    
    LDP X2, X3, [sp], #16
    LDP X0, X1, [sp], #16

    ADR X4, _next               // function address
    STR X0, [X6, #s_m]          // store m in struct
    STR X1, [X6, #s_a]
    STR X2, [X6, #s_c]
    STR X3, [X6, #s_x_0]
    STR X4, [X6, #s_next]       // store function in struc
    
    MOV X0, X6
    LDP X29, X30, [sp], #16
    RET

_next:
    // Create the next number in the cycle, stores it, 
    // and returns the current number
    // XO should be a reference to self
    // the memory location of the struct    
    STP X29, X30, [sp, #-16]!
    
    LDR X1, [X0, #s_m]      // m
    LDR X2, [X0, #s_a]      // a
    LDR X3, [X0, #s_c]      // c
    LDR X6, [X0, #s_x_0]    // X0

    MUL X4, X6, X2          // a * X0
    ADD X4, X4, X3          // a * X0 + c

    // get modulus (a * X0 + c) mode m
    UDIV X5, X4, X1         // (a * X0 + c) / m == X4 / X1
    MSUB X4, X5, X1, X4     // X4 now the next value

    STR X4, [X0, #s_x_0]    // store X4 in stuct as X0

    MOV X0, X6              // return the original X0

    LDP X29, X30, [sp], #16
    RET

_delete:
    // frees the structure memory
    // XO is a refernce to to the structure
    STP X29, X30, [sp, #-16]!
    bl free
    MOV X0, #0
    LDP X29, X30, [sp], #16
    RET

