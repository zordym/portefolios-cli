use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Hexagonal,
    Onion,
    Layered,
    Pipeline,
    Microkernel,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Architecture::Hexagonal => write!(f, "Hexagonal"),
            Architecture::Onion => write!(f, "Onion"),
            Architecture::Layered => write!(f, "Layered"),
            Architecture::Pipeline => write!(f, "Pipeline"),
            Architecture::Microkernel => write!(f, "Microkernel"),
        }
    }
}

impl Architecture {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hexagonal" => Some(Architecture::Hexagonal),
            "onion" => Some(Architecture::Onion),
            "layered" => Some(Architecture::Layered),
            "pipeline" => Some(Architecture::Pipeline),
            "microkernel" => Some(Architecture::Microkernel),
            _ => None,
        }
    }
}
