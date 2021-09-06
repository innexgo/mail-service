#!/bin/bash
./target/debug/mail-service \
  --port=8078 \
  --database-url=postgres://postgres:toor@localhost/mail \
  --from-address noreply@innexgo.com \
  --dryrun
