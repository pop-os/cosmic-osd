name := 'cosmic-osd'
rootdir := ''
prefix := '/usr'
polkit-agent-helper-1 := '/usr/libexec/polkit-agent-helper-1'
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

base-dir := absolute_path(clean(rootdir / prefix))
bin-dst := base-dir / 'bin' / name
helper-name := 'cosmic-polkit-helper'
helper-dst := base-dir / 'libexec' / helper-name
unit-dst := base-dir / 'lib' / 'systemd' / 'system'
pam-dst := absolute_path(clean(rootdir / 'etc' / 'pam.d'))

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
    install -Dm0755 {{ cargo-target-dir / 'release' / helper-name }} {{helper-dst}}
    install -Dm0644 data/pam.d/cosmic-osd-fingerprint {{ pam-dst / 'cosmic-osd-fingerprint' }}
    install -Dm0644 data/systemd/cosmic-polkit-helper.socket {{ unit-dst / 'cosmic-polkit-helper.socket' }}
    sed 's|@LIBEXECDIR@|{{ prefix / 'libexec' }}|' 'data/systemd/cosmic-polkit-helper@.service' \
        | install -Dm0644 /dev/stdin '{{ unit-dst / 'cosmic-polkit-helper@.service' }}'

# Vendor Cargo dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor --locked | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies
[private]
vendor-extract:
    #!/usr/bin/env sh
    rm -rf vendor
    tar pxf vendor.tar
