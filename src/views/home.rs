use crate::{
    circuit::Circuit,
    utils::print_header,
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

    let mut title = String::from("Home");

    if !circuit.get_series().is_empty() {
        title.push_str(format!(" - CircuitInfo (Intensity").as_str());
        title.push_str(
            format!(": {} A)", (circuit.get_intensity() / 100.0).round() * 100.0).as_str(),
        );
    }

    print_header(&title, &message.unwrap_or_else(|| String::from("")));

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
