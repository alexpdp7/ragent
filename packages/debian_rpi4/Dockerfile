FROM docker.io/library/debian:bookworm
RUN dpkg --add-architecture arm64
RUN apt update
RUN apt install -y curl build-essential pkg-config libssl-dev:arm64 gcc-aarch64-linux-gnu
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo install cargo-deb
COPY cargo_config /root/.cargo/config
WORKDIR /root/src/
CMD AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR=/usr/include/openssl/ \
    AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu/ \
    cargo deb --target=aarch64-unknown-linux-gnu
