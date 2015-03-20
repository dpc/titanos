#!/bin/bash

git clone git://git.qemu.org/qemu.git qemu.git
cd qemu.git
git submodule update --init
./configure --target-list=aarch64-softmmu
make
