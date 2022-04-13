#!/bin/sh
mkdir aarch64
cross build --release --target aarch64-unknown-linux-gnu && cp ./target/aarch64-unknown-linux-gnu/release/mail-service ./aarch64/mail-service
