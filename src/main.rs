use circuit::{Circuit, PowerSupply, Series};

use dialoguer::{console::Term, theme::ColorfulTheme};

mod circuit;
mod views;

fn main() {
    let term = Term::stdout();

    let theme = &ColorfulTheme::default();

    let power_supply = PowerSupply::new(12.0);

    let series: Series = vec![];

    let mut circuit = Circuit::new(power_supply, series);

    loop {
        let exit = views::home(&mut circuit, &term, theme);

        if exit {
            break;
        }
    }
}
