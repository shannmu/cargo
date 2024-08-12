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

    let input = "cargo uninstall \t\t";
    let expected = snapbox::str![
        "% cargo uninstall
--bin                -p                   cargo-rudra          cargo-xr             rust-ld
--color              -q                   cargo-semver-checks  cargo-xrun           rust-lld
--config             -v                   cargo-size           cargo-xrustc         rust-nm
--frozen             bindgen              cargo-strip          cargo-xt             rust-objcopy
--help               bootimage            cargo-xb             cargo-xtest          rust-objdump
--locked             cargo-bootimage      cargo-xbuild         mdbook               rust-profdata
--offline            cargo-cov            cargo-xc             mdbook-toc           rust-readobj
--package            cargo-nextest        cargo-xcheck         mini-redis-cli       rust-size
--quiet              cargo-nm             cargo-xclippy        mini-redis-server    rust-strip
--root               cargo-objcopy        cargo-xdoc           nu                   rustfilt
--verbose            cargo-objdump        cargo-xfix           rudra                
-Z                   cargo-profdata       cargo-xinstall       rust-ar              
-h                   cargo-readobj        cargo-xpublish       rust-cov             "
    ];
    let actual = super::common::complete(input, "zsh");

    assert_data_eq!(actual, expected);
}

#[cargo_test]
fn zsh() {
    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    // Change current directory to the project root
    std::env::set_current_dir(cwd).unwrap();

    let input = "cargo uninstall \t\t";
    let expected = "% cargo uninstall
--bin                -p                   cargo-rudra          cargo-xr             rust-ld
--color              -q                   cargo-semver-checks  cargo-xrun           rust-lld
--config             -v                   cargo-size           cargo-xrustc         rust-nm
--frozen             bindgen              cargo-strip          cargo-xt             rust-objcopy
--help               bootimage            cargo-xb             cargo-xtest          rust-objdump
--locked             cargo-bootimage      cargo-xbuild         mdbook               rust-profdata
--offline            cargo-cov            cargo-xc             mdbook-toc           rust-readobj
--package            cargo-nextest        cargo-xcheck         mini-redis-cli       rust-size
--quiet              cargo-nm             cargo-xclippy        mini-redis-server    rust-strip
--root               cargo-objcopy        cargo-xdoc           nu                   rustfilt
--verbose            cargo-objdump        cargo-xfix           rudra                
-Z                   cargo-profdata       cargo-xinstall       rust-ar              
-h                   cargo-readobj        cargo-xpublish       rust-cov             ";
    let actual = super::common::complete(input, "zsh");
    assert_data_eq!(actual, expected);
}
