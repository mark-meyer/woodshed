.global newnode
.global prependnode
.global fromarray
.global reverselist

	.equ NULL, 0
	.equ value, 0
	.equ next,  4
	.equ size,  12

.text

newnode: 
	// argument X0: integer to value of node
	// returns X0: a pointer to node in memory with value and null next
	STP	X29, X30, [sp, #-16]!	// push FR and LR to stack
	STR X0, [sp, #-16]!			// push argument to stack

	MOV X0, #size				// allocate memory for node
	BL	malloc					// node address is now in X0
	
	LDR W9, [sp], #16			// pop argument off stack
	STR	W9, [X0]				// store argument value in node
	MOV X1, #NULL				// Null for next pointer
	STR X1, [X0, #next]
	LDP X29, X30, [sp], #16     // Restore FP, LR from stack
	RET

prependnode: 
	// Makes passed in node new head of list
	// arguments:
	// X0: node to prepend to list
	// X1: head node of list
	// Returns: X0 the passed in node with next value pointing to original
	STP	X29, X30, [sp, #-16]!	// push FR and LR to stack

	STR	X1, [X0, #next]			// link next value of first node to second

	LDP X29, X30, [sp], #16     // Restore FP, LR from stack
	RET

fromarray:
	// create a linked list from array in memory
	// arguments:
	// X0: memory location of array
	// X1: length in bytes of array
	// Returns
	// X0: memory location of first node in list
	STP		X29, X30, [sp, #-16]!	// push FR and LR to stack
			// save registers list an len on stack
	
	MOV		X20, X0					// save array  
	MOV		X21, X1					// save len
	SUB		X21, X21, #1			// get index of last element from arg
	LDR     W0, [X0, X21, LSL #2]   // value for node
    BL      newnode					// new node will be in X0
loop:
    SUBS    X21, x21, #1			// decrement counter 
    B.MI   _endfromarray			// end if negative
    MOV     X22, X0					// previous head of list
    LDR     W0, [X20, X21, LSL #2]	// get new value
    BL      newnode					// new head
    MOV     X1, X22					// put previous head in register for second arg
    BL      prependnode				// link previous head to new head
    B       loop

_endfromarray:
	LDP X29, X30, [sp], #16     	// Restore FP, LR from stack
	RET


reverselist:
	MOV 	X21, #NULL				// previous node
_reverseloop:
	LDR		X22, [X0, #next]		// load next node
	STR 	X21, [X0, #next]		// set current next to previous
	CMP		X22, #NULL				// if next node is null break
	B.EQ	_endreverse
	MOV		X21, X0					// current becomes new previous
	MOV		X0, X22					// next beomes new current
	B		_reverseloop

_endreverse:
	RET
