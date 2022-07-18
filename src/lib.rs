use clap::{value_parser, App, Arg, ArgMatches};
use regex::Regex;
use std::error::Error;
use walkdir::DirEntry;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

pub fn run(config: &Config) -> MyResult<()> {
    let file_type_filter = |entry: &DirEntry| {
        config.types.is_empty()
            || config.types.iter().any(|ft| match ft {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };
    let file_name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(entry.file_name().to_str().unwrap()))
    };
    for path in &config.paths {
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            })
            .filter(file_type_filter)
            .filter(file_name_filter)
            .for_each(|e| println!("{}", e.path().display()));
    }
    Ok(())
}

// My initial stab at this; needs more structure and modularity -\**/-
pub fn old_run(config: &Config) -> MyResult<()> {
    for path in &config.paths {
        for entry in walkdir::WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if !config.types.is_empty() && !config.types.contains(&type_for(&entry)) {
                        continue;
                    }

                    let name = entry.file_name().to_str();
                    if name.is_none() {
                        continue;
                    }
                    let path = entry.path().to_str().unwrap();
                    if config.names.is_empty() {
                        println!("{}", path)
                    } else {
                        let name = name.unwrap();
                        for re in &config.names {
                            if re.is_match(name) {
                                println!("{}", path)
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Retrieve the EntryType for a given DirEntry
#[inline]
fn type_for(entry: &DirEntry) -> EntryType {
    let path = entry.path();
    if path.is_dir() {
        EntryType::Dir
    } else if path.is_file() {
        EntryType::File
    } else if path.is_symlink() {
        EntryType::Link
    } else {
        unimplemented!();
    }
}

impl clap::ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    #[inline]
    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        match self {
            Self::Dir => Some(clap::PossibleValue::new("d")),
            Self::File => Some(clap::PossibleValue::new("f")),
            Self::Link => Some(clap::PossibleValue::new("l")),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    types: Vec<EntryType>,
    names: Vec<Regex>,
}

pub fn parse_config(cmd_args: Vec<String>) -> MyResult<Config> {
    let matches = App::new("khoj")
        .version("0.1.0")
        .author("sanjayts")
        .about("find implemented in Rust")
        .arg(
            Arg::new("types")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .value_parser(value_parser!(EntryType))
                .multiple_values(true)
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("names")
                .short('n')
                .long("name")
                .value_name("NAME")
                .value_parser(value_parser!(Regex))
                .multiple_values(true)
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .multiple_values(true)
                .takes_value(true)
                .default_value("."),
        )
        .get_matches_from(cmd_args);

    let types = collect_many::<EntryType>(&matches, "types");
    let names = collect_many::<Regex>(&matches, "names");
    let paths = collect_many::<String>(&matches, "paths");
    Ok(Config {
        paths,
        types,
        names,
    })
}

#[inline]
// We need the below trait bounds since these are mandated by get_many
fn collect_many<T: Clone + Send + Sync + 'static>(matches: &ArgMatches, id: &str) -> Vec<T> {
    matches
        .get_many::<T>(id)
        .unwrap_or_default()
        .map(|s| s.to_owned())
        .collect()
}
