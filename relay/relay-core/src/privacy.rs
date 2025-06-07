//! Privacy Guardian - Enforces privacy boundaries

use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub struct Guardian {
    private_zones: HashSet<PathBuf>,
}

impl Guardian {
    pub fn new() -> Self {
        Self {
            private_zones: HashSet::new(),
        }
    }
    
    pub fn add_private_zone(&mut self, path: PathBuf) {
        self.private_zones.insert(path);
    }
    
    pub fn is_private(&self, path: &Path) -> bool {
        self.private_zones.iter().any(|zone| path.starts_with(zone))
    }
    
    pub fn check_access(&self, agent_id: &str, path: &Path) -> crate::Result<()> {
        if self.is_private(path) {
            Err(crate::RelayError::PrivacyViolation(
                format!("Agent {} attempted to access private zone: {:?}", agent_id, path)
            ))
        } else {
            Ok(())
        }
    }
}
