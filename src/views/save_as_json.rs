use crate::circuit::Circuit;

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use console::{style, Term};

use dialoguer::{theme::ColorfulTheme, Input};

pub fn save_as_json_view(circuit: &Circuit, term: &Term, theme: &ColorfulTheme) {
    let mut stdout = std::io::stdout();

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

    let title = String::from("Save Circuit as JSON");
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let file_name = Input::<String>::with_theme(theme)
        .with_prompt("Enter the path to the JSON file")
        .interact_on(&term)
        .unwrap();

    let json = circuit.to_json();

    std::fs::write(file_name, json).expect("Unable to write file");
}
