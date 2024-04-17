use crate::{circuit::Circuit, views::home::HomeReturn};
use console::{style, Term};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use std::io::stdout;

pub fn print_circuit_view(circuit: &mut Circuit, term: &Term) -> HomeReturn {
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

    let title = String::from("Print Circuit");
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    circuit.update_tensions();
    println!("{}", circuit);

    let _ = term.read_key();

    HomeReturn::Continue
}
