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
        "% 
--bin       --config    --help      --offline   --quiet     --verbose   -h          -q          Cargo.toml
--color     --frozen    --locked    --package   --root      -Z          -p          -v          src/"
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

    let input = "cargo uninstall \t\t";
    let expected = "% cargo uninstall
--bin       --config    --help      --offline   --quiet     --verbose   -h          -q          Cargo.toml  
--color     --frozen    --locked    --package   --root      -Z          -p          -v          src/        ";
    let actual = super::common::complete(input, "zsh");
    assert_data_eq!(actual, expected);
}
