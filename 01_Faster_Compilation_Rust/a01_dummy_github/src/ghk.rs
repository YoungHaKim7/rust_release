use clap::Subcommand;

use crate::pr::Pr;

#[derive(Subcommand)]
pub enum Ghk {
    /// Manage pull requests
    Pr {
        #[command(subcommand)]
        command: Pr,
    },
    /// Login or Logout
    Auth,
}

impl Ghk {
    pub fn exec(&self) {
        match self {
            Ghk::Auth => {
                println!("trying to authenticate")
            }
            Ghk::Pr { command } => {
                command.exec();
            }
        }
    }
}
