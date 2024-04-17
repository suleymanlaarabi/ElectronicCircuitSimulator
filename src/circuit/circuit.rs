use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PowerSupply {
    voltage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Resistor {
    resistance: f64,
    tension_in_circuit: f64,
}

#[derive(Serialize, Deserialize)]
pub enum SeriesElement {
    Component(ElectronicComponent),
    Parallel(Vec<Series>),
}

pub type Series = Vec<SeriesElement>;

#[derive(Serialize, Deserialize)]
pub struct Circuit {
    power_supply: PowerSupply,
    intensity: f64,
    circuit: Series,
}

#[derive(Serialize, Deserialize)]
pub enum ElectronicComponent {
    Resistor(Resistor),
}

impl ElectronicComponent {
    pub fn new_resistor(resistance: f64) -> Self {
        ElectronicComponent::Resistor(Resistor::new(resistance))
    }

    pub fn set_resistance(&mut self, resistance: f64) {
        match self {
            ElectronicComponent::Resistor(resistor) => resistor.resistance = resistance,
        }
    }
}

impl ElectronicComponentTrait for ElectronicComponent {
    fn get_resistance(&self) -> f64 {
        match self {
            ElectronicComponent::Resistor(resistor) => resistor.get_resistance(),
        }
    }

    fn get_tension(&self) -> f64 {
        match self {
            ElectronicComponent::Resistor(resistor) => resistor.get_tension(),
        }
    }

    fn set_tension(&mut self, tension: f64) {
        match self {
            ElectronicComponent::Resistor(resistor) => resistor.set_tension(tension),
        }
    }
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

impl SeriesElement {
    pub fn new_parallel(parallel: Vec<Series>) -> Self {
        SeriesElement::Parallel(parallel)
    }

    pub fn new(component: ElectronicComponent) -> Self {
        SeriesElement::Component(component)
    }
}

impl PowerSupply {
    pub fn new(voltage: f64) -> Self {
        PowerSupply { voltage }
    }

    pub fn get_voltage(&self) -> f64 {
        self.voltage
    }
}

impl Resistor {
    pub fn new(resistance: f64) -> Self {
        Resistor {
            resistance,
            tension_in_circuit: 0.0,
        }
    }

    pub fn get_resistance(&self) -> f64 {
        self.resistance
    }

    pub fn get_tension(&self) -> f64 {
        self.tension_in_circuit
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

fn calculate_parallel_resistance(series: &[Series]) -> f64 {
    series
        .iter()
        .map(|s| 1.0 / calculate_total_resistance(s))
        .sum::<f64>()
        .recip()
}

fn calculate_total_resistance(elements: &[SeriesElement]) -> f64 {
    elements.iter().fold(0.0, |acc, element| {
        acc + match element {
            SeriesElement::Component(component) => component.get_resistance(),
            SeriesElement::Parallel(parallel_series) => {
                calculate_parallel_resistance(parallel_series)
            }
        }
    })
}

fn set_tensions_in_circuit(elements: &mut [SeriesElement], voltage: f64) {
    let total_resistance = calculate_total_resistance(elements);

    elements.iter_mut().for_each(|element| match element {
        SeriesElement::Component(component) => {
            component.set_tension((voltage / total_resistance) * component.get_resistance());
        }
        SeriesElement::Parallel(parallel_series) => {
            let parallel_voltage =
                voltage * calculate_parallel_resistance(parallel_series) / total_resistance;
            parallel_series.iter_mut().for_each(|series| {
                set_tensions_in_circuit(series, parallel_voltage);
            });
        }
    });
}

pub fn calculate_current(circuit: &Circuit) -> f64 {
    let total_resistance = calculate_total_resistance(&circuit.circuit);
    circuit.power_supply.get_voltage() / total_resistance
}

impl Circuit {
    pub fn new(power_supply: PowerSupply, circuit: Series) -> Self {
        let mut new_circuit = Circuit {
            power_supply,
            circuit,
            intensity: 0.0,
        };

        new_circuit.update_tensions();
        new_circuit.update_intensity();

        new_circuit
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn update_intensity(&mut self) -> f64 {
        self.intensity = calculate_current(self);
        self.intensity
    }

    pub fn update_tensions(&mut self) {
        set_tensions_in_circuit(&mut self.circuit, self.power_supply.get_voltage());
    }

    pub fn update(&mut self) {
        self.update_tensions();
        self.update_intensity();
    }

    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }

    pub fn get_series(&self) -> &Series {
        &self.circuit
    }

    pub fn get_mut_series(&mut self) -> &mut Series {
        &mut self.circuit
    }

    pub fn get_power_supply(&self) -> &PowerSupply {
        &self.power_supply
    }
}
