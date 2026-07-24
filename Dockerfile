# Rust version MUST match rust-toolchain.toml's channel pin. The build switches to
# the pinned toolchain inside the mounted repo, so `rustup target add` below has to
# target that same version or the aarch64 std ends up missing (E0463 core/std).
ARG version=1.93.0
FROM rust:${version}
ARG version

RUN dpkg --add-architecture arm64
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    git \
    ca-certificates \
    libclang-dev \
    libudev-dev \
    libudev-dev:arm64 \
    libinput-dev \
    libinput-dev:arm64 \
    libxkbcommon-dev \
    libxkbcommon-dev:arm64 \
    libssl-dev \
    libssl-dev:arm64

RUN apt-get install -y \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross

# Taskfile support
RUN curl -1sLf 'https://dl.cloudsmith.io/public/task/task/setup.deb.sh' | bash
RUN apt-get install -y task

# RPM support
RUN apt-get install -y rpm librpmbuild10 elfutils

# Install the aarch64 std for the PINNED toolchain ($version), not just the image
# default — mirrors cosmic-comp, which builds the same libudev/libinput deps.
RUN rustup target add --toolchain $version aarch64-unknown-linux-gnu
RUN rustup toolchain install --force-non-host $version-aarch64-unknown-linux-gnu
RUN rustup component add clippy
RUN chmod -R 777 /usr/local/rustup

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

# Retry transient network failures (crates.io / git deps) instead of failing the build
ENV CARGO_NET_RETRY=10
