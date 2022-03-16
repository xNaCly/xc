 # compiler flags
FLAGS := -fdiagnostics-color=always  \
									 -Wall \
									 -Wpedantic \
									 -std=c11 \
									 -Wextra \
									 -Werror \
									 -Wshadow \
									 -Wundef \
									 -fno-common
									 	# use color in diagnostics
										# enables all construction warnings
									 	# enable all warnings demanded by ISO C
										# set of warnings and errors not covered by -Wall
										# all warnings cause errors
										# warnings for shadowing variables
										# warnings for undefined macros
										# warnings for global variables

BUILD_DIR := ./dist
COMPILE := $(FLAGS) xc.c -o $(BUILD_DIR)/xc.out 

# run the previously built executable
run: main
	$(BUILD_DIR)/xc.out *

help: main
	$(BUILD_DIR)/xc.out -h

# compile the executable
main: pre
	gcc $(COMPILE)

pre:
	mkdir -p $(BUILD_DIR)
