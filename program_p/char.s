// replicate the CHAR function from MIX
// Given an integer in X0 return bytes for the numbers characters
.data
fstr: 
	.asciz "32\n"

.text

.global _start

_start:
	MOV	X19, #0x320a
        LSL	X19, X19, #8	
	MOV	X0, #1
	MOV	X1, X19
	MOV	X2, #3
	MOV	X8, #64
	SVC	#0

	
	MOV	X0, #0
	MOV	X8, #93
	SVC	#0
	
