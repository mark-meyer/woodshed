// Insertion Sort: sorts an array of 32 bit intergers in place.
// Following this algorithm:
// sort(A):
//  N = len(A)
//  i = 1
//  while i < n:
//    current = A[i]
//    j = i - 1
//    while j >= 0 and A[j] > current:
//      A[j + 1] = A[j]
//      j = j - 1
//    A[j + 1] = current
//    i = i + 1

// REGISTERS:
// X0: array memory location
// X1: lengh of the array
// X2: outer loop index = i
// W3: current
// X4: inner loop index = j
// X5: j + 1 for indexing A
// W6: value at A[j]


	.type isort, %function
	.global isort

isort:
	MOV		X2, #1					// i index of current element to sort

outerloop:
	CMP		X2, X1					// exit if loop is over
	BGE		return
	LDR		W3, [X0, X2, LSL #2]	// current, the value being sorted
	SUB 	X4, X2, #1				// j inner loop index = i - 1
innerloop:
	LDR 	W6, [X0, X4, LSL #2]	// A[j] the value we are comparing
	CMP		W6, W3					// while A[j] > current end loop 
	BLE		setorder
	ADD		X5, X4, #1				// j+1
	STR		W6, [X0, X5, LSL #2]   	// A[j+1] = A[j]
	SUBS	X4, X4, #1 				// j = j - 1
	B.PL	innerloop				// loop if index is positiv
setorder:
	ADD		X6, X4, #1				// j + 1
	STR		W3, [X0, X6, LSL #2] 
	ADD		X2, X2, #1				// i = i + 1
	B		outerloop
return:
	ret
	.size	isort, (. - isort)
