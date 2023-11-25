SORTJOBS = $(BUILD_DIR)/quicksort.o $(BUILD_DIR)/main.o
TESTJOBS = qsort_test.c $(BUILD_DIR)/quicksort.o
BUILD_DIR = build

ifdef DEBUG
DEBUGFLGS = -g
else
DEBUGFLGS =
endif

.PHONY: all
all:  $(BUILD_DIR)/main

# make build directory
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# primary target
$(BUILD_DIR)/main:  $(SORTJOBS) | $(BUILD_DIR)
	gcc -o $@ $^

# build object files from assembly
$(BUILD_DIR)/%.o : %.s | $(BUILD_DIR)
	as $(DEBUGFLGS) $< -o $@

# build main object
$(BUILD_DIR)/main.o: main.c  | $(BUILD_DIR) 
	gcc -c -o $@ $< 

.PHONY: test
test: $(TESTJOBS) | $(BUILD_DIR)
	gcc -g -o $(BUILD_DIR)/$@ $(DEBUGFLGS) $(TESTJOBS)
	./$(BUILD_DIR)/test

.PHONY: clean
clean: 
	rm -r $(BUILD_DIR)
