#!/usr/bin/env bash
set -euxo pipefail

: Compile the C++ library
g++ -fPIC -c example.cpp
: Create a static library
ar src libexample.a example.o
: The library should contain an unmangled init and volume functions:
nm libexample.a
: FIN
