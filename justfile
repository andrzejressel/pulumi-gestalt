set windows-shell := ["pwsh.exe", "-c"]

@default: build test

build: build-language-plugin install-requirements build-wasm-components build-libraries

build-language-plugin:
    cd pulumi-language-wasm && just

install-requirements:
    rustup component add rustfmt
    cargo install cargo-component@0.10.1 --locked || cargo-component --version
    cargo install wasm-tools@1.201.0 --locked || wasm-tools --version

build-wasm-components:
    cargo component build -p pulumi_wasm -p pulumi_wasm_provider_random -p pulumi_wasm_example_simple

build-libraries:
    cargo build -p pulumi_wasm_runner \
                -p pulumi_wasm_rust \
                -p pulumi_wasm_rust_macro \
                -p pulumi_wasm_random

check:
    cargo fmt --all -- --check

format:
    cargo fmt --all
    cargo clippy --all --all-features --fix

format-clippy:
    cargo fmt --all
    cargo clippy --all --all-features --fix --allow-dirty --allow-staged

test:
    cargo test --all

[windows]
copy-provider:
    Remove-Item 'providers/pulumi_wasm_provider_random' -Recurse -Force
    Remove-Item 'providers/pulumi_wasm_provider_random_rust' -Recurse -Force
    mkdir 'providers/pulumi_wasm_provider_random'
    mkdir 'providers/pulumi_wasm_provider_random_rust'
    Copy-Item 'pulumi_wasm_generator/tests/output/random_provider/lib/*' 'providers/pulumi_wasm_provider_random_rust' -Recurse -Force
    Copy-Item 'pulumi_wasm_generator/tests/output/random_provider/provider/*' 'providers/pulumi_wasm_provider_random' -Recurse -Force