#!/usr/bin/env bash

mkdir build
cd build
cmake -DCMAKE_TOOLCHAIN_FILE=../cmake-framework/Arduino-Toolchain.cmake  .. 
cmake -DCMAKE_TOOLCHAIN_FILE=../cmake-framework/Arduino-Toolchain.cmake  .. 
