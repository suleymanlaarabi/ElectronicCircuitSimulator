use crate::{
    circuit::Circuit,
    views::{
        circuit_view, edit_component::edit_component_view, get_from_json::get_from_json_view,
        pages_enum::Pages, save_as_json::save_as_json_view,
    },
};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

pub enum HomeReturn {
    Exit,
    Continue,
    ContinueWithMessage(String),
}

pub fn home(
    circuit: &mut Circuit,
    term: &Term,
    theme: &ColorfulTheme,
    message: Option<String>,
) -> HomeReturn {
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

    let mut title = String::from("Home");

    if message.is_some() {
        title.push_str(" - ");
        title.push_str(message.as_ref().unwrap().as_str());
    }

    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let menu: Vec<Pages> = vec![
        Pages::PrintCircuit,
        Pages::GetFromJson,
        Pages::EditComponent,
        Pages::SaveAsJson,
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

            return circuit_view::print_circuit_view(circuit, term);
        }
        Some(1) => {
            return get_from_json_view(circuit, term, theme);
        }
        Some(2) => {
            println!("Edit Component");

            return edit_component_view(circuit, theme);
        }
        Some(3) => {
            println!("Save As JSON");
            save_as_json_view(circuit, term, theme);
            return circuit_view::print_circuit_view(circuit, term);
        }
        Some(4) => {
            println!("Exit");
            return HomeReturn::Exit;
        }
        _ => {
            return HomeReturn::Exit;
        }
    }
}
