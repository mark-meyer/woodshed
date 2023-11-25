// Binary Search Function
// Arguments:
//	X0 - memory location of sorted integer array
//	X1 - length of array
//	X2 - value to search for
// Returns:
//	X0 - index of value or -1 if not found
// Registers:
//	X3 - lower index of search
//	X4 - upper index of search
//	X5 - size of search interval
//	X6 - middle index
//	X7 - value at middle index

.global binary_search

binary_search:
	STP		X29, X30, [sp, #-16]!	// push FP and LR
	MOV		X3, #0					// lower index
	SUB		X4, X1, #1				// upper index
loop:
	SUBS	X5, X4, X3				// length of section
	BMI		_notfound				// subtraction negative, nothing left to search
	LSR		X6, X5, #1				// offset of middle index (half the section length)
	ADD		X6, X6, X3				// add offset to lower for offest relative to section
	LDR		W7, [X0, X6, LSL #2]	// value to compare
	CMP		W2, W7
	BEQ		_found					// we found the value
	BGT		sethigher				// the value we're looking for is bigger
	B		setlower				// the value is smaller
setlower:
	SUB		X4, X6, #1				// since the value is smaller reset upper limit
	B		loop
sethigher:
	ADD		X3, X6, #1				// since the value is higher reset lower limit
	B		loop
_notfound:
	MOV		X0, #-1					// bummer, not found set return value to -1
	B		_end
_found:
	MOV		X0, X6					// set X0 to index to return
	B		_end
_end:
	LDP		X29, X30, [sp], #16		// pop FP and LR
	RET	
