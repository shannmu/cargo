use cargo_test_support::prelude::*;
use snapbox::assert_data_eq;

#[cargo_test]
fn zsh() {
    let input = "cargo build --target \t\t";
    let actual = super::common::complete(input, "zsh");
    let expected = snapbox::str![
        "% cargo build --target
--all                     --jobs                    --workspace               LICENSE-APACHE
--all-features            --keep-going              -F                        LICENSE-MIT
--all-targets             --lib                     -Z                        LICENSE-THIRD-PARTY
--artifact-dir            --locked                  -h                        README.md
--bench                   --manifest-path           -j                        benches/
--benches                 --message-format          -p                        build.rs
--bin                     --no-default-features     -q                        ci/
--bins                    --offline                 -r                        clippy.toml
--build-plan              --package                 -v                        crates/
--color                   --profile                 .cargo/                   credential/
--config                  --quiet                   .git/                     deny.toml
--example                 --release                 .github/                  publish.py
--examples                --target                  .gitignore                src/
--exclude                 --target-dir              .ignore                   target/
--features                --test                    CHANGELOG.md              tests/
--frozen                  --tests                   CODE_OF_CONDUCT.md        triagebot.toml
--future-incompat-report  --timings                 CONTRIBUTING.md           windows.manifest.xml
--help                    --unit-graph              Cargo.lock                
--ignore-rust-version     --verbose                 Cargo.toml                "
    ];
    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn bash() {
    let input = "cargo build --target \t\t";
    let actual = super::common::complete(input, "bash");
    let expected = snapbox::str![
        "% 
--all                     --jobs                    --workspace               LICENSE-APACHE
--all-features            --keep-going              -F                        LICENSE-MIT
--all-targets             --lib                     -Z                        LICENSE-THIRD-PARTY
--artifact-dir            --locked                  -h                        README.md
--bench                   --manifest-path           -j                        benches/
--benches                 --message-format          -p                        build.rs
--bin                     --no-default-features     -q                        ci/
--bins                    --offline                 -r                        clippy.toml
--build-plan              --package                 -v                        crates/
--color                   --profile                 .cargo/                   credential/
--config                  --quiet                   .git/                     deny.toml
--example                 --release                 .github/                  publish.py
--examples                --target                  .gitignore                src/
--exclude                 --target-dir              .ignore                   target/
--features                --test                    CHANGELOG.md              tests/
--frozen                  --tests                   CODE_OF_CONDUCT.md        triagebot.toml
--future-incompat-report  --timings                 CONTRIBUTING.md           windows.manifest.xml
--help                    --unit-graph              Cargo.lock                
--ignore-rust-version     --verbose                 Cargo.toml                "
    ];
    assert_data_eq!(actual, expected);
}
