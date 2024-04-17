use crate::circuit::{Circuit, Series, SeriesElement};

use console::style;

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use dialoguer::theme::ColorfulTheme;

use std::{io::stdout, thread};

fn clear_terminal() {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");
}

fn render_series(series: &Series, theme: &ColorfulTheme) {
    clear_terminal();
    let select = dialoguer::Select::with_theme(theme)
        .with_prompt("Select a component to edit")
        .items(series)
        .default(0)
        .interact()
        .unwrap();

    let selected_series = &series[select];

    match selected_series {
        SeriesElement::Component(component) => {
            println!("Editing component: {}", component);
        }
        SeriesElement::Parallel(series) => {
            render_select_parallel(series, theme);
        }
    }
}

fn render_select_parallel(series: &Vec<Series>, theme: &ColorfulTheme) {
    clear_terminal();

    let select = dialoguer::Select::with_theme(theme)
        .with_prompt("Select a component to edit")
        .items(
            &series
                .iter()
                .map(|s| s.first().unwrap())
                .collect::<Vec<&SeriesElement>>(),
        )
        .default(0)
        .interact()
        .unwrap();
    let selected_series = &series[select];

    render_series(selected_series, theme);
}

pub fn edit_component_view(circuit: &mut Circuit, theme: &ColorfulTheme) -> bool {
    if circuit.get_series().is_empty() {
        println!("No components in the circuit");
        thread::sleep(std::time::Duration::from_secs(2));
        return false;
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

    let series: &Series = circuit.get_series();

    render_series(series, theme);

    false
}
