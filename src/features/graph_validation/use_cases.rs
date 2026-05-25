pub mod validate {
    use crate::features::graph_validation::adapters::bpmn::to_domain_process;
    use crate::features::graph_validation::domain::graph::Graph;

    use crate::transpiler::bpmn::{parse_bpmn, Definitions, Process};

    pub fn validate_bpmn_process(process: &Process) -> Result<(), Vec<String>> {
        let domain_process = to_domain_process(process);
        Graph::new(&domain_process).validate()
    }

    pub fn validate_bpmn_definitions(definitions: &Definitions) -> Result<(), Vec<String>> {
        match &definitions.process {
            Some(process) => validate_bpmn_process(process),
            None => Ok(()),
        }
    }

    pub fn validate_bpmn_xml(content: &str) -> Result<Definitions, String> {
        let defs = parse_bpmn(content).map_err(|e| format!("Invalid BPMN: {}", e))?;
        defs.validate_for_bpmn()?;
        validate_bpmn_definitions(&defs).map_err(|errors| errors.join("\n"))?;
        Ok(defs)
    }
}
