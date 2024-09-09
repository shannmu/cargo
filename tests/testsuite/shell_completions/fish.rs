use cargo_test_support::cargo_test;
use snapbox::assert_data_eq;

use crate::shell_completions;

#[cargo_test]
fn test_register_native_completions() {
    let input = "cargo \t\t";
    let expected = snapbox::str![
        "% cargo
--version                                                                                  (Print version info and exit)
--list                                                                                         (List installed commands)
--explain                                                      (Provide a detailed explanation of a rustc error message)
--verbose                                                        (Use verbose output (-vv very verbose/build.rs output))
--quiet                                                                                (Do not print cargo log messages)
--color                                                                                  (Coloring: auto, always, never)
--locked                                                                (Assert that `Cargo.lock` will remain unchanged)
--offline                                                                            (Run without accessing the network)
--frozen                                                          (Equivalent to specifying both --locked and --offline)
--config                                                                                (Override a configuration value)
--help                                                                                                      (Print help)
-V                                                                                         (Print version info and exit)
-v                                                               (Use verbose output (-vv very verbose/build.rs output))
-q                                                                                     (Do not print cargo log messages)
-C                                                            (Change to DIRECTORY before doing anything (nightly-only))
-Z                                             (Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details)
-h                                                                                                          (Print help)
add                                                                     (Add dependencies to a Cargo.toml manifest file)
bench                                                                        (Execute all benchmarks of a local package)
build                                                              (Compile a local package and all of its dependencies)
check                                                     (Check a local package and all of its dependencies for errors)
clean                                                            (Remove artifacts that cargo has generated in the past)
config                                                                                    (Inspect configuration values)
doc                                                                                    (Build a package's documentation)
fetch                                                                 (Fetch dependencies of a package from the network)
fix                                                                  (Automatically fix lint warnings reported by rustc)
generate-lockfile                                                                  (Generate the lockfile for a package)
help                                                                              (Displays help for a cargo subcommand)
info                                                               (Display information about a package in the registry)
init                                                               (Create a new cargo package in an existing directory)
install                                                                                          (Install a Rust binary)
locate-project                                             (Print a JSON representation of a Cargo.toml file's location)
login                                                                                            (Log in to a registry.)
logout                                                                   (Remove an API token from the registry locally)
metadata  (Output the resolved dependencies of a package, the concrete used versions including overrides, in machine-r…)
new                                                                               (Create a new cargo package at <path>)
owner                                                                     (Manage the owners of a crate on the registry)
package                                                        (Assemble the local package into a distributable tarball)
pkgid                                                                    (Print a fully qualified package specification)
publish                                                                               (Upload a package to the registry)
read-manifest                                                    (Print a JSON representation of a Cargo.toml manifest.)
remove                                                             (Remove dependencies from a Cargo.toml manifest file)
report                                                                   (Generate and display various kinds of reports)
run                                                                       (Run a binary or example of the local package)
rustc                                                        (Compile a package, and pass extra options to the compiler)
rustdoc                                                 (Build a package's documentation, using specified custom flags.)
search                                                  (Search packages in the registry. Default registry is crates.io)
test                                      (Execute all unit and integration tests and build examples of a local package)
tree                                                                (Display a tree visualization of a dependency graph)
uninstall                                                                                         (Remove a Rust binary)
update                                                          (Update dependencies as recorded in the local lock file)
vendor                                                                   (Vendor all dependencies for a project locally)
verify-project                                                                     (Check correctness of crate manifest)
version                                                                                       (Show version information)
yank                                                                              (Remove a pushed crate from the index)"];

    let actual = shell_completions::common::complete(input, "fish");
    assert_data_eq!(actual, expected);
}
