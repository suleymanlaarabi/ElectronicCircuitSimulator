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

    let series: Series = vec![
        SeriesElement::new(Resistor::new(10.0)),
        SeriesElement::new(Resistor::new(20.0)),
        SeriesElement::new_parallel(vec![
            vec![
                SeriesElement::new(Resistor::new(30.0)),
                SeriesElement::new(Resistor::new(40.0)),
            ],
            vec![
                SeriesElement::new(Resistor::new(50.0)),
                SeriesElement::new_parallel(vec![
                    vec![SeriesElement::new(Resistor::new(60.0))],
                    vec![SeriesElement::new(Resistor::new(70.0))],
                ]),
            ],
        ]),
    ];

    let mut circuit = Circuit::new(power_supply, series);

    loop {
        let exit = views::home(&mut circuit, &term, theme);

        if exit {
            break;
        }
    }
}
