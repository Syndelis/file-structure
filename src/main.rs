use core::fmt;
use std::{path::{Path, PathBuf}, fs, error::{Error, self}, io};

use clap::{Parser, Subcommand, CommandFactory, error::ErrorKind};
use yaml_rust::{YamlLoader, Yaml};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Whether the file structure should be created or destroyed
    #[command(subcommand)]
    action: Action,

    /// The path to the YAML file
    #[arg(short, long, default_value = "Structfile.yml")]
    file: PathBuf,
    
    /// Destination directory for the generated structure
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Action {
    Create,
    Destroy,
}

fn main() {
    
    let cli = Cli::parse();

    cli.assert_destination_exists();
    let struct_spec = cli.load_yaml().unwrap();

    let res = match cli.action {
        Action::Create => create_entry(cli.destination, &struct_spec),
        Action::Destroy => destroy_entry(cli.destination, &struct_spec),
    };

}

impl Cli {
    fn load_yaml(&self) -> Option<Yaml> {
        if !self.file.exists() {
            Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "File {} does not exist. \
                        Try using the `-f` flag to specify a different file.",
                        self.file.display()
                    )
                )
                .exit();
            return None;
        }

        let mut docs = YamlLoader::load_from_str(
            &fs::read_to_string(&self.file).unwrap()
        ).unwrap();
        
        if docs.get(0).is_none() {
            Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "File {} is empty. \
                        Try using the `-f` flag to specify a different file.",
                        self.file.display()
                    )
                )
                .exit();
            None
        }
        else {
            Some(docs.swap_remove(0))
        }
    }

    fn assert_destination_exists(&self) {
        if !self.destination.exists() {
            println!(
                "Destination directory {} does not exist. \
                Creating it now...",
                self.destination.display()
            );
        }
    }

}

fn create_entry(path: PathBuf, value: &Yaml) -> Result<()> {
    match value {
        Yaml::String(content) => {
            println!("Creating file {} with contents {}", path.display(), content);
            fs::write(path, content)?;
        },
        Yaml::Hash(subhash) => {
            println!("Creating directory {}", path.display());
            fs::create_dir_all(&path)?;
            for (subpath, subvalue) in subhash {
                if let Yaml::String(subpath) = subpath {
                    create_entry(path.join(subpath), subvalue)?;
                }
                else {
                    panic!("Invalid path: {:?}", subpath);
                }
            }
        },
        _ => path_error(path)?,
    }

    Ok(())
}

fn destroy_entry(path: PathBuf, value: &Yaml) -> Result<()> {

    if let Yaml::Hash(hash) = value {
        for (subpath, subvalue) in hash {

            if let Yaml::String(subpath) = subpath {
                let path = &path;
                let subpath = path.join(subpath);
                
                match subvalue {
                    Yaml::String(_) => {
                        println!("Removing file {}", subpath.display());
                        filter_not_found_error(fs::remove_file(subpath))?;
                    },
                    Yaml::Hash(_) => {
                        println!("Removing directory {}", subpath.display());
                        filter_not_found_error(fs::remove_dir_all(subpath))?;
                    },
                    _ => path_error(subpath)?,
                }
            }
            else {
                panic!("Invalid path: {:?}", subpath);
            }
        }
    }

    Ok(())
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn path_error(path: PathBuf) -> Result<()> {
    Err(Box::new(WrongYamlSubtypeError { path }))
}

fn filter_not_found_error(res: io::Result<()>) -> io::Result<()> {
    if let Err(err) = &res {
        if err.kind() == io::ErrorKind::NotFound {
            return Ok(());
        }
    }

    res
}

#[derive(Debug)]
struct WrongYamlSubtypeError {
    path: PathBuf,
}

impl fmt::Display for WrongYamlSubtypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value for path {}", self.path.display())
    }
}

impl error::Error for WrongYamlSubtypeError {}
