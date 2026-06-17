use crate::features::graph_validation::adapters::bpmn::to_workflow;
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{Definitions, Process};
use crate::features::graph_validation::domain::graph::Graph;

pub fn validate_bpmn_process(process: &Process) -> Result<(), Vec<String>> {
    let workflow = to_workflow(process);
    Graph::new(&workflow).validate()
}

pub fn validate_bpmn_definitions(definitions: &Definitions) -> Result<(), Vec<String>> {
    match &definitions.process {
        Some(process) => validate_bpmn_process(process),
        None => Ok(()),
    }
}
