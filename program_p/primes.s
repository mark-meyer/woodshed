	.equ	L, 500
.data
prime:
	.space 2000	// space for 500 integers
buff:
	.space 40	// space for print buffer
printfmt:
	.asciz "%lu "
newline:
	.asciz "\n "

.text
.global _start
.align 2

_start:
	STP	X29, X30, [sp, #-16]!
	LDR	X19, =prime		// memory location of prime array
	MOV	X1, #0			// index relative to prime
	MOV	W2, #3			// Potential prime

newprime:
	ADD	X1, X1, #1		// increment index
	STR	W2, [X19, X1, lsl#2]    // found a prime, store it at index X1
	CMP	X1, #L-1		// if we've found them all stop looking
	B.eq	print
nexttest:
	ADD	W2, W2, #2		// get next potential prime (inc by 2)
	MOV	X3, #1			// X3 will be the index into the known primes
testprime:
	LDR	W6, [X19, X3, lsl #2]   // load a known prime indexd by X3
	UDIV	X4, X2, X6		// test our potential prime by dividing by known prime
	MSUB	X5, X4, X6, X2		// get the remained
	CMP	X5, #0			// if remainder is 0 it's not prime
	B.eq	nexttest		// jump to get a new potential prime
	CMP	X4, X6			// might be prime but still need to test 
	ADD	X3, X3, #1		// increment index into known primes
	B.gt	testprime		// test this one
	B	newprime
print:
	// not going to bother with buffered output like taocp
	// but DO print in comlumns

	// taocp is tricky here. It depends on counting down in increments of 50
	// from 451, until the cycle matches up and you hit zero
	// it's probably easier to write a regular loop.
	// X21 starts at -50, but immediatel adds 501 to is making it 451
	// this is an index into the array of primes
	MOV	X21, #-50		// Ri5 in taocp
fillbuff:
	ADD	X21, X21, #(L+1)
nextfill:
	LDR	X0, =printfmt
	SUB	X21, X21, #1		// taocp allows loading with a PRIMES offset of -1. 
	LDR	W1, [X19, X21, lsl #2]  // not sure how to do this other than modify it
	ADD	X21, X21,#1 
	BL	printf	
	SUB	X21, X21, #50
	CMP	X21, #0
	B.GT	nextfill	
	LDR	X0, =newline
	BL 	printf
	CMP	X21, #0
	B.NE	fillbuff	
quit:
	LDP	X29, X30, [sp], #16
	MOV	X0, #0			// zero return code
	MOV	X8, #93			// service code exit
	SVC	0

