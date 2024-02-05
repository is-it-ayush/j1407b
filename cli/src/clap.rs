use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "j1047b",
    version,
    about = "A toy container runtime written in Rust",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
pub struct ClapCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Pull an image from the registry")]
    Pull {
        #[arg(index = 1, help = "The image to pull")]
        image: String,
    },
}
