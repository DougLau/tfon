#![forbid(unsafe_code)]

use anyhow::Result;
use argh::FromArgs;
use std::io::{stdin, stdout, Read};

/// Command-line arguments
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    #[argh(subcommand)]
    cmd: Command,
}

/// Sub-commands
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Tfon(TfonCommand),
}

/// convert font to tfon format
#[derive(Clone, Copy, FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tfon")]
struct TfonCommand {}

impl TfonCommand {
    fn convert(self) -> Result<()> {
        let mut buf = String::with_capacity(1024);
        stdin().read_to_string(&mut buf)?;
        let parser = tfon::ifntx::Parser::new(&buf);
        tfon::tfon::write(stdout(), parser)?;
        Ok(())
    }
}

impl Args {
    /// Run selected command
    fn run(self) -> Result<()> {
        match &self.cmd {
            Command::Tfon(tfon) => tfon.convert(),
        }
    }
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    args.run()?;
    Ok(())
}
