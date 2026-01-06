use std::collections::HashMap;

pub struct ModelBuilder {
    pub parameters: HashMap<String, f64>,
    pub interactions: Vec<(String, String, f64)>,
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

    pub fn add_interaction(&mut self, source: &str, target: &str, strength: f64) {
        self.interactions.push((source.to_string(), target.to_string(), strength));
    }

    pub fn build_model(&self) -> Model {
        Model {
            parameters: self.parameters.clone(),
            interactions: self.interactions.clone(),
        }
    }
}

pub struct Model {
    pub parameters: HashMap<String, f64>,
    pub interactions: Vec<(String, String, f64)>,
}

impl Model {
    pub fn get_parameter(&self, name: &str) -> Option<&f64> {
        self.parameters.get(name)
    }

    pub fn get_interactions(&self) -> &Vec<(String, String, f64)> {
        &self.interactions
    }
}