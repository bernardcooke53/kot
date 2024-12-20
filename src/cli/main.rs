use crate::cli::repl;
use crate::magik;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::io::{self, IsTerminal, Read};

pub const LMGTFY: &'static str = "https://letmegooglethat.com";

#[derive(Debug, Parser)]
#[command(version, disable_help_subcommand = true, bin_name = "kot")]
pub(crate) struct TheBrainCell {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Foreign object to insert into the recipient text
    #[arg(short = 'o', long, default_value = "o")]
    pub(crate) foreign_object: String,

    /// Have a conversation (voids ur health insurance)
    #[arg(short, long)]
    pub(crate) interactive: bool,

    #[arg()]
    pub(crate) recipient: Option<String>,
}

#[derive(Debug, Args)]
pub(crate) struct PlsArgs {
    /// Fine, what the fuck is it?
    pub plea: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Ask nicely
    Help,

    /// THE BUTT THE BUTT THE BUTT
    Butt,

    /// Pls
    Pls(PlsArgs),
}

pub fn main() -> Result<()> {
    let the_braincell = TheBrainCell::parse();
    if let Some(command) = the_braincell.command {
        match command {
            Commands::Help => println!("oh poor baby, you are beyond help."),
            Commands::Butt => println!("THE BUTT THE BUTT THE BUTT"),
            Commands::Pls(args) => webbrowser::open(
                format!("{}/?q={}", LMGTFY, urlencoding::encode(args.plea.as_str())).as_str(),
            )?,
        }
        return Ok(());
    } else {
        if the_braincell.interactive
            || the_braincell.recipient.is_none()
            || io::stdin().is_terminal()
        {
            repl::repl(the_braincell.foreign_object.clone())?
        }
        if the_braincell.recipient.is_some() {
            println!(
                "{}",
                magik::with_consent(
                    the_braincell.recipient.unwrap(),
                    the_braincell.foreign_object
                )
            );
            return Ok(());
        }
        let mut recipient = String::new();
        io::stdin().read_to_string(&mut recipient).unwrap();
        println!(
            "{}",
            magik::with_consent(recipient, the_braincell.foreign_object.clone())
        );
        Ok(())
    }
}
