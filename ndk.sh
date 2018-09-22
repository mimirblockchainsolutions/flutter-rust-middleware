#!/bin/bash

rm -rf NDK
mkdir NDK

${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm64

export PATH=`pwd`/NDK/arm64/bin:$PATH

${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir NDK/arm

export PATH=`pwd`/NDK/arm/bin:$PATH

${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir NDK/x86

export PATH=`pwd`/NDK/x86/bin:$PATH

