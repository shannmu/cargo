use cargo_test_support::cargo_test;
use snapbox::assert_data_eq;

use crate::shell_completions;

#[cargo_test]
fn test_register_native_completions() {
    let input = "cargo \t\t";
    let expected = snapbox::str![
        "% cargo
COMPLETING argument  
--color    --version  check              install         read-manifest  update        
--config   -C         clean              locate-project  remove         vendor        
--explain  -V         config             login           report         verify-project
--frozen   -Z         doc                logout          run            version       
--help     -h         fetch              metadata        rustc          yank          
--list     -q         fix                new             rustdoc      
--locked   -v         generate-lockfile  owner           search       
--offline  add        help               package         test         
--quiet    bench      info               pkgid           tree         
--verbose  build      init               publish         uninstall    "
    ];
    let actual = shell_completions::common::complete(input, "elvish");
    assert_data_eq!(actual, expected);
}
