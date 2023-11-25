// Linked List with integer values
.data
prtfmt: .asciz "%d "
newline: .asciz "\n"
array:	.word 1, 2, 3, 4, 5, 6, 7, 8
	.equ len, (. -array)/4
.align 4
.global _start

.text

_start:
    LDR     X0, =array
    MOV     X1, #len
    BL      fromarray
    BL      reverselist
    MOV     X20, X0

_printlist:
    CMP     X20, #0
    B.EQ    _printend
    LDR     X0, =prtfmt
    LDR     W1, [X20]
    BL      printf
    LDR     X20, [X20, #4]
    BL      _printlist

_printend:
    LDR     X0, =newline
    BL      printf


_end:
    MOV     X0, #0
    MOV     X8, #93
    SVC     0
