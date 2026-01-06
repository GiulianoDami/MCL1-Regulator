rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicPathway {
    pub id: String,
    pub name: String,
    pub description: String,
    pub genes_involved: Vec<String>,
    pub activation_state: PathwayActivation,
    pub mcl1_interaction_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathwayActivation {
    Active,
    Inactive,
    Conditional,
}

impl MetabolicPathway {
    pub fn new(
        id: String,
        name: String,
        description: String,
        genes_involved: Vec<String>,
        mcl1_interaction_score: f64,
    ) -> Self {
        Self {
            id,
            name,
            description,
            genes_involved,
            activation_state: PathwayActivation::Conditional,
            mcl1_interaction_score,
        }
    }

    pub fn set_activation_state(&mut self, state: PathwayActivation) {
        self.activation_state = state;
    }

    pub fn is_active(&self) -> bool {
        matches!(self.activation_state, PathwayActivation::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self.activation_state, PathwayActivation::Inactive)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayCollection {
    pub pathways: HashMap<String, MetabolicPathway>,
}

impl PathwayCollection {
    pub fn new() -> Self {
        Self {
            pathways: HashMap::new(),
        }
    }

    pub fn add_pathway(&mut self, pathway: MetabolicPathway) {
        self.pathways.insert(pathway.id.clone(), pathway);
    }

    pub fn get_pathway(&self, id: &str) -> Option<&MetabolicPathway> {
        self.pathways.get(id)
    }

    pub fn get_mut_pathway(&mut self, id: &str) -> Option<&mut MetabolicPathway> {
        self.pathways.get_mut(id)
    }

    pub fn all_pathways(&self) -> impl Iterator<Item = &MetabolicPathway> {
        self.pathways.values()
    }

    pub fn active_pathways(&self) -> impl Iterator<Item = &MetabolicPathway> {
        self.pathways.values().filter(|p| p.is_active())
    }

    pub fn inactive_pathways(&self) -> impl Iterator<Item = &MetabolicPathway> {
        self.pathways.values().filter(|p| p.is_inactive())
    }
}