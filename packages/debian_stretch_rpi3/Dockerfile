FROM debian:stretch
RUN dpkg --add-architecture armhf
RUN apt update
RUN apt install -y curl build-essential pkg-config libssl-dev:armhf gcc-arm-linux-gnueabihf
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN cargo install cargo-deb
COPY cargo_config /root/.cargo/config
RUN cp /usr/bin/arm-linux-gnueabihf-objcopy /usr/bin/objcopy
WORKDIR /root/src/
CMD ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR=/usr/include/openssl/ \
    ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf/ \
    cargo deb --target=armv7-unknown-linux-gnueabihf
