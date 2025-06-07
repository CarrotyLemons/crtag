/// loads the `CRTagDefinitions.toml`
pub fn run() -> Result<toml::map::Map<String, toml::Value>, String> {
    let mut definitions = std::path::Path::new(".crtag/CRTagDefinitions.toml");

    // Search all parent directories for definitions
    while !definitions.is_file() {
        definitions = match definitions.parent() {
            Some(path) => {path},
            None => {return Err("Could not load definitions: Definitions not found!".to_string());}
        };
    }

    let file_contents = std::fs::read_to_string(definitions).unwrap();
    Ok(file_contents.parse::<toml::Table>().unwrap())
}