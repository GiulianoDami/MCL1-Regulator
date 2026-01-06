rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protein {
    pub id: String,
    pub name: String,
    pub sequence: String,
    pub interactions: Vec<Interaction>,
    pub pathway_associations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub target_id: String,
    pub target_name: String,
    pub binding_affinity: f64,
    pub interaction_type: InteractionType,
    pub evidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Binding,
    Inhibition,
    Activation,
    Modification,
}

impl Protein {
    pub fn new(id: String, name: String, sequence: String) -> Self {
        Protein {
            id,
            name,
            sequence,
            interactions: Vec::new(),
            pathway_associations: Vec::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }

    pub fn add_pathway_association(&mut self, pathway: String) {
        self.pathway_associations.push(pathway);
    }

    pub fn get_binding_partners(&self) -> Vec<&str> {
        self.interactions
            .iter()
            .filter(|i| matches!(i.interaction_type, InteractionType::Binding))
            .map(|i| i.target_name.as_str())
            .collect()
    }

    pub fn get_high_affinity_interactions(&self, threshold: f64) -> Vec<&Interaction> {
        self.interactions
            .iter()
            .filter(|i| i.binding_affinity >= threshold)
            .collect()
    }

    pub fn get_pathway_count(&self) -> usize {
        self.pathway_associations.len()
    }
}