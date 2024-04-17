use circuit::{Circuit, PowerSupply, Series};

use dialoguer::{console::Term, theme::ColorfulTheme};

mod circuit;
mod utils;
mod views;

fn main() {
    let term = Term::stdout();

    let theme = &ColorfulTheme::default();

    let power_supply = PowerSupply::new(12.0);

    let series: Series = vec![];

    let mut circuit = Circuit::new(power_supply, series);

    let mut current_message: Option<String> = Some(String::from(""));

    loop {
        match current_message.to_owned() {
            Some(data) => {
                if data.is_empty() {
                    current_message = None;
                }
            }
            None => {}
        }

        let response = views::home(&mut circuit, &term, theme, current_message.clone());

        match response {
            views::HomeReturn::Exit => break,
            views::HomeReturn::Continue => continue,
            views::HomeReturn::ContinueWithMessage(message) => {
                current_message = Some(message);
                continue;
            }
        }
    }
}
