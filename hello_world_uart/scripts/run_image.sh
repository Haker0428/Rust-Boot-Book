#! /bin/bash

qemu-system-aarch64 -machine raspi3 -nographic -kernel ../target/aarch64-unknown-none/debug/hello_world_uart
