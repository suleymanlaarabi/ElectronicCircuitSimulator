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

impl Display for SeriesElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeriesElement::Component(component) => write!(f, "{}", component),
            SeriesElement::Parallel(series) => {
                // print the first element of the parallel series next ....
                let first_element = series.first().unwrap().first().unwrap();
                write!(f, "Parallel : ({} ...)", first_element)?;
                Ok(())
            }
        }
    }
}

fn display_series(series: &Series, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for element in series {
        write!(f, "{}", element)?;
    }
    Ok(())
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print circuit voltage
        let voltage = self.get_power_supply().get_voltage();
        let voltage_rounded = (voltage * 100.0).round() / 100.0;
        writeln!(f, "Power Supply (Voltage: {}V)", voltage_rounded)?;

        // print circuit components
        for series in self.get_series() {
            match series {
                SeriesElement::Component(component) => writeln!(f, "{}", component)?,
                SeriesElement::Parallel(series) => {
                    display_series(series.first().unwrap(), f)?;
                    writeln!(f, "")?;
                }
            }
        }

        Ok(())
    }
}
