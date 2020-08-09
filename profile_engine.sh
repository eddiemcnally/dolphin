!#/bin/bash

valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ./target/release/dolphin
