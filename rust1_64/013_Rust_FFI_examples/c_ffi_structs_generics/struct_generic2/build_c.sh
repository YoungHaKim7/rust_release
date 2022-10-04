#!/bin/bash
LIBS="-ldl -lrt -lpthred -lgcc_s -lc -lm -lrt -lpthread -lutil"
cc program.c target/debug/libstruct_basic.a -o program -g $LIBS