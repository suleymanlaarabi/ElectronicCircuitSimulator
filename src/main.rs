mod circuit;
use std::fs;

use circuit::{Circuit, PowerSupply, Resistor, Series, SeriesElement};

use dialoguer::{console::Term, theme::ColorfulTheme};

mod views;

fn render_circuit(circuit: &mut Circuit) {
    circuit.update_tensions();
    println!("{}", circuit);
}

fn main() {
    let term = Term::stdout();

    let theme = &ColorfulTheme::default();

    let power_supply = PowerSupply::new(220.0);

    let series: Series = vec![];

    let mut circuit = Circuit::new(power_supply, series);

    loop {
        let exit = views::home(&mut circuit, &term, theme);

        if exit {
            break;
        }
    }
}
