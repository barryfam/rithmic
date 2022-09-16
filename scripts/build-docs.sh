#!/bin/sh

cargo clean \
&& cargo doc --no-deps -p ac-library-rs \
&& cargo doc --no-deps
