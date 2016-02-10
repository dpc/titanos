#!/bin/bash

unset RELEASE
unset SELFTEST

make || exit 1
SELFTEST=1 make || exit 1
RELEASE=1 make || exit 1
RELEASE=1 SELFTEST=1 make || exit 1
