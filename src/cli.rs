use crate::config;
use crate::db::{db::Database,mariadb};


use clap::{Args, Parser, Subcommand};
use colored::Colorize;

/// A simple tool to fill or generate quickly a database with fake data.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    GenerateConfig {
        /// Generate a template config file.
        #[arg(short,long, default_value_t = String::from("./config.yml"))]
        path: String,
    },
    Fill {
        #[clap(subcommand)]
        options: FillSubcommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum FillSubcommands {
    ByFile {
        #[arg()]
        path: String,
    },
    Inline {
        #[command(flatten)]
        fill_script_args: FillScriptArgs,

        /// Number of occurrence to create.
        #[arg(short, long)]
        occurrence: u32,
    },
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct FillScriptArgs {
    /// Generate a script. If not provided, it will fill directly the database.
    #[arg(short, long)]
    path_output_script: Option<String>,

    /// URL of the database to fill.
    #[arg(short = 'u', long)]
    db_url: Option<String>,
}

impl Cli {
    pub async fn handle(self) -> Self {
        match &self.command {
            Commands::GenerateConfig { path } => {
                match config::Configuration::generate(path) {
                    Ok(()) => {
                        println!("{}", "[SUCCESS]".green());
                    }
                    Err(e) => {
                        eprintln!("{} {}", "[ERROR]".red(), e);
                    }
                };
            }
            Commands::Fill { options } => {
                match options {
                    FillSubcommands::ByFile { path } => {
                        let config = config::Configuration::read_from_file(path).unwrap();
                        mariadb::MariaDB::fill(&config).await.unwrap();
                    },
                    FillSubcommands::Inline { fill_script_args, occurrence } => {

                    }
                }
            }
        };
        self
    }
}
