// 1.3.2 pg 145
// Translation of MIX program for maximum
//
// Find the max. Given n elements X[1],X[2], ... X[n], find
// m and j such that m = X[j] = max(1 <= i <= n) X[i] where j
// is the largest index satisfying this condition
//
// M1. [initializ] Set j <- n, k <- n, m <- X[n]
// M2. [All tested?] if k = 0, terminate
// M3. [Compare] if X[k] <= m goto M5
// M4. [Change M] Set j <- k, m <- X[k] (new max value)
// M5. [Decrease k] decrease k by one and goto M2.

// Registers
// X0 - current max and return valu
// X1 - array pointer
// X3 - k = loop count starting an n counting backward
// X2 - j = index of current max

.global _start

_start: 
	LDR 	X1, =bytes	// CONTENT
	MOV	X3, #(n-1)	// INIT k <- n
	B	enter		// JMP CHANGEM
loop:
	LDRB	W5, [X1, X3]	// get value at offset  	
	CMP	W0, W5 		// M3 compare
	B.gt	changem		
enter:
	MOV	X2, X3		// M4 change M
	LDRB	W0, [X1, X2]
changem:
	SUBS	X3, X3, #1	// M5 decrease k
	B.PL	loop		

// Exit. leave max value in X0 for return code 
exit:
	MOV	X8, #93
	SVC	0
	
.data
bytes: .byte 223, 221,222, 100, 220, 190, 11, 2 
n: .word .-bytes
