use console::{style, Term};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use dialoguer::{theme::ColorfulTheme, Input};
use std::io::stdout;

use crate::circuit::Circuit;
use crate::views::{circuit_view, pages_enum::Pages};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_from_json_view(circuit: &mut Circuit, term: &Term) {
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

    let json_file_path = Input::<String>::new()
        .with_prompt("Enter the path to the JSON file")
        .interact_on(&term)
        .unwrap();

    let path = Path::new(&json_file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut json_string = String::new();
    match file.read_to_string(&mut json_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let circuit_from_json: Circuit = serde_json::from_str(&json_string).unwrap();
            *circuit = circuit_from_json;
        }
    }

    circuit_view::print_circuit_view(circuit, term);
}
