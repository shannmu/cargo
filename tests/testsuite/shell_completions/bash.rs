use cargo_test_support::cargo_test;
use snapbox::assert_data_eq;

use crate::shell_completions;

#[cargo_test]
fn test_register_native_completions() {
    let input = "cargo \t\t";
    let expected = snapbox::str![
        "% 
--version          --help             check              install            read-manifest      update
--list             -V                 clean              locate-project     remove             vendor
--explain          -v                 config             login              report             verify-project
--verbose          -q                 doc                logout             run                version
--quiet            -C                 fetch              metadata           rustc              yank
--color            -Z                 fix                new                rustdoc            
--locked           -h                 generate-lockfile  owner              search             
--offline          add                help               package            test               
--frozen           bench              info               pkgid              tree               
--config           build              init               publish            uninstall          "
    ];
    let actual = shell_completions::common::complete(input, "bash");
    assert_data_eq!(actual, expected);
}
