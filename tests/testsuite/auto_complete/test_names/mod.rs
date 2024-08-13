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

    let input = "cargo build --test \t\t";
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
--build-plan              --lib                     --tests                   test1
--color                   --locked                  --timings                 test2
--config                  --manifest-path           --unit-graph              
--example                 --message-format          --verbose                 
--examples                --no-default-features     --workspace               "
    ];
    let actual = super::common::complete(input, "bash");
    assert_data_eq!(actual, expected);

    let input = "cargo test \t\t";
    let expected = snapbox::str![
        "% 
--all                     --features                --offline                 -F
--all-features            --frozen                  --package                 -Z
--all-targets             --future-incompat-report  --profile                 -h
--bench                   --help                    --quiet                   -j
--benches                 --ignore-rust-version     --release                 -p
--bin                     --jobs                    --target                  -q
--bins                    --lib                     --target-dir              -r
--color                   --locked                  --test                    -v
--config                  --manifest-path           --tests                   test1
--doc                     --message-format          --timings                 test2
--example                 --no-default-features     --unit-graph              
--examples                --no-fail-fast            --verbose                 
--exclude                 --no-run                  --workspace               "
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

    let input = "cargo build --test \t\t";
    let expected = "% cargo build --test
--all                     --exclude                 --offline                 -F
--all-features            --features                --package                 -Z
--all-targets             --frozen                  --profile                 -h
--artifact-dir            --future-incompat-report  --quiet                   -j
--bench                   --help                    --release                 -p
--benches                 --ignore-rust-version     --target                  -q
--bin                     --jobs                    --target-dir              -r
--bins                    --keep-going              --test                    -v
--build-plan              --lib                     --tests                   test1
--color                   --locked                  --timings                 test2
--config                  --manifest-path           --unit-graph              
--example                 --message-format          --verbose                 
--examples                --no-default-features     --workspace               ";
    let actual = super::common::complete(input, "zsh");
    assert_data_eq!(actual, expected);

    let input = "cargo test \t\t";
    let expected = "% cargo test
--all                     --features                --offline                 -F
--all-features            --frozen                  --package                 -Z
--all-targets             --future-incompat-report  --profile                 -h
--bench                   --help                    --quiet                   -j
--benches                 --ignore-rust-version     --release                 -p
--bin                     --jobs                    --target                  -q
--bins                    --lib                     --target-dir              -r
--color                   --locked                  --test                    -v
--config                  --manifest-path           --tests                   test1
--doc                     --message-format          --timings                 test2
--example                 --no-default-features     --unit-graph              
--examples                --no-fail-fast            --verbose                 
--exclude                 --no-run                  --workspace               ";
    let actual = super::common::complete(input, "zsh");
    assert_data_eq!(actual, expected);
}
