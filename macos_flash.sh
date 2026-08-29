#!/usr/bin/env bash
set -e

BIN=thetos_rc_demo
PROFILE=debug
CARGO_RELEASE=""
OPENOCD_SCRIPTS="${OPENOCD_SCRIPTS:-/opt/homebrew/share/openocd/scripts}"

case "${1:-}" in
  -r|--release) PROFILE=release; CARGO_RELEASE="--release" ;;
  -h|--help)    echo "usage: $0 [--release]   (or set ELF=path to flash a prebuilt binary)"; exit 0 ;;
  "")           ;;
  *)            echo "unknown option: $1" >&2; exit 1 ;;
esac

# ELF overrides everything: flash a prebuilt binary (e.g. the Docker output) without building
if [ -z "${ELF:-}" ]; then
  cargo build $CARGO_RELEASE
  ELF="target/thumbv7m-none-eabi/$PROFILE/$BIN"
fi

openocd -s "$OPENOCD_SCRIPTS" \
  -f interface/stlink.cfg -f target/stm32l1.cfg \
  -c "program $ELF verify reset exit"
