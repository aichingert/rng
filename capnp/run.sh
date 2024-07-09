#!/bin/sh

zig c++ -std=c++20 -Wall server.cpp hello.capnp.cpp \
    $(pkg-config --cflags --libs capnp-rpc) -o server

zig c++ -std=c++20 -Wall client.cpp hello.capnp.cpp \
    $(pkg-config --cflags --libs capnp-rpc) -o client
