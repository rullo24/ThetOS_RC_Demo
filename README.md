# ThetOS RC Demo

Firmware built on [ThetOS](https://github.com/rullo24/ThetOS) v1.0.0. Target board: ST Nucleo-L152RE.

## Build (Docker)

```
./docker_build.sh
```

Produces `out/thetos_rc_demo.elf`. Debug build: `./docker_build.sh --build-arg PROFILE=debug`.

Needs Docker with BuildKit (Docker Desktop has it by default).

## Flash

Flashing is done on the host — a container cannot reach the USB probe.

```
ELF=out/thetos_rc_demo.elf ./macos_flash.sh
```

Needs OpenOCD on the host, and the board connected by USB (its on-board ST-Link is the probe). The green user LED should start blinking.

## Build and flash without Docker

```
./macos_flash.sh            # debug
./macos_flash.sh --release
```

Builds locally (needs `rustup`) and flashes in one step.

## Controlling the Car

The car takes single-character commands over USART2 (115200 baud, 8N1). The Nucleo's on-board ST-Link exposes this as a USB serial port, so no extra wiring is needed. No Enter key is needed: each keypress is one command.

Find the port, then connect:

```
ls /dev/cu.usbmodem*
screen /dev/cu.usbmodem<N> 115200
```

Exit `screen` with `Ctrl-A`, then `K`, then `y`.

On reset the car prints:

```
rc ready: w/a/d/s drive, +/- both, 1/2 left, 3/4 right
```

| Key | Action |
|---|---|
| `w` | Drive forward |
| `a` | Turn left (left wheel stopped, right wheel driving at reduced speed) |
| `d` | Turn right (right wheel stopped, left wheel driving at reduced speed) |
| `s` or space | Stop |
| `+` / `-` | Both motors faster / slower (about 1% per press) |
| `1` / `2` | Left motor trim down / up |
| `3` / `4` | Right motor trim down / up |

Replies:

- Drive and stop keys echo the same character as an acknowledgement.
- Trim keys print the new duty, for example `left: 62%`.
- Any other key prints `byte: <value>`, so a wrong keystroke is visible.

If the UART reports a receive error, the car stops.

Close the serial session before flashing, since an open session can hold the port.
