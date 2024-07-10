#!/bin/sh

# Set the sysroot to the root of your cross-compiler's environment.
SYSROOT=$(pwd)/arm-buildroot

# PKG_CONFIG_DIR is left empty.
export PKG_CONFIG_DIR=

# PKG_CONFIG_LIBDIR includes paths to the pkg-config directories within the sysroot.
export PKG_CONFIG_LIBDIR=${SYSROOT}/usr/lib/pkgconfig:${SYSROOT}/usr/share/pkgconfig

# PKG_CONFIG_SYSROOT_DIR specifies the sysroot directory.
export PKG_CONFIG_SYSROOT_DIR=${SYSROOT}

# PKG_CONFIG_ALLOW_CROSS=1 tells pkg-config that cross-compiling is allowed.
export PKG_CONFIG_ALLOW_CROSS=1

# Set PKG_CONFIG_PATH to ensure pkg-config finds the .pc files
export PKG_CONFIG_PATH=${PKG_CONFIG_LIBDIR}

# Debug output
echo "PKG_CONFIG_LIBDIR: $PKG_CONFIG_LIBDIR"
echo "PKG_CONFIG_SYSROOT_DIR: $PKG_CONFIG_SYSROOT_DIR"
echo "PKG_CONFIG_PATH: $PKG_CONFIG_PATH"

# Check if pkg-config can find libudev
PKG_CONFIG_LIBDIR=${PKG_CONFIG_LIBDIR} \
PKG_CONFIG_SYSROOT_DIR=${PKG_CONFIG_SYSROOT_DIR} \
PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 \
PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 \
PKG_CONFIG_PATH=${PKG_CONFIG_PATH} \
pkg-config --libs --cflags --debug libudev

# Ensure the correct linker is used
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabihf-gcc

# Run cargo build, which will use the above environment variables when invoking pkg-config.
cargo build --target=armv7-unknown-linux-gnueabi --release

