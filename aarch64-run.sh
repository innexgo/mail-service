#!/bin/sh
./target/debug/mail-service \
  --port=8078 \
  --database-url=postgres://ubuntu:toor@localhost/mail \
  --from-address noreply@innexgo.com \
  --dryrun
