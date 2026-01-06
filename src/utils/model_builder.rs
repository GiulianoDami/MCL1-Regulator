rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModelBuilder {
    pub parameters: HashMap<String, f64>,
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Clone)]
pub struct Interaction {
    pub source: String,
    pub target: String,
    pub weight: f64,
    pub type_: String,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            interactions: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, name: &str, value: f64) {
        self.parameters.insert(name.to_string(), value);
    }

    pub fn add_interaction(&mut self, source: &str, target: &str, weight: f64, type_: &str) {
        self.interactions.push(Interaction {
            source: source.to_string(),
            target: target.to_string(),
            weight,
            type_: type_.to_string(),
        });
    }

    pub fn build_model(&self) -> Model {
        Model {
            parameters: self.parameters.clone(),
            interactions: self.interactions.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub parameters: HashMap<String, f64>,
    pub interactions: Vec<Interaction>,
}

impl Model {
    pub fn get_parameter(&self, name: &str) -> Option<f64> {
        self.parameters.get(name).copied()
    }

    pub fn get_interactions_by_type(&self, type_: &str) -> Vec<&Interaction> {
        self.interactions
            .iter()
            .filter(|interaction| interaction.type_ == *type_)
            .collect()
    }
}