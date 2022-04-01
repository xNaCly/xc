 # compiler flags
FLAGS := -fdiagnostics-color=always  \
									 -Wall \
									 -Wpedantic \
									 -std=c99 \
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

run: main
	$(BUILD_DIR)/xc.out *

help: main
	$(BUILD_DIR)/xc.out -h

version: main
	$(BUILD_DIR)/xc.out -v

install: main
	cp ./xc.1 ./xc.1_copy
	gzip ./xc.1_copy
	mv ./xc.1_copy.gz ./xc.1.gz
	sudo mkdir -p /usr/local/share/man/man1
	sudo mv xc.1.gz /usr/local/share/man/man1/xc.1.gz
	sudo mv $(BUILD_DIR)/xc.out /usr/local/bin/xc

uninstall:
	sudo rm /usr/local/man/man1/xc.1.gz
	sudo rm /usr/local/bin/xc

main: pre
	gcc $(COMPILE)

debug: pre
	gcc -g3 $(COMPILE)

pre:
	mkdir -p $(BUILD_DIR)
