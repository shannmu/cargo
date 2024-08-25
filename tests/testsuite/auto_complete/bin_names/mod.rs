use cargo_test_support::cargo_test;
use cargo_test_support::current_dir;
use cargo_test_support::Project;
use snapbox::assert_data_eq;

#[cargo_test]
fn bash() {
    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    // Change current directory to the project root
    std::env::set_current_dir(cwd).unwrap();

    let input = "cargo build --bin \t\t";
    let expected = snapbox::str![
        "% 
--all                     --exclude                 --offline                 -F
--all-features            --features                --package                 -Z
--all-targets             --frozen                  --profile                 -h
--artifact-dir            --future-incompat-report  --quiet                   -j
--bench                   --help                    --release                 -p
--benches                 --ignore-rust-version     --target                  -q
--bin                     --jobs                    --target-dir              -r
--bins                    --keep-going              --test                    -v
--build-plan              --lib                     --tests                   Cargo.toml
--color                   --locked                  --timings                 src/
--config                  --manifest-path           --unit-graph              
--example                 --message-format          --verbose                 
--examples                --no-default-features     --workspace               "
    ];
    let actual = super::common::complete(input, "bash");
    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn zsh() {
    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    // Change current directory to the project root
    std::env::set_current_dir(cwd).unwrap();

    let input = "cargo build --bin \t\t";
    let expected = "% cargo build --bin
--all                     --exclude                 --offline                 -F
--all-features            --features                --package                 -Z
--all-targets             --frozen                  --profile                 -h
--artifact-dir            --future-incompat-report  --quiet                   -j
--bench                   --help                    --release                 -p
--benches                 --ignore-rust-version     --target                  -q
--bin                     --jobs                    --target-dir              -r
--bins                    --keep-going              --test                    -v
--build-plan              --lib                     --tests                   Cargo.toml
--color                   --locked                  --timings                 src/
--config                  --manifest-path           --unit-graph              
--example                 --message-format          --verbose                 
--examples                --no-default-features     --workspace               ";
    let actual = super::common::complete(input, "zsh");
    assert_data_eq!(actual, expected);
}
