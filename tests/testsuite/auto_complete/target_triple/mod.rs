use cargo_test_support::prelude::*;
use snapbox::assert_data_eq;

#[cargo_test]
fn zsh() {
    let input = "cargo build --target \t\t";
    let actual = super::common::complete(input, "zsh");
    let expected = snapbox::str![
        "% cargo build --target
--all                     --examples                --message-format          --unit-graph
--all-features            --exclude                 --no-default-features     --verbose
--all-targets             --features                --offline                 --workspace
--artifact-dir            --frozen                  --package                 -F
--bench                   --future-incompat-report  --profile                 -Z
--benches                 --help                    --quiet                   -h
--bin                     --ignore-rust-version     --release                 -j
--bins                    --jobs                    --target                  -p
--build-plan              --keep-going              --target-dir              -q
--color                   --lib                     --test                    -r
--config                  --locked                  --tests                   -v
--example                 --manifest-path           --timings                 x86_64-unknown-linux-gnu"
    ];
    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn bash() {
    let input = "cargo build --target \t\t";
    let actual = super::common::complete(input, "bash");
    let expected = snapbox::str![
        "% 
--all                     --examples                --message-format          --unit-graph
--all-features            --exclude                 --no-default-features     --verbose
--all-targets             --features                --offline                 --workspace
--artifact-dir            --frozen                  --package                 -F
--bench                   --future-incompat-report  --profile                 -Z
--benches                 --help                    --quiet                   -h
--bin                     --ignore-rust-version     --release                 -j
--bins                    --jobs                    --target                  -p
--build-plan              --keep-going              --target-dir              -q
--color                   --lib                     --test                    -r
--config                  --locked                  --tests                   -v
--example                 --manifest-path           --timings                 x86_64-unknown-linux-gnu"
    ];
    assert_data_eq!(actual, expected);
}
