FROM docker.io/library/debian:bullseye
RUN apt update
RUN apt install -y curl build-essential  libssl-dev pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN cargo install cargo-deb
WORKDIR /root/src/
CMD cargo deb
