set windows-shell := ["pwsh.exe", "-c"]

@default: build-language-plugin-rust build-pulumi-test regenerator install-requirements test-all rust-docs fmt

# Regenerate "DO NOT EDIT" sections, recreate generator examples (but does not compile them), reformat whole project, check changelog
housekeeping-ci-flow: regenerator regenerate-pulumi-test-schema regenerate-language-rust regenerate-generator-tests changelog-dry-run fmt

# Runs all amd64 unit and doc tests tests
base-ci-flow: test

native-ci-flow: build-language-plugin-rust build-pulumi-test test-examples-native

# Regenerates provider from generator's integration test
generator-ci-flow COMPILATION_NAME:
    just test-provider-compilation {{COMPILATION_NAME}}

# Test docs examples and creates docs
test-docs-ci-flow: test-docs

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo check

build-language-plugin-rust:
    cd pulumi-language-rust && just build

[unix]
build-rust-bridge:
    cargo build -p pulumi_gestalt_rust_language_server --release

[windows]
build-rust-bridge:
    cargo build -p pulumi_gestalt_rust_language_server --release --target x86_64-pc-windows-gnu

build-pulumi-test:
    cd pulumi-test && just pulumi-test-install

package-language-plugin-rust VERSION:
    cd pulumi-language-rust && just package-language-plugin {{VERSION}}

test-language-plugin-rust:
    cd pulumi-language-rust && just test

test-language-plugin-rust-single TEST:
    cd pulumi-language-rust && just test-single "TestLanguage/{{TEST}}"

install-requirements:
    rustup component add rustfmt
    rustup component add llvm-tools-preview
    mise install

check:
    cargo fmt -- --check
    cargo clippy --tests --all-features

fmt:
    cd pulumi-language-rust && just fmt
    cd pulumi-test && just fmt
    cargo fmt
    cargo clippy --tests --all-features --fix --allow-dirty --allow-staged

clippy-to-file:
    cargo clippy --tests --all-features --message-format=json | clippy-sarif > rust-clippy-results.sarif
    python external/sarif-normalizer.py --in-place rust-clippy-results.sarif

regenerator:
    cd pulumi-language-rust && just regenerate-test-list
    cargo run -p regenerator

regenerate-generator-tests $DO_NOT_COMPILE="true":
    cargo nextest run -p pulumi_gestalt_generator --all-features --test '*' --profile all_cores

regenerate-pulumi-test-schema:
    cd pulumi-test && just pulumi-test-regenerate-schema-json

regenerate-language-rust:
    cd pulumi-language-rust && just regenerate

regenerate-and-format:
    just regenerator
    just regenerate-generator-tests
    just fmt

publish:
    cargo publish --workspace --all-features

test-provider-compilation COMPILATION_NAME:
    cargo llvm-cov nextest -p pulumi_gestalt_generator --cobertura --output-path covertura.xml --features generator_{{COMPILATION_NAME}} --test '*'

test-examples-native:
    cargo nextest run \
        -p pulumi_gestalt_example_dependencies \
        -p pulumi_gestalt_example_docker \
        -p pulumi_gestalt_example_multiple_providers \
        -p pulumi_gestalt_example_native \
        -p pulumi_gestalt_example_raw_rust \
        -p pulumi_gestalt_example_secret \
        -p pulumi_gestalt_example_stash \
        -p pulumi_gestalt_example_simple \
        -p pulumi_gestalt_example_test \
        -p pulumi_gestalt_example_typesystem \
        --features example_test

generator-tests:
    cargo nextest run --all-features -p pulumi_gestalt_generator

generator-tests-release:
    cargo nextest run --all-features -p pulumi_gestalt_generator --release

test-all:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml --all-features

test-all-release:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml --all-features --release

test:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml

docs:
    docker-compose -f docker-compose.docs.yml up

test-docs:
    cargo test --doc
    just rust-docs

rust-docs:
    cargo doc --no-deps \
        -p pulumi_gestalt_serde_constant_string \
        -p pulumi_gestalt_build \
        -p pulumi_gestalt_rust \
        -p pulumi_gestalt_core \
        -p pulumi_gestalt_providers_cloudflare \
        -p pulumi_gestalt_providers_docker \
        -p pulumi_gestalt_providers_random

rust-docs-release $RUSTDOCFLAGS="--html-in-header docs_additions/umami.html":
    just rust-docs

update-version NEW_VERSION:
    sd "0.0.0-DEV" "{{NEW_VERSION}}" "Cargo.toml" "pulumi-language-rust/main.go"

changelog-generate-for-repo NEW_VERSION:
    cargo run -p changelog -- generate-repo-changelog {{NEW_VERSION}}

changelog-generate-for-docs:
    cargo run -p changelog -- generate-for-docs

changelog-dry-run:
    cargo run -p changelog -- dry-run
