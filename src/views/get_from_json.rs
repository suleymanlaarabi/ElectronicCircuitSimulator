use console::{style, Term};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use dialoguer::{theme::ColorfulTheme, Input};
use std::io::stdout;

use crate::{circuit::Circuit, views::HomeReturn};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_from_json_view(circuit: &mut Circuit, term: &Term, theme: &ColorfulTheme) -> HomeReturn {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");

    let title = String::from("Get From JSON");
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let json_file_path = Input::<String>::with_theme(theme)
        .with_prompt("Enter the path to the JSON file")
        .interact_on(&term)
        .expect("Couldn't get the JSON file path");

    let path = Path::new(&json_file_path);

    let file = File::open(&path);

    let mut json_string = String::new();

    if file.is_err() {
        return HomeReturn::ContinueWithMessage(String::from("Couldn't open the file"));
    } else {
        let mut file = file.unwrap();

        match file.read_to_string(&mut json_string) {
            Err(_) => {
                return HomeReturn::ContinueWithMessage(String::from("Couldn't read the file"));
            }
            Ok(_) => {
                let circuit_from_json: Result<Circuit, _> = serde_json::from_str(&json_string);
                match circuit_from_json {
                    Err(_) => {
                        return HomeReturn::ContinueWithMessage(String::from(
                            "Couldn't parse the JSON",
                        ));
                    }
                    Ok(circuit_from_json) => {
                        *circuit = circuit_from_json;
                        circuit.update();
                        return HomeReturn::ContinueWithMessage(String::from(
                            "Circuit loaded from JSON",
                        ));
                    }
                }
            }
        }
    }
}
