#!/bin/sh

unset RELEASE
unset SELFTEST

make
SELFTEST=1 make
RELEASE=1 make
RELEASE=1 SELFTEST=1 make
