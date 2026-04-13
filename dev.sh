#!/bin/bash

anchor build --no-idl

RUSTUP_TOOLCHAIN=nightly-2024-11-01 anchor idl build -p drishti_core
RUSTUP_TOOLCHAIN=nightly-2024-11-01 anchor idl build -p drishti_registry

anchor test --skip-build