name := 'cosmic-osd'
rootdir := ''
prefix := '/usr'
polkit-agent-helper-1 := '/usr/libexec/polkit-agent-helper-1'

debug := '0'
vendor := '0'

base-dir := absolute_path(clean(rootdir / prefix))
bindir := base-dir / 'bin'

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
target := if debug == '1' { 'debug' } else { 'release' }
release-arg := if debug == '1' { '' } else { '--release' }
vendor-arg := if vendor == '1' { '--frozen' } else { '' }

bin-src := cargo-target-dir / target / name
bin-dst := bindir / name

# Default recipe which runs `just build-release`
[private]
default: build-release

# Compiles with debug profile
build-debug *args:
    cargo build {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Builds the binary (respects debug and vendor variables)
build *args:
    env POLKIT_AGENT_HELPER_1={{polkit-agent-helper-1}} cargo build {{release-arg}} {{vendor-arg}} --bin {{name}} {{args}}

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
    install -Dm0755 {{bin-src}} {{bin-dst}}

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
