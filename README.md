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
