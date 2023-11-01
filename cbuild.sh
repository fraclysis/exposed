#!/bin/bash
cargo build
gcc -O0 -g -o ./build/$1 ./examples/$1.c -lX11 -L/usr/X11R6/lib -I/usr/X11R6/include ./target/debug/libexposed.a ./target/debug/libexposed_gl.a -I./includes -lGL
./build/$1