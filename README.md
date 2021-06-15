# Performance

This weeks exercise requires you to optimize RocksDB. RocksDB is a high-performance persistent LSM-based key value store. We introduced changes changes to make the performance of rocksdb worse in throughput and latency.

Your task is to get as close to unmodified RocksDB performance as possible with the rocksdb version in the directory ```rocksdb```.

## Deliveries

The execution of the benchmark ```db_bench``` has to be correct and as fast as possible.


## General Information

1. For building run 
   ```console
   $ make all
   # or for faster building
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
4. Except the tests all other configuration take a ```NUM``` and ```THREADS``` parameter to configure the tests to your system e.g.:
   ```console
   $ make run NUM=10000 THREADS=4
   ```
   However, the final test will use the predefined numbers
