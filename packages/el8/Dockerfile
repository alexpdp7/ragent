FROM index.docker.io/rockylinux:8
RUN yum install -y openssl-devel
RUN yum groupinstall -y 'Development Tools'
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN cargo install cargo-rpm
WORKDIR /root/src/
CMD cargo rpm build
