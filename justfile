name := 'cosmic-osd'
rootdir := ''
prefix := '/usr'
polkit-agent-helper-1 := '/usr/libexec/polkit-agent-helper-1'
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

base-dir := absolute_path(clean(rootdir / prefix))
bin-dst := base-dir / 'bin' / name

# Default recipe which runs `just build-release`
[private]
default: build-release

# Compiles with debug profile
build-debug *args:
    env POLKIT_AGENT_HELPER_1={{polkit-agent-helper-1}} cargo build {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Build a debian package locally without a schroot or vendoring
build-deb:
    dpkg-buildpackage -d -nc

# Runs `cargo clean`
clean:
    cargo clean

# `cargo clean` and removes vendored dependencies
clean-dist: clean
    rm -rf .cargo vendor vendor.tar

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Installs files
install:
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}

# Vendor Cargo dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies
[private]
vendor-extract:
    #!/usr/bin/env sh
    rm -rf vendor
    tar pxf vendor.tar
