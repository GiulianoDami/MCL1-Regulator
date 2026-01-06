rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicPathway {
    pub id: String,
    pub name: String,
    pub description: String,
    pub activation_state: PathwayActivation,
    pub associated_proteins: Vec<String>,
    pub regulatory_nodes: HashMap<String, RegulatoryNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub regulation_strength: f64,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Enzyme,
    TranscriptionFactor,
    Transporter,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Inactive,
    Modulated,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathwayActivation {
    Activated,
    Inhibited,
    Neutral,
    Variable,
}

impl MetabolicPathway {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            activation_state: PathwayActivation::Neutral,
            associated_proteins: Vec::new(),
            regulatory_nodes: HashMap::new(),
        }
    }

    pub fn set_activation_state(&mut self, state: PathwayActivation) {
        self.activation_state = state;
    }

    pub fn add_regulatory_node(&mut self, node: RegulatoryNode) {
        self.regulatory_nodes.insert(node.id.clone(), node);
    }

    pub fn add_associated_protein(&mut self, protein: String) {
        self.associated_proteins.push(protein);
    }

    pub fn get_node(&self, node_id: &str) -> Option<&RegulatoryNode> {
        self.regulatory_nodes.get(node_id)
    }

    pub fn update_node_status(&mut self, node_id: &str, status: NodeStatus) -> bool {
        if let Some(node) = self.regulatory_nodes.get_mut(node_id) {
            node.status = status;
            true
        } else {
            false
        }
    }
}