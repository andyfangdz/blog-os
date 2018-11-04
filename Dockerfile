FROM rustlang/rust:nightly AS buildenv

RUN apt-get update && apt-get install -y \
    qemu \
 && rm -rf /var/lib/apt/lists/*

RUN rustup component add rust-src

RUN cargo install cargo-xbuild && \
    cargo install bootimage --version "^0.5.0"
