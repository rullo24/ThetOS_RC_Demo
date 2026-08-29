# syntax=docker/dockerfile:1
# Build-only firmware image; flash on the host, not in the container.
# Usage: docker build --output out .   ->   out/thetos_rc_demo.elf

FROM rust:1.98-bookworm AS build

ARG PROFILE=release   # release | debug

# embedded target + cargo-binutils (cargo size / objdump); not in the base image
RUN rustup target add thumbv7m-none-eabi \
 && rustup component add llvm-tools-preview \
 && cargo install cargo-binutils

WORKDIR /app
COPY . .

# cache mounts persist between builds; copy the ELF out within the RUN (mounts aren't in the image)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    set -eux; \
    case "$PROFILE" in \
      release) FLAG=--release; DIR=release ;; \
      debug)   FLAG=;          DIR=debug ;; \
      *) echo "PROFILE must be release or debug" >&2; exit 1 ;; \
    esac; \
    cargo build $FLAG; \
    cp "target/thumbv7m-none-eabi/$DIR/thetos_rc_demo" /firmware.elf

# --output writes this stage's files to the host
FROM scratch AS export
COPY --from=build /firmware.elf /thetos_rc_demo.elf
