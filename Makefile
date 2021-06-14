# Set you prefererred CFLAGS/compiler compiler here.
# Our github runner provides gcc-10 by default.

.PHONY: all compile check

NUM?=200000
#THREADS?=$(shell ./cpu.sh | grep "logical" | awk '{print $$NF}')
THREADS?=16

# this target should build all executables for all tests
all: | build/Makefile
	$(MAKE) -C build db_bench


build/Makefile: | build
	cd build \
	&& cmake ../rocksdb/ -DFAIL_ON_WARNINGS=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo -DLS1_QUIET=1


build:
	mkdir -p $@

run_small: NUM=10000
run_small: run

run: all
	./build/db_bench -flagfile tests/config --num=$(NUM) --threads=$(THREADS)

# C example:
#all:
#	$(CC) $(CFLAGS) -o task-name task-name.c

# C++ example:
#all:
#	$(CXX) $(CXXFLAGS) -o task-name task-name.cpp

# Rust example:
#all:
#	$(CARGO) build --release

# Usually there is no need to modify this
check: all
	$(MAKE) -C tests check
