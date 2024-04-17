use crate::circuit::{Circuit, ElectronicComponent, Series, SeriesElement};

use console::style;

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use dialoguer::theme::ColorfulTheme;

use std::io::stdout;

use super::HomeReturn;

fn clear_terminal() {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");
}

fn edit_component(component: &mut ElectronicComponent) -> HomeReturn {
    let resistance = dialoguer::Input::<f64>::new()
        .with_prompt("Enter the resistance Ω")
        .interact()
        .unwrap();

    component.set_resistance(resistance);

    HomeReturn::ContinueWithMessage(String::from("Component edited successfully"))
}

fn render_series(series: &mut Series, theme: &ColorfulTheme) -> HomeReturn {
    clear_terminal();
    let select = dialoguer::Select::with_theme(theme)
        .with_prompt("Select a component to edit")
        .items(series)
        .default(0)
        .interact()
        .unwrap();

    let selected_series = &mut series[select];

    match selected_series {
        SeriesElement::Component(component) => edit_component(component),
        SeriesElement::Parallel(series) => render_select_parallel(series.as_mut(), theme),
    }
}

fn render_select_parallel(series: &mut Vec<Series>, theme: &ColorfulTheme) -> HomeReturn {
    clear_terminal();

    let select = dialoguer::Select::with_theme(theme)
        .with_prompt("Select a component to edit")
        .items(
            &series
                .iter()
                .enumerate()
                .map(|(i, s)| format!("Branche {}: ({}...)", i, s.first().unwrap()))
                .collect::<Vec<String>>(),
        )
        .default(0)
        .interact()
        .unwrap();
    let selected_series = &mut series[select];

    render_series(selected_series, theme)
}

pub fn edit_component_view(circuit: &mut Circuit, theme: &ColorfulTheme) -> HomeReturn {
    if circuit.get_series().is_empty() {
        println!("No components in the circuit");
        return HomeReturn::ContinueWithMessage(String::from("No components in the circuit"));
    }

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

    let title = String::from("Edit Component");
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let series = circuit.get_mut_series();

    render_series(series, theme)
}
