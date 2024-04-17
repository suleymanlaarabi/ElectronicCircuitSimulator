use serde::{Deserialize, Serialize};

use super::{ElectronicComponentTrait, Resistor};

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
