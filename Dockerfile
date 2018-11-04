FROM rustlang/rust:nightly AS buildenv

RUN rustup component add rust-src

RUN cargo install cargo-xbuild && \
    cargo install bootimage --version "^0.5.0"

FROM buildenv

ADD . /os
WORKDIR /os
RUN cargo xbuild --target x86_64-blog-os.json --release && bootimage build --release
