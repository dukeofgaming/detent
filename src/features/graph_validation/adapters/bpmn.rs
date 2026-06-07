use crate::features::graph_validation::domain::bpmn as domain_bpmn;
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Process;

pub fn to_domain_process(p: &Process) -> domain_bpmn::Process {
    domain_bpmn::Process {
        id: p.id.clone(),
        name: p.name.clone(),
        is_executable: p.is_executable,
        process_type: p.process_type.clone(),
        documentation: p.documentation.as_ref().map(|d| d.text.clone()),
        start_events: p
            .start_events
            .iter()
            .map(|e| domain_bpmn::StartEvent {
                id: e.id.clone(),
                name: e.name.clone(),
                outgoing: e.outgoing.clone(),
                documentation: e.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
        end_events: p
            .end_events
            .iter()
            .map(|e| domain_bpmn::EndEvent {
                id: e.id.clone(),
                name: e.name.clone(),
                incoming: e.incoming.clone(),
                documentation: e.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
        tasks: p
            .tasks
            .iter()
            .map(|t| domain_bpmn::Task {
                id: t.id.clone(),
                name: t.name.clone(),
                incoming: t.incoming.clone(),
                outgoing: t.outgoing.clone(),
                documentation: t.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
        service_tasks: p
            .service_tasks
            .iter()
            .map(|t| domain_bpmn::ServiceTask { id: t.id.clone() })
            .collect(),
        script_tasks: p
            .script_tasks
            .iter()
            .map(|t| domain_bpmn::ScriptTask { id: t.id.clone() })
            .collect(),
        exclusive_gateways: p
            .exclusive_gateways
            .iter()
            .map(|g| domain_bpmn::ExclusiveGateway { id: g.id.clone() })
            .collect(),
        parallel_gateways: p
            .parallel_gateways
            .iter()
            .map(|g| domain_bpmn::ParallelGateway { id: g.id.clone() })
            .collect(),
        sequence_flows: p
            .sequence_flows
            .iter()
            .map(|f| domain_bpmn::SequenceFlow {
                id: f.id.clone(),
                name: f.name.clone(),
                source_ref: f.source_ref.clone(),
                target_ref: f.target_ref.clone(),
                condition_expression: f.condition_expression.as_ref().map(|c| c.expression.clone()),
                documentation: f.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
    }
}
