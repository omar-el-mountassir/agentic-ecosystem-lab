//! Agent Management - Handle AI agent lifecycle and communication

use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

pub struct Pool {
    agents: HashMap<String, AgentInfo>,
    message_tx: mpsc::Sender<Message>,
}

#[derive(Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub content: serde_json::Value,
}

impl Pool {
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel(100);
        Self {
            agents: HashMap::new(),
            message_tx: tx,
        }
    }
    
    pub async fn register_agent(&mut self, agent: AgentInfo) {
        self.agents.insert(agent.id.clone(), agent);
    }
    
    pub async fn send_message(&self, message: Message) -> crate::Result<()> {
        self.message_tx.send(message).await
            .map_err(|_| crate::RelayError::AgentError("Failed to send message".into()))
    }
}
