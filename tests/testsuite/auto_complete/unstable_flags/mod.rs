use cargo_test_support::cargo_test;
use snapbox::assert_data_eq;

#[cargo_test]
fn bash() {
    let input = "cargo -Z \t\t";
    let expected = snapbox::str!["% 
.cargo/               CODE_OF_CONDUCT.md    LICENSE-THIRD-PARTY   crates/               tests/
.git/                 CONTRIBUTING.md       README.md             credential/           triagebot.toml
.github/              Cargo.lock            benches/              deny.toml             windows.manifest.xml
.gitignore            Cargo.toml            build.rs              publish.py            
.ignore               LICENSE-APACHE        ci/                   src/                  
CHANGELOG.md          LICENSE-MIT           clippy.toml           target/               "];
    let actual = super::common::complete(input, "bash");

    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn zsh() {
    let input = "cargo -Z \t\t";
    let expected = snapbox::str!["% cargo -Z
.cargo/               CODE_OF_CONDUCT.md    LICENSE-THIRD-PARTY   crates/               tests/
.git/                 CONTRIBUTING.md       README.md             credential/           triagebot.toml
.github/              Cargo.lock            benches/              deny.toml             windows.manifest.xml
.gitignore            Cargo.toml            build.rs              publish.py            
.ignore               LICENSE-APACHE        ci/                   src/                  
CHANGELOG.md          LICENSE-MIT           clippy.toml           target/               "];
    let actual = super::common::complete(input, "zsh");

    assert_data_eq!(actual, expected);
}
