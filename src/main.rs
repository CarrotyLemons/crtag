mod init;

/// specifies the semantic version of CarroTag being used
static VERSION: &str = "1.0.0";

/// is used to receive the next command line argument
fn _recieve_command_line(arguments: &mut std::env::Args) -> Option<String> {
    match arguments.next() {
        None => {
            println!("Command not recieved!");
            None
        }
        Some(command) => {
            if command.is_ascii() {
                Some(command)
            } else {
                println!("Input is not ASCII!");
                None
            }
        }
    }
}

/// loads the `CRTagDefinitions.toml`
fn _load_definitions() -> Result<toml::map::Map<String, toml::Value>, String> {
    // TODO: make the function search upwards for the definitions

    let carrotag_definitions;
    match std::fs::read_to_string(".carrotag/carrotag_definitions.toml") {
        Ok(string) => {
            carrotag_definitions = string;
        }
        Err(string) => return Err(format!("CRTagDefinitions.toml not found: {string}")),
    };

    Ok(carrotag_definitions.parse::<toml::Table>().unwrap())
}

fn main() {
    // Gather all command line arguments
    let mut arguments = std::env::args();

    // Skip binary location argument
    arguments.next();

    // Recieve the current executable command
    let command = match _recieve_command_line(&mut arguments) {
        None => return,
        Some(message) => message,
    };

    // Handle CLI argument processing and call functions
    let result = match command.as_str() {
        "init" => {
            let path;

            match arguments.next() {
                Some(supplied_path) => {
                    path = [
                        supplied_path,
                        ".crtag".to_string(),
                    ]
                    .join("/");
                },
                // By default use current directory
                None => path = ".crtag".to_string(),
            };

            init::run(path)
        }

        // Load the CRTagDefinitions
        // ```
        //let definitions = match _load_definitions() {
        //    Ok(map) => map,
        //    Err(message) => {
        //        println!("{message}");
        //        return;
        //    }
        //};
        // ```
        // "add" => add(arguments, definitions),
        // "find" => find(arguments, definitions),
        // "date" => date(arguments, definitions),
        // "subtag" => subtag(arguments, definitions),
        // "check" => check(arguments),
        // "new" => new(arguments, definitions),
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

// fn add(
//     mut arguments: std::env::Args,
//     definitions: toml::map::Map<String, toml::Value>,
// ) -> Result<(), String> {
// }

// fn find(
//     mut arguments: std::env::Args,
//     definitions: toml::map::Map<String, toml::Value>,
// ) -> Result<(), String> {
// }

// fn date(
//     mut arguments: std::env::Args,
//     definitions: toml::map::Map<String, toml::Value>,
// ) -> Result<(), String> {
// }

// fn subtag(
//     mut arguments: std::env::Args,
//     definitions: toml::map::Map<String, toml::Value>,
// ) -> Result<(), String> {
// }

// fn check(mut arguments: std::env::Args) -> Result<(), String> {}

// fn new(
//     mut arguments: std::env::Args,
//     definitions: toml::map::Map<String, toml::Value>,
// ) -> Result<(), String> {
// }
