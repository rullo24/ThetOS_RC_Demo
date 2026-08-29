#!/usr/bin/env bash
set -e
cargo build
openocd -s /opt/homebrew/share/openocd/scripts \
  -f interface/stlink.cfg -f target/stm32l1.cfg \
  -c "program target/thumbv7m-none-eabi/debug/thetos_rc_demo verify reset exit"
