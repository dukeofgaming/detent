//! Graph validation — Domain-layer semantic checks on Process graphs
//!
//! Provides graph-level validation that is standard-neutral (BPMN, SWS, etc.).
//! Graph invariants belong in the Domain layer per ADR-7.
//!
//! This module re-exports from the `detent-core` crate, which holds the
//! framework-agnostic domain types and graph operations. It also provides
//! conversion utilities to bridge adapter types to domain types.

pub use detent_core::bpmn;
pub use detent_core::process_graph::Graph;

use crate::compiler::bpmn as adapter_bpmn;

/// Convert a BPMN adapter [`Process`](adapter_bpmn::Process) to a domain
/// [`Process`](detent_core::bpmn::Process) for graph operations.
pub fn to_domain_process(p: &adapter_bpmn::Process) -> detent_core::bpmn::Process {
    detent_core::bpmn::Process {
        id: p.id.clone(),
        name: p.name.clone(),
        is_executable: p.is_executable,
        process_type: p.process_type.clone(),
        documentation: p.documentation.as_ref().map(|d| d.text.clone()),
        start_events: p
            .start_events
            .iter()
            .map(|e| detent_core::bpmn::StartEvent {
                id: e.id.clone(),
                name: e.name.clone(),
                outgoing: e.outgoing.clone(),
                documentation: e.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
        end_events: p
            .end_events
            .iter()
            .map(|e| detent_core::bpmn::EndEvent {
                id: e.id.clone(),
                name: e.name.clone(),
                incoming: e.incoming.clone(),
                documentation: e.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
        tasks: p
            .tasks
            .iter()
            .map(|t| detent_core::bpmn::Task {
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
            .map(|t| detent_core::bpmn::ServiceTask { id: t.id.clone() })
            .collect(),
        script_tasks: p
            .script_tasks
            .iter()
            .map(|t| detent_core::bpmn::ScriptTask { id: t.id.clone() })
            .collect(),
        exclusive_gateways: p
            .exclusive_gateways
            .iter()
            .map(|g| detent_core::bpmn::ExclusiveGateway { id: g.id.clone() })
            .collect(),
        parallel_gateways: p
            .parallel_gateways
            .iter()
            .map(|g| detent_core::bpmn::ParallelGateway { id: g.id.clone() })
            .collect(),
        sequence_flows: p
            .sequence_flows
            .iter()
            .map(|f| detent_core::bpmn::SequenceFlow {
                id: f.id.clone(),
                name: f.name.clone(),
                source_ref: f.source_ref.clone(),
                target_ref: f.target_ref.clone(),
                condition_expression: f
                    .condition_expression
                    .as_ref()
                    .map(|c| c.expression.clone()),
                documentation: f.documentation.as_ref().map(|d| d.text.clone()),
            })
            .collect(),
    }
}
