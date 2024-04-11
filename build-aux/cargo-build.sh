#!/bin/sh
#
# This script can be used to automatically pack the resources
# and locales and place them in the default cargo-build
# compilation directory (a.k.a. target), if you don't want to
# use meson.

cd "$(dirname "$0")/.."

cargo build

meson_opts="$@"
if [ -d build ]; then
    meson_opts="--reconfigure $meson_opts"
fi

ninja -C build data/request-maker.gresource
ninja -C build request-maker-gmo
mkdir -p target/share/request-maker
cp -R build/data/request-maker.gresource target/share/request-maker
cp -R build/po target/share/locale