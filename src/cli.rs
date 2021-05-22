use crate::commands;
use crate::commands::command::Command;
use crate::config::QwerConfig;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// List all packages and their installed versions
    #[structopt(name = "list", visible_aliases = &["ls"])]
    ListPackages(commands::list_packages::ListPackages),
}

impl SubCommand {
    pub fn call(self, config: QwerConfig) {
        match self {
            Self::ListPackages(cmd) => cmd.call(config),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "qwer")]
pub struct Cli {
    #[structopt(flatten)]
    pub config: QwerConfig,

    #[structopt(subcommand)]
    pub subcmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::from_args()
}
