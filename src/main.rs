use std::{fmt::Error, string};

use toml::map;

static VERSION: &str = "1.0.0";

fn recieve_command_line(arguments: &mut std::env::Args) -> Option<String> {
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

fn main() {
    let mut arguments = std::env::args();

    let current_directory = arguments.next().unwrap();

    let command = match recieve_command_line(&mut arguments) {
        None => return,
        Some(message) => message,
    };

    let result = match command.as_str() {
        "add" => add(arguments),
        // "find" => {

        // },
        // "date" => {

        // },
        // "subtag" => {

        // },
        // "hidden" => {

        // },
        // "check" => {

        // },
        // "new" => {

        //}
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

fn load_definitions() -> Result<toml::map::Map<String, toml::Value>, String> {
    // TODO: make the function search upwards for the definitions

    let carrotag_definitions;
    match std::fs::read_to_string(".carrotag/carrotag_definitions.toml") {
        Ok(string) => {
            carrotag_definitions = string;
        }
        Err(string) => return Err(format!("Definitions not found! {string}")),
    };

    Ok(carrotag_definitions.parse::<toml::Table>().unwrap())
}

// TODO: the command line called function, calls _new but supplies arguments
fn new() {

}

// TODO: the internally called function
fn _new(carrotag_definitions: &mut toml::map::Map<String, toml::value::Value>, tag: &String) {

}

fn add(mut arguments: std::env::Args) -> Result<(), String> {
    // Get the tags and related structure
    let mut carrotag_definitions = load_definitions()?;

    // Seperate the target file into its directory and the file
    let file_path: String = match arguments.next() {
        None => return Err("Target not recieved!".to_string()),
        Some(argument) => argument,
    };

    let mut file_path: Vec<&str> = file_path.split("/").collect();
    file_path.reverse();

    // Vector cannot be empty at this point
    let file = file_path.pop().unwrap();

    file_path.reverse();
    let file_path: String = file_path.join("/");

    let tags: Vec<String> = arguments.collect();

    // Find whether this tag exists and create it if not
    for tag in tags.iter() {
        'ensuring_tag: {
            for stored_tag in carrotag_definitions.keys() {
                match carrotag_definitions[stored_tag]["aliases"] {
                    toml::Value::Array(internals) => {
                        for value in internals.iter() {
                            match value {
                                toml::Value::String(alias) => {
                                    // No need to create tag if it already exists
                                    if alias == tag {
                                        break 'ensuring_tag;
                                    }
                                }
                                _ => {
                                    return Err("Tag aliases are not string!".to_string())
                                }
                            }
                        }
                    },
                    _ => return Err("Tag aliases are not an array!".to_string()),
                }
            }
            // Create the tag if it does not exist
            _new(&mut carrotag_definitions, tag);
        }
    }

    // TODO: Create/find the carrotag toml and update the values there

    Ok(())
}
