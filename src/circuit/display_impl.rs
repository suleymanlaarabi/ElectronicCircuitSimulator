use std::fmt::Display;

use crate::circuit::{Circuit, ElectronicComponentTrait, PowerSupply, Resistor, SeriesElement};

use super::{ElectronicComponent, Series};

impl Display for PowerSupply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Power Supply (Voltage: {}V)", self.get_voltage())
    }
}

impl Display for Resistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tension_rounded = (self.get_tension() * 100.0).round() / 100.0;
        write!(
            f,
            "Resistor (Resistance: {}Ω, Tension in Circuit: {}V)",
            self.get_resistance(),
            tension_rounded
        )
    }
}

impl Display for dyn ElectronicComponentTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tension_rounded = (self.get_tension() * 100.0).round() / 100.0;
        write!(
            f,
            "Resistor (Resistance: {}Ω, Tension in Circuit: {}V)",
            self.get_resistance(),
            tension_rounded
        )
    }
}

impl Display for ElectronicComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElectronicComponent::Resistor(resistor) => write!(f, "{}", resistor),
        }
    }
}

fn display_series_element(element: &SeriesElement) -> String {
    match element {
        SeriesElement::Component(component) => format!("{}", component),
        SeriesElement::Parallel(series) => {
            let mut series_str = String::new();
            for (i, element) in series.iter().enumerate() {
                series_str.push_str(&format!(
                    "Parallel {}: {}\n",
                    i + 1,
                    display_series(&element)
                ));
            }
            series_str
        }
    }
}

fn display_series(series: &Series) -> String {
    let mut series_str = String::new();
    for (i, element) in series.iter().enumerate() {
        series_str.push_str(&format!(
            "Element {}: {}\n",
            i + 1,
            display_series_element(element)
        ));
    }
    series_str
}

pub fn display_circuit(circuit: &Circuit) -> String {
    let mut circuit_str = String::new();
    circuit_str.push_str(&format!("Power Supply: {}\n", circuit.get_power_supply()));
    circuit_str.push_str(&format!(
        "Series:\n{}",
        display_series(&circuit.get_series())
    ));
    circuit_str
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_circuit(self))
    }
}
