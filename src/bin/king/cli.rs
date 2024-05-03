use crate::subcommands;
use std::{env, fs, path::{Path, PathBuf}};
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to config folder where render should be written
    /// typically ~/.config/king-loaf/
    #[arg(short, long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// binge shows in a given dir
    Binge {
        /// path to videos
        #[arg(short,long)]
        path: Option<String>
    },
    Host {
        // render template and host file
        path: Option<String>
    }
}

pub fn setup() {
    let cli = Cli::parse();

    /*
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
     * */

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Binge { path }) => {
            if let Some(s) = path.as_ref() {
                println!("Starting binge with a path :0 {s}");
                subcommands::play_list(Path::new(&s))
            } else { // use the CWD as the path
                let p = env::current_dir().unwrap();
                println!("Starting binge in cwd: {}",p.display());
                subcommands::play_list(p.as_path());
            }
        }
        Some(Commands::Host { path }) => {
            if let Some(s) = path.as_ref() {
                // set path as destination for render
                // call host command
                println!("Writing render to:{s}");
                subcommands::host(Path::new(&s))
            } else { // no pwd provided ( write render to ~/.config/kloaf/renders/index.html
                subcommands::host(
                    {
                        // get home directory
                        // join home directory with kloaf path
                        Path::new(env::var_os("HOME").expect("Couldn't get home directory").to_str().unwrap()).join(".config/kloaf").as_path()
                    }
                );
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
