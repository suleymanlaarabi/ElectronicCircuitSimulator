use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PowerSupply {
    voltage: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Resistor {
    resistance: f64,
    tension_in_circuit: f64,
}

pub trait ElectronicComponentTrait {
    fn get_resistance(&self) -> f64;
    fn get_tension(&self) -> f64;
    fn set_tension(&mut self, tension: f64);
    fn to_resistor(&self) -> Resistor {
        Resistor {
            resistance: self.get_resistance(),
            tension_in_circuit: self.get_tension(),
        }
    }
}

impl Serialize for dyn ElectronicComponentTrait {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Resistor::serialize(&self.to_resistor(), serializer)
    }
}

impl<'de> Deserialize<'de> for Box<dyn ElectronicComponentTrait> {
    fn deserialize<D>(deserializer: D) -> Result<Box<dyn ElectronicComponentTrait>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let resistor = Resistor::deserialize(deserializer)?;
        Ok(Box::new(resistor))
    }
}

#[derive(Serialize, Deserialize)]
pub enum SeriesElement {
    Component(Box<dyn ElectronicComponentTrait>),
    Parallel(Vec<Series>),
}

impl Clone for Box<dyn ElectronicComponentTrait> {
    fn clone(&self) -> Self {
        Box::new(Resistor {
            resistance: self.get_resistance(),
            tension_in_circuit: self.get_tension(),
        })
    }
}

impl Clone for SeriesElement {
    fn clone(&self) -> Self {
        match self {
            SeriesElement::Component(component) => SeriesElement::Component(component.clone()),
            SeriesElement::Parallel(parallel) => {
                SeriesElement::Parallel(parallel.iter().map(|s| s.clone()).collect())
            }
        }
    }
}

impl SeriesElement {
    pub fn new<T: 'static + ElectronicComponentTrait>(component: T) -> Self {
        SeriesElement::Component(Box::new(component))
    }

    pub fn new_parallel(parallel: Vec<Series>) -> Self {
        SeriesElement::Parallel(parallel)
    }
}

pub type Series = Vec<SeriesElement>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Circuit {
    power_supply: PowerSupply,
    circuit: Vec<SeriesElement>,
}

impl PowerSupply {
    pub fn new(voltage: f64) -> Self {
        PowerSupply { voltage }
    }
}

impl Resistor {
    pub fn new(resistance: f64) -> Self {
        Resistor {
            resistance,
            tension_in_circuit: 0.0,
        }
    }
}

impl ElectronicComponentTrait for Resistor {
    fn get_resistance(&self) -> f64 {
        self.resistance
    }
    fn get_tension(&self) -> f64 {
        self.tension_in_circuit
    }
    fn set_tension(&mut self, tension: f64) {
        self.tension_in_circuit = tension;
    }
}

impl dyn ElectronicComponentTrait {
    pub fn new(resistance: f64) -> Box<dyn ElectronicComponentTrait> {
        Box::new(Resistor::new(resistance))
    }
}

fn calculate_total_resistance(elements: &[SeriesElement]) -> f64 {
    elements.iter().fold(0.0, |acc, element| match element {
        SeriesElement::Component(component) => acc + component.get_resistance(),
        SeriesElement::Parallel(parallel_series) => {
            acc + 1.0
                / parallel_series
                    .iter()
                    .map(|series| calculate_total_resistance(series))
                    .sum::<f64>()
                    .recip()
        }
    })
}

fn set_tensions_in_circuit(elements: &mut [SeriesElement], voltage: f64, total_resistance: f64) {
    for element in elements.iter_mut() {
        match element {
            SeriesElement::Component(component) => {
                let drop = (component.get_resistance() / total_resistance) * voltage;
                println!("Setting tension: {}", drop);
                component.set_tension(drop);
            }
            SeriesElement::Parallel(parallel_series) => {
                let parallel_resistance = calculate_total_resistance(
                    &parallel_series
                        .iter()
                        .flatten()
                        .cloned()
                        .collect::<Vec<_>>(),
                );
                let parallel_voltage = (parallel_resistance / total_resistance) * voltage;
                for series in parallel_series.iter_mut() {
                    set_tensions_in_circuit(series, parallel_voltage, parallel_resistance);
                }
            }
        }
    }
}

impl Circuit {
    pub fn new(power_supply: PowerSupply, circuit: Vec<SeriesElement>) -> Self {
        Circuit {
            power_supply,
            circuit,
        }
    }
    pub fn update_tensions(&mut self) {
        let total_resistance = calculate_total_resistance(&self.circuit);
        set_tensions_in_circuit(
            &mut self.circuit,
            self.power_supply.voltage,
            total_resistance,
        );
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Display for PowerSupply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Power Supply (Voltage: {}V)", self.voltage)
    }
}

impl Display for Resistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tension_rounded = (self.tension_in_circuit * 100.0).round() / 100.0;
        write!(
            f,
            "Resistor (Resistance: {}Ω, Tension in Circuit: {}V)",
            self.resistance, tension_rounded
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

impl Display for SeriesElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeriesElement::Component(component) => write!(f, "{}", component),
            SeriesElement::Parallel(parallel) => {
                write!(f, "Parallel Elements:\n")?;
                display_series(&parallel.iter().flatten().cloned().collect::<Vec<_>>(), f)
            }
        }
    }
}

fn display_series(series: &[SeriesElement], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (i, element) in series.iter().enumerate() {
        write!(f, "Element {}: {}\n", i + 1, element)?;
    }
    Ok(())
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circuit:\n")?;
        write!(f, "Power Supply: {}\n", self.power_supply)?;
        write!(f, "Series Elements:\n")?;
        for (i, element) in self.circuit.iter().enumerate() {
            write!(f, "Element {}: {}\n", i + 1, element)?;
        }
        Ok(())
    }
}
