## Pulumi Gestalt

![GitHub Release](https://img.shields.io/github/v/release/andrzejressel/pulumi-gestalt?include_prereleases&sort=date)
[![Build](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml)
[![Deploy](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml/badge.svg)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml)
[![Docs](https://readthedocs.org/projects/pulumi-gestalt/badge/?version=latest)](https://app.readthedocs.org/projects/pulumi-gestalt/builds/?version__slug=latest)
[![codecov](https://codecov.io/gh/andrzejressel/pulumi-gestalt/graph/badge.svg?token=J3IN76CSOP)](https://codecov.io/gh/andrzejressel/pulumi-gestalt)

**Pulumi Gestalt** is a framework designed to simplify the process of adding new language support to the Pulumi
ecosystem. It provides APIs for **WebAssembly (Wasm)**, **C FFI (Foreign Function Interface)**, and **Rust**, enabling
seamless integration of new languages with Pulumi.

## Motivation

Pulumi currently supports a limited number of programming languages. Adding support for a new language typically
requires significant effort to bridge the language with Pulumi's core infrastructure. Pulumi Gestalt aims to reduce this
effort by providing a common set of tools and APIs for language integration.

The framework is designed to work with both high-level and low-level languages, allowing developers to focus on
language-specific integration details without worrying about the underlying Pulumi infrastructure.

## User Guide

- [Rust](https://pulumi-gestalt.readthedocs.io/latest/languages/rust/)

## Integration Guide

- [C FFI](https://pulumi-gestalt.readthedocs.io/latest/integrations/c-ffi/)
- [Rust](https://pulumi-gestalt.readthedocs.io/latest/integrations/rust/)
- [Wasm](https://pulumi-gestalt.readthedocs.io/latest/integrations/wasm/)

### Quick start

https://github.com/andrzejressel/pulumi-gestalt-example

### Installation

#### Language plugin

```
pulumi plugin install language gestalt "VERSION" --server github://api.github.com/andrzejressel/pulumi-gestalt
```

#### Wasm Runner

```
cargo binstall -y pulumi_gestalt_wasm_runner@VERSION
```