FROM rustlang/rust:nightly

RUN rustup component add rust-src

RUN cargo install cargo-xbuild
