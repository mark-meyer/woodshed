# Using gcc to compile the .S (uppercase) files
# uses gcc to pre-process the #include and #define statements
# in the assembly
# the -c flag compiles without linking which results in the object (.o) file

UPPEROBJS = maximum.o 

ifdef DEBUG
	DEBUGFLGS = -g
else
	DEBUGFLGS = 
endif

all: maximum 

%.o : %.S
	gcc $(DEBUGFLGS) -c  $< -o $@


%.o : %.s
	as $(DEBUGFLGS) $< -o $@

maximum: $(UPPEROBJS)
	ld -o maximum $(UPPEROBJS)

