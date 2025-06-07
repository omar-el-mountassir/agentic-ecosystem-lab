//! Security - Capability-based permissions and audit logging

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub action: String,
    pub resource: String,
    pub constraints: HashMap<String, String>,
}

#[derive(Debug)]
pub struct CapabilityManager {
    agent_capabilities: HashMap<String, Vec<Capability>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            agent_capabilities: HashMap::new(),
        }
    }
    
    pub fn grant_capability(&mut self, agent_id: &str, capability: Capability) {
        self.agent_capabilities
            .entry(agent_id.to_string())
            .or_insert_with(Vec::new)
            .push(capability);
    }
    
    pub fn check_permission(&self, agent_id: &str, action: &str, resource: &str) -> bool {
        if let Some(capabilities) = self.agent_capabilities.get(agent_id) {
            capabilities.iter().any(|cap| {
                cap.action == action && resource.starts_with(&cap.resource)
            })
        } else {
            false
        }
    }
}
