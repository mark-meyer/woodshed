SORTJOBS = main.o sort.o

all: main

$.o : %.S
	gcc -g -c $< -o $@

%.o : %.s
	as -g $< -o $@

main: $(SORTJOBS)
	ld -lc -o main $(SORTJOBS)
