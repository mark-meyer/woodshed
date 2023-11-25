ifdef DEBUG
DEBUGFLGS = -g
else
DEBUGFLGS =
endif

all: bsearch

bsearch: bsearch.c
	gcc -o bsearch  $(DEBUGFLGS) search.s bsearch.c

test: bsearch_test.c
	gcc -o test $(DEBUGFLGS) search.s bsearch_test.c
	./test
