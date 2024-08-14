use cargo_test_support::cargo_test;
use snapbox::assert_data_eq;

#[cargo_test]
fn bash() {
    let input = "cargo -Z \t\t";
    let expected = snapbox::str!["% 
advanced-env              config-include            msrv-policy               rustdoc-map
allow-features            direct-minimal-versions   mtime-on-use              rustdoc-scrape-examples
asymmetric-token          doctest-xcompile          next-lockfile-bump        script
avoid-dev-deps            dual-proc-macros          no-index-update           separate-nightlies
binary-dep-depinfo        features                  package-workspace         skip-rustdoc-fingerprint
bindeps                   gc                        panic-abort-tests         target-applies-to-host
build-std                 git                       print-im-a-teapot         trim-paths
build-std-features        gitoxide                  profile-rustflags         unstable-options
cargo-lints               host-config               public-dependency         
codegen-backend           minimal-versions          publish-timeout           "];
    let actual = super::common::complete(input, "bash");

    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn zsh() {
    let input = "cargo -Z \t\t";
    let expected = snapbox::str!["% cargo -Z
advanced-env              config-include            msrv-policy               rustdoc-map
allow-features            direct-minimal-versions   mtime-on-use              rustdoc-scrape-examples
asymmetric-token          doctest-xcompile          next-lockfile-bump        script
avoid-dev-deps            dual-proc-macros          no-index-update           separate-nightlies
binary-dep-depinfo        features                  package-workspace         skip-rustdoc-fingerprint
bindeps                   gc                        panic-abort-tests         target-applies-to-host
build-std                 git                       print-im-a-teapot         trim-paths
build-std-features        gitoxide                  profile-rustflags         unstable-options
cargo-lints               host-config               public-dependency         
codegen-backend           minimal-versions          publish-timeout           "];
    let actual = super::common::complete(input, "zsh");

    assert_data_eq!(actual, expected);
}
