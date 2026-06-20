//! BPMN Definitions type (root element)

use serde::{Deserialize, Serialize};

use super::{BPMNDiagram, Process};

/// Root element of a BPMN document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "definitions")]
pub struct Definitions {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@targetNamespace", skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,

    #[serde(rename = "@exporter", skip_serializing_if = "Option::is_none")]
    pub exporter: Option<String>,

    #[serde(rename = "@exporterVersion", skip_serializing_if = "Option::is_none")]
    pub exporter_version: Option<String>,

    /// Single process (most common case)
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    /// Diagram info
    #[serde(rename = "BPMNDiagram", skip_serializing_if = "Option::is_none")]
    pub bpmn_diagram: Option<BPMNDiagram>,
}

impl Definitions {
    pub fn validate_for_bpmn(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("BPMN definitions must have an id".to_string());
        }

        if let Some(process) = &self.process {
            process.validate_for_bpmn()?;
        }

        Ok(())
    }
}
