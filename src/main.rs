use std::{
    env::Args,
    path::{Path, PathBuf},
};

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
            // Ensure a `CRTagDefinitions.toml` does not already exist
            match load_definitions() {
                Ok(_) => {
                    println!("Cannot init: Definitions already exist!");
                    return;
                }
                _ => {}
            };

            let crtag_directory;

            // Convert path from supplied representation into its `.crtag` represetnation
            match arguments.next() {
                Some(supplied_directory) => {
                    crtag_directory = supplied_directory;
                }
                // By default use current directory
                None => {crtag_directory = ".".to_string()}
            };

            let crtag_directory = Path::new(crtag_directory.as_str()).to_path_buf();

            match ensure_crtag_file(&crtag_directory, "CRTagDefinitions.toml".to_string()) {
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
        "find" => find(arguments),
        "subtag" => {
            let tag = match arguments.next() {
                Some(contents) => contents,
                None => {
                    println!("Could not subtag: Insufficient arguments supplied!");
                    return;
                }
            };

            subtag(tag, arguments)
        }
        "new" => new(arguments),
        "version" => {
            println!("{VERSION}");
            Ok(())
        }
        "list" => {
            list()
        }
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

fn add(supplied_directory: PathBuf, tags: Args) -> Result<(), String> {
    let definitions = load_definitions()?;

    // Load in `crtag.toml` sidecar file
    let tag_file_path = ensure_crtag_file(&supplied_directory, "CRTag.toml".to_string())?;
    let tag_file_contents = std::fs::read_to_string(&tag_file_path).unwrap();
    let mut tag_file_contents = tag_file_contents.parse::<toml::Table>().unwrap();

    // Update version in sidecar file
    tag_file_contents.insert(
        "version".to_string(),
        toml::Value::String(VERSION.to_string()),
    );

    // Adds all tags to crtag sidecar file
    let mut invalid_tags: Result<(), Vec<String>> = Ok(());
    for tag in tags {
        if !definitions.contains_key(&tag) {
            // Add tag to error message
            match invalid_tags.clone() {
                Ok(_) => {
                    let mut invalid_message: Vec<String> = Vec::new();
                    invalid_message.push(format!("Could not add tag {tag}: Tag is not in the definitions!"));
                    invalid_tags = Err(invalid_message);
                },
                Err(message) => {
                    let mut message = message.clone();
                    message.push(format!("Could not add tag {tag}: Tag is not in the definitions!"));
                    invalid_tags = Err(message);
                }
            }
        }

        // Ensure tags exists
        let crtag_tags = match tag_file_contents.get("tags") {
            Some(contents) => contents.clone(),
            None => toml::value::Value::Array(Vec::new()),
        };

        // Ensure the tags are an array
        let mut crtag_tags = match crtag_tags {
            toml::Value::Array(contents) => contents,
            _ => return Err("Could not read tags: Tags were not of type array!".to_string()),
        };

        // Ensure the tag is in target tags
        if !crtag_tags.contains(&toml::Value::String(tag.clone())) {
            crtag_tags.push(toml::Value::String(tag));
        }

        // Edit the crtag map
        tag_file_contents.insert("tags".to_string(), toml::value::Value::Array(crtag_tags));
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
        Ok(_) => {},
        Err(error) => return Err(format!(
            "Could not write toml string: {}",
            error.to_string()
        )),
    };

    match invalid_tags.clone() {
        Ok(_) => Ok(()),
        Err(message) => {
            Err(message.join("\n"))
        }
    }
}

fn new(tags: Args) -> Result<(), String> {
    let mut definitions = load_definitions()?;

    // Add all tags to the `CRTagDefinitions.toml` file
    for tag in tags {
        if !definitions.contains_key(&tag) {
            let mut tag_values = toml::map::Map::new();

            tag_values.insert(
                "version".to_string(),
                toml::Value::String(VERSION.to_string()),
            );
            tag_values.insert("aliases".to_string(), toml::Value::Array(Vec::new()));
            tag_values.insert("subtags".to_string(), toml::Value::Array(Vec::new()));

            let tag_values = toml::value::Value::Table(tag_values);
            definitions.insert(tag, tag_values);
        };
    }

    // Store the updated `CRTagDefinitions.toml` file
    store_definitions(&definitions)
}

fn find(tags: Args) -> Result<(), String> {
    let definitions = load_definitions()?;

    let mut searchable_tags = Vec::new();

    // Create the list of all searchable tags
    for tag in tags {
        searchable_tags.push(tag.clone());

        let subtags;
        // Get tag attributes
        match definitions.get(tag.as_str()) {
            Some(contents) => {
                match contents.clone().get("subtags") {
                    Some(contents) => subtags = contents.clone(),
                    None => continue
                };
            },
            None => continue
        }

        // Ignore if subtags does not match needed format, should be permissive when searching.
        let subtags = match subtags {
            toml::value::Value::Array(contents) => contents,
            _ => continue,
        };

        for subtag in subtags {
            match subtag {
                toml::Value::String(contents) => searchable_tags.push(contents),
                _ => continue,
            }
        }
    }

    // Search for subtags
    let definitions_path = locate_definitions()?.parent().unwrap().parent().unwrap().to_path_buf();

    let found = search_dir(definitions_path, &searchable_tags, Vec::new())?;

    for item in found {
        println!("{item}");
    }

    Ok(())
}

fn list() -> Result<(), String> {
    let definitions = load_definitions()?;

    for tag in definitions {
        print!("{}", tag.0);
        match tag.1["subtags"].clone() {
            toml::Value::Array(contents) => {
                if !contents.is_empty() {
                    print!(":");
                }
                for subtag in contents {
                    match subtag {
                        toml::Value::String(message) => {print!("\n\t{message}");}
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        println!()
    }

    Ok(())
}

fn ensure_crtag_file(crtag_directory: &PathBuf, file: String) -> Result<PathBuf, String>{
    let mut crtag_directory = crtag_directory.clone();

    if !crtag_directory.is_dir() {
        return Err(format!("Could not create file: {} is not a valid directory!", crtag_directory.to_str().unwrap()))
    }

    crtag_directory.push(".crtag");

    if !crtag_directory.is_dir() {
        std::fs::create_dir(crtag_directory.clone()).unwrap();
    }

    crtag_directory.push(file);

    if !crtag_directory.exists() {
        std::fs::write(&crtag_directory, "").unwrap();
    }

    Ok(crtag_directory)
}

fn search_dir(
    dir: PathBuf,
    tags: &Vec<String>,
    mut found: Vec<String>,
) -> Result<Vec<String>, String> {

    // Add matches to output
    let mut potential_tag_dir = dir.clone();
    potential_tag_dir.push(".crtag/CRTag.toml");

    // Add data to crtag contents
    'get_tags: {
        // Load crtag contents of this directory if applicable
        let crtag_contents = match std::fs::read_to_string(&potential_tag_dir) {
            Ok(contents) => {
                contents.parse::<toml::Table>().unwrap()
            },
            _ => {break 'get_tags}
        };
    
        let crtag_tags = match crtag_contents.get("tags") {
            Some(contents) => {contents.clone()},
            _ => {break 'get_tags}
        };


        let crtag_tags = match crtag_tags {
            toml::value::Value::Array(contents) => {
                contents
            },
            _ => {break 'get_tags}
        };

        for search_tag in tags {
            if crtag_tags.contains(&toml::Value::String(search_tag.clone())) && !found.contains(&search_tag){
                found.push(dir.to_str().unwrap().to_string())
            }
        }
    }

    // Get all files in the dir
    let files = match dir.read_dir() {
        Ok(sub_dirs) => sub_dirs,
        Err(error) => return Err(format!("Could not search dir {}: {}", dir.to_str().unwrap(), error.to_string())),
    };

    // Recurse for each directory found
    for file in files {
        match file {
            Ok(contents) => {
                if contents.path().is_dir() {
                    found = search_dir(contents.path(), tags, found)?;
                }
            }
            Err(_) => continue,
        }
    }

    return Ok(found)
}

fn subtag(tag: String, subtags: Args) -> Result<(), String> {
    let mut definitions = load_definitions()?;

    // Check that tag exists
    if !definitions.contains_key(&tag) {
        return Err(format!("Could not find subtag: Tag {tag} does not exist"));
    }

    let tag_contents = definitions.get(&tag).unwrap();
    let mut tag_contents = match tag_contents {
        toml::Value::Table(contents) => contents.clone(),
        _ => {
            return Err(format!(
                "Could not load contents of {tag}: Contents is not of type table!"
            ))
        }
    };
    let tag_subtags = tag_contents.get("subtags").unwrap().clone();
    let mut tag_subtags = match tag_subtags {
        toml::Value::Array(contents) => contents,
        _ => return Err("Could not read subtags: Subtags were not of type array!".to_string()),
    };

    // Check that all subtags exist and add them
    for subtag in subtags {
        if !definitions.contains_key(&subtag) {
            return Err(format!(
                "Could not give {tag} subtag {subtag}: Subtag does not exist!"
            ));
        }

        // Ensure the subtag is in target tags
        if !tag_subtags.contains(&toml::Value::String(subtag.clone())) {
            tag_subtags.push(toml::Value::String(subtag));
        }
    }

    // Edit the tag information
    tag_contents.insert(
        "subtags".to_string(),
        toml::value::Value::Array(tag_subtags),
    );

    // Edit the definitions information
    definitions.insert(tag, toml::value::Value::Table(tag_contents));

    // Store the updated `CRTagDefinitions.toml` file
    store_definitions(&definitions)
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
    let definitions_path = Path::new(".crtag/CRTagDefinitions.toml").to_path_buf();
    let mut upper_directory = Path::new(".").canonicalize().unwrap();
    let mut search_path;

    // Search all parent directories for definitions
    loop {
        search_path = upper_directory.join(&definitions_path);

        if search_path.is_file() {
            return Ok(search_path.to_path_buf())
        }

        upper_directory = match upper_directory.parent() {
            Some(path) => {
                path.to_path_buf()
            },
            None => {
                return Err("Could not load definitions: Definitions not found!".to_string());
            }
        };
    }
}