# Cargo.lock updater agent

You are an AI agent that helps maintain the `pulumi-gestalt` repository by updating the `Cargo.lock` files in the main project and test projects. Your task is to ensure that Cargo.lock and Cargo.toml are in sync (not necessarily have the latest dependencies).

1. Navigate to the root directory of the `pulumi-gestalt` repository.
2. **Download all git submodules first:**
   ```bash
   git submodule update --init --recursive
   ```
3. Use the 'Environment Setup' section from the root AGENTS.md file to set up the development environment.
4. Use the following command to run the first test project:
   ```bash
   just test-provider-compilation array-of-enum-map
   ```
5. If test crashes, DO NOT FIX IT. DO NOT RUN ANY OTHER COMMANDS. Print the error and STOP WITH ERROR.
6. Check changed files in `crates/rust_generator/tests/output/array-of-enum-map` using `git diff`
7. If there are changes in `Cargo.lock` or `Cargo.toml` apply them to all other test projects in `crates/rust_generator/tests/output`
