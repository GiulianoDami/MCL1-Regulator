rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protein {
    pub id: String,
    pub name: String,
    pub sequence: String,
    pub molecular_weight: f64,
    pub isoelectric_point: f64,
    pub domains: Vec<Domain>,
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub partner_id: String,
    pub partner_name: String,
    pub binding_affinity: f64,
    pub interaction_type: InteractionType,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Binding,
    Inhibition,
    Activation,
    Modification,
    Unknown,
}

impl Protein {
    pub fn new(
        id: String,
        name: String,
        sequence: String,
        molecular_weight: f64,
        isoelectric_point: f64,
    ) -> Self {
        Protein {
            id,
            name,
            sequence,
            molecular_weight,
            isoelectric_point,
            domains: Vec::new(),
            interactions: Vec::new(),
        }
    }

    pub fn add_domain(&mut self, domain: Domain) {
        self.domains.push(domain);
    }

    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }

    pub fn get_domain_by_name(&self, name: &str) -> Option<&Domain> {
        self.domains.iter().find(|domain| domain.name == name)
    }

    pub fn get_interactions_by_type(&self, interaction_type: &InteractionType) -> Vec<&Interaction> {
        self.interactions
            .iter()
            .filter(|interaction| &interaction.interaction_type == interaction_type)
            .collect()
    }

    pub fn get_binding_partners(&self) -> Vec<&Interaction> {
        self.get_interactions_by_type(&InteractionType::Binding)
    }

    pub fn get_inhibitors(&self) -> Vec<&Interaction> {
        self.get_interactions_by_type(&InteractionType::Inhibition)
    }

    pub fn get_activators(&self) -> Vec<&Interaction> {
        self.get_interactions_by_type(&InteractionType::Activation)
    }
}