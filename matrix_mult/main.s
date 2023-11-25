// Multiply 2 3x3 matrices
// FROM: "Programming with 64-bin ARM Assembly" 

.global _start
	.equ 	N, 3			// Matrix dimension
	.equ 	WDSIZE, 4		// Size of the elements

_start:
	STR	LR, [SP, #-16]!		// save link register
	STP	X19, X20, [SP, #-16]!	// save nonvolotile regsiters

	MOV	W1, #N			// Row index
	LDR	X4, =A			// Address of current row
	LDR	X19, =C			// Address of result matric
rowloop:
	LDR	X5, =B			// First column of B
	MOV	W2, #N			// Column index counting down to 0
colloop:
	// zero accumulator register
	MOV	X7, #0

	MOV	W0, #N			// dot product loop counter
	MOV	X12, X4			// Row for dot product
	MOV	X6, X5			// Column for dot product
dotloop:
	// Do dot prodcut of a row of A with a column of B
	LDR	W9, [X12], #WDSIZE	// Load A[row, i] and incement
	LDR	W10, [X6], #(N*WDSIZE)	// load B[i, col]
	SMADDL	X7, W9, W10, X7		// multiply and accumulate into X7
	SUBS	W0, W0, #1		// decrement loop counter
	B.NE	dotloop			// if not zero loop

	STR	W7, [X19], #4		// store C[row, col]
	ADD	X5, X5, #WDSIZE		// Increase column
	SUBS	W2, W2, #1		// decrease col loop count
	B.NE	colloop			// if not zero next col
	
	ADD	X4, X4, #(N*WDSIZE)	// increase to next row
	SUBS	W1, W1, #1		// decrement row loop counter
	B.NE	rowloop			// branch to next row if not zero

// Print out Matrix C
// Loop through 3 rows printing three cols each time
	MOV	W20, #3			// print three rows
	LDR	X19, =C			// address of result matrix
printloop:
	LDR	X0, =prtstr		// printf format string
	LDR	W1, [X19], #WDSIZE	// First element of current row
	LDR	W2, [X19], #WDSIZE	// Second element
	LDR	W3, [X19], #WDSIZE	// Third element
	BL	printf
	SUBS	W20, W20, #1		// dec loop counter
	B.NE	printloop		// if not zero keep printing

	MOV	X0, #0			// return code
	LDP	X19, X20, [SP], #16	// restore registers
	LDR	LR, [SP], #16

	MOV 	X0, #0
	MOV 	X8, #93
	SVC	0
.data
// first matrix
A:	.word	1,2,3
	.word	4,5,6
	.word	7,8,9
// second matrix
B:	.word	9,8,7
	.word	6,5,4
	.word	3,2,1
// Result
C:	.fill	9, 4, 0

prtstr: .asciz	"%3d %3d %3d\n"
	


