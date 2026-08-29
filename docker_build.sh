#!/usr/bin/env bash
set -e
docker build --output out "$@" .
echo "firmware -> out/thetos_rc_demo.elf"
