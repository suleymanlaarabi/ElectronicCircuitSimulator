use crate::{
    circuit::Circuit,
    views::{circuit_view, get_from_json::get_from_json_view, pages_enum::Pages},
};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn home(circuit: &mut Circuit, term: &Term, theme: &ColorfulTheme) -> bool {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");

    let app_title: console::StyledObject<&str> = style("Electronic Circuit Simulator")
        .bold()
        .underlined()
        .green();
    println!("\n{}", app_title);

    let title = String::from("Home");
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let menu: Vec<Pages> = vec![
        Pages::PrintCircuit,
        Pages::GetFromJson,
        Pages::EditComponent,
        Pages::Exit,
    ];

    let selection = Select::with_theme(theme)
        .with_prompt("Select an option")
        .default(0)
        .items(&menu)
        .interact_on_opt(&term)
        .unwrap();

    match selection {
        Some(0) => {
            println!("Print Circuit");
            circuit_view::print_circuit_view(circuit, term);
            return false;
        }
        Some(1) => {
            println!("Get From JSON");
            get_from_json_view(circuit, term);
            return false;
        }
        Some(2) => {
            println!("Edit Component");
            return false;
        }
        Some(3) => {
            println!("Exit");
            return true;
        }
        _ => {
            return true;
        }
    }
}
