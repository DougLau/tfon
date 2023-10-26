// fonto: Program to convert between font formats
#![forbid(unsafe_code)]

use anyhow::Result;
use argh::FromArgs;
use std::io::{stdin, stdout, IsTerminal, Read};
use tfon::Prop;

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

/// Example font property iterator
#[derive(Clone, Debug)]
struct PropIter<'a> {
    pos: u16,
    #[allow(dead_code)]
    buf: &'a str,
}

impl<'a> Iterator for PropIter<'a> {
    type Item = Prop<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        self.pos += 1;
        match pos {
            0 => Some(Prop::FontName("Example Font")),
            1 => Some(Prop::FontNumber(1)),
            2 => Some(Prop::CharSpacing(1)),
            3 => Some(Prop::LineSpacing(1)),
            p if p < 256 + 3 => Some(Prop::CodePoint(p - 3)),
            _ => None,
        }
    }
}

impl<'a> PropIter<'a> {
    /// Create a new example font property iterator
    fn new(buf: &'a str) -> Self {
        PropIter { pos: 0, buf }
    }
}

impl TfonCommand {
    fn convert(self) -> Result<()> {
        let mut buf = String::with_capacity(1024);
        if stdin().is_terminal() {
            let props = PropIter::new(&buf);
            tfon::tfon::write(stdout(), props)?;
        } else {
            stdin().read_to_string(&mut buf)?;
            let props = tfon::ifntx::Parser::new(&buf);
            tfon::tfon::write(stdout(), props)?;
        };
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

/// Program entry point
fn main() -> Result<()> {
    let args: Args = argh::from_env();
    args.run()?;
    Ok(())
}
