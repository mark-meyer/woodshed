# Using gcc to compile the .S (uppercase) files
# uses gcc to pre-process the #include and #define statements
# in the assembly
# the -c flag compiles without linking which results in the object (.o) file

UPPEROBJS = primes.o 

ifdef DEBUG
	DEBUGFLGS = -g
else
	DEBUGFLGS = 
endif

all: primes 

%.o : %.S
	gcc $(DEBUGFLGS) -c  $< -o $@


%.o : %.s
	as $(DEBUGFLGS) $< -o $@

primes: $(UPPEROBJS)
	ld -lc -o primes $(UPPEROBJS)

