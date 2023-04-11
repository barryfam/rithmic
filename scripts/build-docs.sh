#!/usr/bin/env fish

cargo clean \
&& cargo doc --no-deps -p ac-library-rs \
&& cargo doc --no-deps

cd (dirname ( status --current-filename ))/..
lychee README.md
