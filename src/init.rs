pub fn run(path: String) -> Result<(), String> {
    let mut path = std::path::Path::new(path.as_str()).to_path_buf();

    // Check if crtag directory exists
    if path.is_dir() {
        // Check if definition file exists
        path.push("CRTagDefinitions.toml");
        if path.is_file() {
            return Err(format!("With path {}\nCould not init: CRTagDefinitions.toml exists!", path.to_str().unwrap().to_string()))
        }
    } else {
        // Create path
        match std::fs::create_dir(&path) {
            Ok(_) => {},
            Err(error) => {return Err(format!("With path {}\nCould not make dir for init: {error}!", path.to_str().unwrap().to_string()))}
        }
        // Create definition file path
        path.push("CRTagDefinitions.toml");
    }

    // Create file
    match std::fs::write(&path, "") {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("With path {}\nCould not make definitions for init: {error}!", path.to_str().unwrap().to_string()))
    }
}
