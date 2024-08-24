use std::time::Duration;

pub(crate) fn complete(input: &str, shell: impl Into<Shell>) -> String {
    let shell = shell.into();

    // Load the runtime
    let (mut runtime, _scratch) = match shell {
        Shell::Bash => load_runtime::<completest_pty::BashRuntimeBuilder>(),
        Shell::Zsh => load_runtime::<completest_pty::ZshRuntimeBuilder>(),
    };

    // Exec the completion
    let term = completest::Term::new();
    let actual = runtime
        .complete(input, &term, Duration::from_millis(1000))
        .unwrap();

    actual
}

// Return the scratch directory to keep it not being dropped
pub(crate) fn load_runtime<R: completest::RuntimeBuilder>(
) -> (Box<dyn completest::Runtime>, snapbox::dir::DirRoot)
where
    <R as completest::RuntimeBuilder>::Runtime: 'static,
{
    let shell_name = R::name();

    let home = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/testsuite/auto_complete/snapshots")
        .join(shell_name);

    let scratch = snapbox::dir::DirRoot::mutable_temp()
        .unwrap()
        .with_template(&home)
        .unwrap();

    let home = scratch.path().unwrap().to_owned();

    let bin_path = cargo_test_support::cargo_exe();
    let bin_root = bin_path.parent().unwrap().to_owned();

    let runtime = Box::new(R::with_home(bin_root, home).unwrap());

    (runtime, scratch)
}

#[non_exhaustive]
pub(crate) enum Shell {
    Bash,
    Zsh,
}

impl From<String> for Shell {
    fn from(value: String) -> Self {
        match value.as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => panic!("Unsupported shell"),
        }
    }
}

impl From<&str> for Shell {
    fn from(value: &str) -> Self {
        match value {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => panic!("Unsupported shell"),
        }
    }
}
