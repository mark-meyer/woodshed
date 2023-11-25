// Call Insertion Sort Function

.data
array: .word -100,120, 99109, 1000, 2,-1, 99, 101, 56, 2002
.equ	len, (.-array)/4
fstr: .asciz "%d "
newline: .asciz "\n"
.align 4

.text
.global _start

_start:
	LDR	X0, =array		// pointer to head of array
	MOV	X1, #len		// n = array length
	BL	isort

	// Print array
	MOV	x20, #0			// index to loop
	LDR	X21, =array
	MOV 	X22, #len
printloop:
	CMP	X20, X22
	BGE	_end
	LDR	X0, =fstr
	LDR	W1, [X21, X20, LSL #2]
	BL	printf
	ADD	X20, X20, #1
	B	printloop
	
_end:
	LDR X0, =newline
	BL	printf
	MOV	X8, #93
	SVC	0
