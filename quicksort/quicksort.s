// In-place quicksort implementation 
// using Hoare partitioning 
// Parameters:
// X0 reference to array to be sorted
// X1 numberof elements
// Calling from c:
// extern void quicksort(int array[], int size );


.global _quicksort
.text

_quicksort:
    // Parameters:
    // X0: reference to array to be sorted
    // X1: length of array
    STP X29, X30, [sp, #-16]!   // store fp and lr
    STR X20, [sp, #-16]!        // save register 20
    SUB X2, X1, #1              // high index: last element in array 
    MOV X1, #0                  // low index
    MOV X20, X0                 // store array pointer in non-volitile register

    BL _sort_recurse

    LDR X20, [sp], #16          // restore X20
    LDP X29, X30, [sp], #16     // restore args from stack
    RET

_sort_recurse:
    // Parameters
    // X0: reference to array
    // X1: low index 
    // X2: high index
    STP X29, X30, [sp, #-16]!   // store fp and lr
    STR X21, [sp, #-16]!        // save register 21
    STP X1, X2, [sp, #-16]!     // store argumnets for high, low
    
    CMP X1, X2

    B.GE _end

    MOV X0, X20                 // set first argument to stored array pointed

    BL partition                // partition will retrurn the pivot index in X0
    MOV X21, X0                 // save pivot it in X21 so we can restore X0

    // recurse on partition low to mid
    MOV X0, X20                 // reset X0 back to saved array pointer
    LDR X1, [sp, #0]            // low index
    MOV X2, X21                 // pivot index
    //SUB X2, X2, #1            // subtract
    BL _sort_recurse

    // recurse on partition mid to high
    MOV X0, X20                 // reset X0 back to saved array pointer
    MOV X1, X21                 // pivot index
    ADD X1, X1, #1
    LDR X2, [sp, #8]            // high index
    BL _sort_recurse

    B _end

_end:
    LDP X1, X2, [sp], #16
    LDR X21, [sp], #16
    LDP X29, X30, [sp], #16
    RET


partition:
    // Parameters:
    // X0: reference to an array of integers
    // X1: the low index
    // X2: the high index

    STP X29, X30, [sp, #-16]!   // store fp and lr
    LDR W3, [X0, X1, LSL #2]    // value of low element used as pivot
    SUB X1, X1, #1              // offset low to allow an increase in each loop
    ADD X2, X2, #1              // offset high to allow a decrease in each loop

loop:
    ADD X1, X1, #1              // with each iteration move low up
    SUB X2, X2, #1              // and high down
next_left:                      // find the next index from left that is greater than or equal to the pivot
    LDR W4, [X0, X1, LSL #2]    // next item from left
    CMP W4, W3                  // compare to pivot
    B.GE next_right             // found one, break
    ADD X1, X1, #1              // increase index
    B next_left                 // continue

next_right:                     // find the next index from right that is less than the pivot
    LDR W4, [X0, X2, LSL #2]    // next item from right
    CMP W4, W3                  // compare to pivot
    B.LE continue_partition     // found one, break
    SUB X2, X2, #1              // decrease index
    B   next_right              // keep going

continue_partition:
    cmp X1, X2                  // compare low to high
    B.GE end_partition          // when low is greater than or equal to high it's done
    
    // swap the values at the two indices
    LDR W4, [X0, X1, LSL #2]
    LDR W5, [X0, X2, LSL #2]
    STR W4, [X0, X2, LSL #2]
    STR W5, [X0, X1, LSL #2]
    B loop

end_partition:
    MOV X0, X2                  // return the pivot index
    LDP X29, X30, [sp], #16
    RET
