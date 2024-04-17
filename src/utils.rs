use console::style;

use crossterm::terminal::{Clear, ClearType};

pub fn print_header(title: &String, alert_message: &String) {
    let title_app = style("Electronic Circuit Simulator").underlined().cyan();
    print!("{}", Clear(ClearType::All));
    println!("\n{}", title_app);
    let alert_message = style(alert_message).bold().blue();
    let title = style(title).underlined().green();
    if !alert_message.to_string().is_empty() {
        println!("\n{}\n", alert_message);
    }
    println!("{}\n", title);
}
