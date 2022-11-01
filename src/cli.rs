use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};
use std::{
    fs,
    path::{PathBuf},
};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Whether the file structure should be created or destroyed
    #[command(subcommand)]
    pub action: Action,

    /// The path to the YAML file
    #[arg(short, long, default_value = "Structfile.yml")]
    pub file: PathBuf,

    /// Destination directory for the generated structure
    #[arg(short, long, default_value = ".")]
    pub destination: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Create,
    Destroy,
}

impl Cli {
    pub fn load_yaml(&self) -> Option<Yaml> {
        if !self.file.exists() {
            Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "File {} does not exist. \
                        Try using the `-f` flag to specify a different file.",
                        self.file.display()
                    ),
                )
                .exit();
            return None;
        }

        let mut docs = YamlLoader::load_from_str(&fs::read_to_string(&self.file).unwrap()).unwrap();

        if docs.get(0).is_none() {
            Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "File {} is empty. \
                        Try using the `-f` flag to specify a different file.",
                        self.file.display()
                    ),
                )
                .exit();
            None
        } else {
            Some(docs.swap_remove(0))
        }
    }

    pub fn assert_destination_exists(&self) {
        if !self.destination.exists() {
            println!(
                "Destination directory {} does not exist. \
                Creating it now...",
                self.destination.display()
            );
        }
    }
}
