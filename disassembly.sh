#!/bin/bash
file os/target/riscv64gc-unknown-none-elf/debug/os
rust-readobj -h os/target/riscv64gc-unknown-none-elf/debug/os>asm.txt 
rust-objdump -S os/target/riscv64gc-unknown-none-elf/debug/os>>asm.txt   2>>asm_err.txt  