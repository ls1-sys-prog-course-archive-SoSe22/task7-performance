# Performance

This weeks exercise requires you to optimize [RocksDB](https://rocksdb.org/). RocksDB is a high-performance persistent LSM-based key value store [pdf](https://www.cs.umb.edu/~poneil/lsmtree.pdf). We introduced changes changes to make the performance of rocksdb worse in throughput and latency.

Your task is to get as close to unmodified RocksDB performance as possible with the rocksdb version in the directory `rocksdb`, by finding the changes and fixing them.

## Deliveries

The execution of the benchmark `db_bench` has to be correct and as fast as possible.


## General Information

1. For building run 
   ```console
   $ make all
   # or for faster build time
   $ make all -j$(./cpu.sh | grep "logical" | awk '{print $NF}')
   ```
2. For running db_bench run:
   ```console
   $ make run
   ```
3. For running the tests run:
   ```console
   $ make check
   ```
4. Except the tests all other configuration take a `NUM` and `THREADS` parameter to configure the tests to your system e.g.:
   ```console
   $ make run NUM=10000 THREADS=4
   ```
   However, the final test will use `NUM=50000` and `THREADS=16`
   
 ## Tips
 
 Use the tools presented on Monday, especially [perf](http://www.brendangregg.com/perf.html) and [flamegraph](http://www.brendangregg.com/flamegraphs.html). You can either adopt the `Makefile` or copy the run command from the `Makefile` to run `db_bench` with perf.
 Perf and flamegraphs also support different modes, some of them might be useful.
 
 The changes we introduced should be obvious, even if you do not know C++. Thus no major changes to the source code should be required.
 
 ## References
 [RocksDB](https://rocksdb.org/)  
 [LSM Datastructure](https://www.cs.umb.edu/~poneil/lsmtree.pdf)  
 [Perf](http://www.brendangregg.com/perf.html)  
 [Flamegraph](http://www.brendangregg.com/flamegraphs.html)  
 [Intel VTunes](https://software.intel.com/content/www/us/en/develop/tools/oneapi/components/vtune-profiler.html#gs.3hg9lw)  
 [AMD μProf](https://developer.amd.com/amd-uprof/)  
 [Gperftools](https://gperftools.github.io/gperftools/)  
 [heaptrack](https://github.com/KDE/heaptrack)  
 [Valgrind](https://valgrind.org/)  
 [Google Orbit](https://github.com/google/orbit)  
 [GNU gprof](https://sourceware.org/binutils/docs/gprof/)  
 [Google Benchmark](https://github.com/google/benchmark)  
 [QuickBench](https://quick-bench.com/)  
 [CMake](https://cmake.org/)  
 [CppRefence](https://en.cppreference.com/w/)  
 [C++ Core Guidlines](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines)  
