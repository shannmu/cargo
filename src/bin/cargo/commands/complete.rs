use crate::cli;
use crate::command_prelude::*;
use clap::{Args, FromArgMatches};

pub fn cli() -> Command {
    let cmd = clap::Command::new("complete").about("Generate completion scripts for your shell");

    clap_complete::dynamic::CompleteArgs::augment_args(cmd)
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    if let Ok(arg) = clap_complete::dynamic::CompleteArgs::from_arg_matches(args) {
        arg.complete(&mut cli::cli(gctx));
        Ok(())
    } else {
        Err(CliError {
            error: Some(anyhow::anyhow!(
                "Failed to parse arguments for `cargo complete`"
            )),
            exit_code: -1,
        })
    }
}
