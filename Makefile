# Set you prefererred CFLAGS/compiler compiler here.
# Our github runner provides gcc-10 by default.

.PHONY: all compile check


# this target should build all executables for all tests
all: | build/Makefile
	$(MAKE) compile


compile:
	$(MAKE) -C build db_bench


build/Makefile: | build
	cd build \
	&& cmake ../rocksdb/ -DFAIL_ON_WARNINGS=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo


build:
	mkdir -p $@


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
