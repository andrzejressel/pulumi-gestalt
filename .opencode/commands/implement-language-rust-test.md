---
description: Removes test from "expectedFailures" map in `pulumi-language-rust/language_test.go` and implements missing runtime support.
---

Use this playbook when you want to unskip a language conformance test in
`pulumi-language-rust/language_test.go` and implement missing runtime support.

## One possible solution

1. Find and remove the test from `expectedFailures` in
   `pulumi-language-rust/language_test.go`.
2. Check prior commits for a reference implementation pattern:
   - Base64 support: `5cb8944d8` (`Add to_base64/from_base64 (#1900)`) and
     follow-up `4f7dcce9b` (`Change from_base64 to return String with UTF-8 validation (#1932)`).
   - SHA1 support: `2bfead59c` (`l1-builtin-sha1 (#2012)`).
3. Add missing stdlib runtime support in `crates/rust/src/stdlib.rs`
   (new helper function + unit tests).
4. Map the builtin in code generation in
   `crates/rust_language_server/src/generator.rs` by extending
   `convert_stdlib_function_call` and validating arity
5. Regenerate language server Rust crate
   `just build-rust-bridge` to build the language server and regenerate static library.
6. Regenerate/accept conformance snapshots for the single test:
   `PULUMI_ACCEPT=1 go test ./... -run 'TestLanguage/$1' -count=1`
   from `pulumi-language-rust`.
7. Run formatting and checks:
   - `just fmt`
   - `just check`
   - Optional targeted re-run:
     `go test ./... -run 'TestLanguage/$1' -count=1`
8. Create branch called $1 and push changes to it.
9. Create commit with name $1 and push it

## Important locations

You can try replacing "-" with "_" in test names (and vice versa) if you have trouble finding the test in the codebase.

- File to convert location: @external/pulumi/cmd/pulumi-test-language/tests/testdata/$1
- Assertions: @external/pulumi/cmd/pulumi-test-language/tests/$1.go
- Python implementation (may not exist): @external/pulumi/sdk/python/cmd/pulumi-language-python/testdata/toml/projects/$1
- NodeJS implementation (may not exist): @external/pulumi/sdk/nodejs/cmd/pulumi-language-nodejs/testdata/tsnode/projects/$1
- Go implementation (may not exist): @external/pulumi/sdk/go/pulumi-language-go/testdata/projects/$1
- Java implementation (may not exist): @external/pulumi-java/pkg/cmd/pulumi-language-java/testdata/projects/$1

## Notes

- Keep error messages explicit in runtime helpers (prefer contextual errors).
- Follow the style used in previous builtin implementations for consistency.
