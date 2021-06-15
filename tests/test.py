#!/usr/bin/env python3

import sys

from testsupport import info, run_project_executable, warn

from typing import Tuple

def get_second_entry(line: str) -> int:
    parts = line.split()
    return int(parts[1])

def open_run(run_file: str) -> Tuple[bool, float, int]:
    res = True
    latency = float('inf')
    ops = 0
    with open(run_file, "r") as f:
        for line in f.readlines():
            line = line.strip()
            if line.startswith("Keys:") and get_second_entry(line) != 16:
                info("Changed config key size is not 16 bytes")
                return (False, latency, ops)
            if line.startswith("Values:") and get_second_entry(line) != 100:
                info("Changed config value size is not 100 bytes")
                return (False, latency, ops)
            if line.startswith("Prefix:") and get_second_entry(line) != 0:
                info("Changed config prefix size is not 0 bytes")
                return (False, latency, ops)
            if line.startswith("Memtablerep:") and line.split()[1] != "skip_list":
                info("Changed config memtable is not skip_list")
                return (False, latency, ops)
            if line.startswith("RandomTransactionVerify") and line.split()[1] != "Success.":
                info("Failed correctness test")
                return (False, latency, ops)
            if line.startswith("randomtransaction :"):
                parts = line.split()
                latency = float(parts[2])
                ops = int(parts[4])
        return (True, latency, ops)


def test(idx: int) -> bool:
    optimized = open_run("cache-optimized")
    if not optimized[0]:
        return  False
    user = open_run("cache-run")
    if not user[0]:
        return False

    limit = (40 / (optimized[2] * 0.9)) * user[2]
    print(limit)
    if limit < idx:
        return False
    return True


def main() -> None:
    # Replace with the executable you want to test
    try:
       # info("Run someprojectbinary...")
       # run_project_executable("someprojectbinary")
       if not test(int(sys.argv[1])):
           sys.exit(1)
    except OSError as e:
        warn(f"Failed to run command: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
