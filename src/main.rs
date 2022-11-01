mod cli;
mod errors;

use clap::Parser;
use crate::cli::{Cli, Action};
use crate::errors::{path_error, filter_not_found_error, Result};

use std::{path::{PathBuf}, fs};

use yaml_rust::{Yaml};



fn main() {
    let cli = Cli::parse();

    cli.assert_destination_exists();
    let struct_spec = cli.load_yaml().unwrap();

    let _res = match cli.action {
        Action::Create => create_entry(cli.destination, &struct_spec),
        Action::Destroy => destroy_entry(cli.destination, &struct_spec),
    };
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
