use crate::common::{Repository, compare_with_file};
use anyhow::Context;
use anyhow::Result;
use std::path::Path;

mod common;

const INITIAL_COMMIT_ID: &str = "377a47ceaf4e362dd808c80d3e44ab7d0fe146e4";

#[test]
fn generate_changelog_test() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example_released/.changelog",
    };

    let result = changelog_lib::generate_changelog(&options)?;

    compare_with_file(&result, Path::new("tests/example_released/expected.md"))?;

    Ok(())
}

#[test]
fn generate_mkdocs_changelog() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example_released/.changelog",
    };

    let result = changelog_lib::generate_mkdocs_changelog(&options)?;

    compare_with_file(
        &result,
        Path::new("tests/example_released/expected_mkdocs.md"),
    )?;

    Ok(())
}

fn create_repository() -> Result<Repository> {
    let repository = Repository::new()?;

    let repository = repository
        .copy_file("tests/example_released/.changelog/0.1.0/.gitkeep")?
        .add_and_commit("Initial commit (MUST NOT BE INCLUDED)")?
        .copy_file("tests/example_released/.changelog/0.1.0/1_added.yaml")?
        .add_and_commit("Add 1_added.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/2_changed.yaml")?
        .add_and_commit("Add 2_changed.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/3_deprecated.yaml")?
        .add_and_commit("Add 3_deprecated.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/4_removed.yaml")?
        .add_and_commit("Add 4_removed.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/5_fixed.yaml")?
        .add_and_commit("Add 5_fixed.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/6_security.yaml")?
        .add_and_commit("Add 6_security.yaml")?
        .copy_file("tests/example_released/.changelog/invalid_dir/7_file_move_test.yaml")?
        .add_and_commit("Add 7_pr.yaml")?
        .move_file(
            "tests/example_released/.changelog/invalid_dir/7_file_move_test.yaml",
            "tests/example_released/.changelog/0.1.0/7_file_move_test.yaml",
        )?
        .add_and_commit("Move 7_pr.yaml")?
        .copy_file("tests/example_released/.changelog/0.1.0/8_added_pr.yaml")?
        .add_and_commit("Add 8_pr.yaml (#1)")?
        .create_tag("v0.1.0")?
        .add_and_commit("[no-changelog] Do not include in changelog")?
        .add_and_commit("Some feature")?
        .add_and_commit_renovate("Some renovate bot commit (#3)")?
        .add_and_commit_dependabot("Some dependabot commit (#4)")?
        .add_and_commit("Some PR feature (#5)")?
        .create_tag("v0.2.0")?;

    Ok(repository)
}
