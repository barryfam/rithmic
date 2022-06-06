#!/bin/sh

cargo clean --doc \
&& cargo doc --no-deps -p ac-library-rs \
&& cargo doc --no-deps -p euclid \
&& cargo doc --no-deps -p primal \
&& cargo doc --no-deps
