use std::{env::Args, path::PathBuf, path::Path};

/// specifies the semantic version of CarroTag being used
static VERSION: &str = "1.0.0";

pub type TomlMap = toml::map::Map<String, toml::Value>;

fn main() {
    // Gather all command line arguments
    let mut arguments = std::env::args();

    // Ignore path of executable
    arguments.next();

    // Get the command
    let command = match arguments.next() {
        None => {
            println!("Command not recieved!");
            return;
        }
        Some(command_string) => command_string,
    };

    // Handle CLI argument processing and call functions
    let result: Result<(), String> = match command.as_str() {
        "init" => {
            let crtag_path;

            // Convert path from supplied representation into its `.crtag` represetnation
            match arguments.next() {
                Some(supplied_path) => {
                    crtag_path = [supplied_path, ".crtag".to_string()].join("/");
                }
                // By default use current directory
                None => {
                    crtag_path = ".crtag".to_string();
                }
            };

            let crtag_path = Path::new(crtag_path.as_str()).to_path_buf();

            match ensure_file_exists(&crtag_path, "CRTagDefinitions.toml".to_string()) {
                Ok(_) => Ok(()),
                Err(message) => Err(message),
            }
        }
        "add" => {
            let target_path = match arguments.next() {
                Some(path) => Path::new(path.as_str()).to_path_buf(),
                None => {
                    println!("Could not add: Insufficient arguments supplied!");
                    return;
                }
            };

            add(target_path, arguments)
        }
        // "find" => find(arguments, definitions),
        "subtag" => {
            let tag = match arguments.next() {
                Some(contents) => contents,
                None => {
                    println!("Could not subtag: Insufficient arguments supplied!");
                    return;
                }
            };

            subtag(tag, arguments)
        },
        "new" => new(arguments),
        "version" => {
            println!("{VERSION}");
            Ok(())
        },
        _ => {
            println!("Command was invalid!");
            return;
        }
    };

    match result {
        Ok(_) => (),
        Err(message) => println!("{message}"),
    };
}

fn add(mut target_path: PathBuf, tags: Args) -> Result<(), String> {
    // load definitions
    let definitions = load_definitions()?;

    target_path.push(".crtag");

    // Load in `crtag.toml` sidecar file
    let tag_file_path = ensure_file_exists(&target_path, "CRTag.toml".to_string())?;
    let tag_file_contents = std::fs::read_to_string(&tag_file_path).unwrap();
    let mut tag_file_contents = tag_file_contents.parse::<toml::Table>().unwrap();

    // Update version in sidecar file
    tag_file_contents.insert("version".to_string(), toml::Value::String(VERSION.to_string()));

    // Adds all tags to crtag sidecar file
    for tag in tags {
        if !definitions.contains_key(&tag) {
            return Err(format!(
                "Could not add tag {tag}: Tag is not in the definitions!"
            ));
        }

        tag_file_contents =
            ensure_crtag_has_tag_on_target(&definitions, tag_file_contents, tag)?;
    }

    // Stores the `CRTag.toml` at the specified path
    let toml_string = match toml::to_string(&tag_file_contents) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(format!(
                "Could not convert toml to string: {}",
                error.to_string()
            ))
        }
    };

    match std::fs::write(tag_file_path, toml_string) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "Could not write toml string: {}",
            error.to_string()
        )),
    }
}

fn new(tags: Args) -> Result<(), String> {
    let mut definitions = load_definitions()?;

    // Add all tags to the `CRTagDefinitions.toml` file
    for tag in tags {
        if definitions.contains_key(&tag) {
            match definitions[&tag] {
                toml::Value::Table(_) => {}
                _ => {
                    return Err(
                        "Could not process tag: tag contents is not of type table".to_string()
                    )
                }
            };
        } else {
            let mut tag_values = toml::map::Map::new();

            tag_values.insert(
                "version".to_string(),
                toml::Value::String(VERSION.to_string()),
            );
            tag_values.insert(
                "aliases".to_string(),
                toml::Value::Array(Vec::new()),
            );
            tag_values.insert(
                "subtags".to_string(),
                toml::Value::Array(Vec::new()),
            );

            let tag_values = toml::value::Value::Table(tag_values);
            definitions.insert(tag, tag_values);
        };
    }

    // Store the updated `CRTagDefinitions.toml` file
    store_definitions(&definitions)
}

fn subtag(tag: String, subtags: Args) -> Result<(), String>{
    let mut definitions = load_definitions()?;

    // Check that tag exists
    if !definitions.contains_key(&tag) {
        return Err(format!("Could not find subtag: Tag {tag} does not exist"))
    }

    let tag_contents = definitions.get(&tag).unwrap();
    let mut tag_contents = match tag_contents {
        toml::Value::Table(contents) => contents.clone(),
        _ => {return Err(format!("Could not load contents of {tag}: Contents is not of type table!"))}
    };
    let tag_subtags = tag_contents.get("subtags").unwrap().clone();
    let mut tag_subtags = match tag_subtags {
        toml::Value::Array(contents) => contents,
        _ => {return Err("Could not read subtags: Subtags were not of type array!".to_string())}
    };

    // Check that all subtags exist and add them
    for subtag in subtags {
        if !definitions.contains_key(&subtag) {
            return Err(format!("Could not give {tag} subtag {subtag}: Subtag does not exist!"))
        }

        // Ensure the subtag is in target tags
        if !tag_subtags.contains(&toml::Value::String(subtag.clone())) {
            tag_subtags.push(toml::Value::String(subtag));
        }
    }

    // Edit the tag information
    tag_contents.insert("subtags".to_string(), toml::value::Value::Array(tag_subtags));

    // Edit the definitions information
    definitions.insert(tag, toml::value::Value::Table(tag_contents));

    // Store the updated `CRTagDefinitions.toml` file
    store_definitions(&definitions)
}

/// creates file at path if it does not already exist
fn ensure_file_exists(crtag_path: &PathBuf, file_name: String) -> Result<PathBuf, String> {
    ensure_crtag_directory_exists(&crtag_path)?;

    let mut ensured_path = crtag_path.clone();

    // Check if tag file exists
    ensured_path.push(&file_name);
    if !ensured_path.is_file() {
        // Create file
        match std::fs::write(&ensured_path, "") {
            Ok(_) => Ok(ensured_path),
            Err(error) => Err(format!(
                "With path {}\nCould not make {} for init: {error}!",
                ensured_path.to_str().unwrap().to_string(),
                file_name
            )),
        }
    } else {
        Ok(ensured_path)
    }
}

/// tags target with a tag if it does not already have it
fn ensure_crtag_has_tag_on_target(
    definitions: &TomlMap,
    mut crtag: TomlMap,
    tag: String,
) -> Result<TomlMap, String> {
    // Check if key exists in definitions
    if !definitions.contains_key(&tag) {
        return Err("Could not ensure tag on target: Tag does not exist!".to_string());
    };

    // Ensure tags exists and is an array
    let crtag_tags = match crtag.get("tags") {
        Some(contents) => contents.clone(),
        None => {
            toml::value::Value::Array(Vec::new())
        }
    };

    let mut crtag_tags  = match crtag_tags {
        toml::Value::Array(contents) => contents,
        _ => {return Err("Could not read tags: Tags were not of type array!".to_string())}
    };

    // Ensure the tag is in target tags
    if !crtag_tags.contains(&toml::Value::String(tag.clone())) {
        crtag_tags.push(toml::Value::String(tag));
    }

    // Edit the crtag map
    crtag.insert("tags".to_string(), toml::value::Value::Array(crtag_tags));

    Ok(crtag)
}

/// loads the `CRTagDefinitions.toml`
fn load_definitions() -> Result<TomlMap, String> {
    let definitions_path = locate_definitions()?;

    let file_contents = std::fs::read_to_string(definitions_path).unwrap();
    Ok(file_contents.parse::<toml::Table>().unwrap())
}

/// stores the `CRTagDefinitions.toml`
fn store_definitions(definitions: &TomlMap) -> Result<(), String> {
    let definitions_path = locate_definitions()?;

    let toml_string = match toml::to_string(&definitions) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(format!(
                "Could not convert toml to string: {}",
                error.to_string()
            ))
        }
    };

    match std::fs::write(definitions_path, toml_string) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "Could not write toml string: {}",
            error.to_string()
        )),
    }
}

/// attempts to find the `CRTagDefinitions.toml` file
fn locate_definitions() -> Result<PathBuf, String> {
    let mut definitions_path = Path::new(".crtag/CRTagDefinitions.toml").to_path_buf();

    // Search all parent directories for definitions
    while !definitions_path.is_file() {
        let upper_directory = match Path::new("").parent() {
            Some(path) => path,
            None => {
                return Err("Could not load definitions: Definitions not found!".to_string());
            }
        };

        definitions_path = upper_directory.join(definitions_path);
    }

    Ok(definitions_path.to_path_buf())
}

fn ensure_crtag_directory_exists(path: &PathBuf) -> Result<(), String> {
    // Check if crtag directory exists
    if !path.is_dir() {
        // Create path
        match std::fs::create_dir(&path) {
            Ok(_) => Ok(()),
            Err(error) => {
                return Err(format!(
                    "With path {}\nCould not make dir: {error}!",
                    path.to_str().unwrap().to_string()
                ))
            }
        }
    } else {
        Ok(())
    }
}
