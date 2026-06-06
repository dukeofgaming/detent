use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::graph_validation::domain::bpmn::Process;
use std::collections::HashMap;

#[path = "../fixtures/linear_process.rs"]
mod fixtures;
mod steps;

#[path = "scenarios/well_formed_process_passes.rs"]
mod well_formed_process_passes;
#[path = "scenarios/no_start_event_rejected.rs"]
mod no_start_event_rejected;
#[path = "scenarios/no_end_event_rejected.rs"]
mod no_end_event_rejected;
#[path = "scenarios/dangling_target_reported.rs"]
mod dangling_target_reported;
#[path = "scenarios/duplicate_id_rejected.rs"]
mod duplicate_id_rejected;
#[path = "scenarios/orphan_node_detected.rs"]
mod orphan_node_detected;
#[path = "scenarios/dangling_source_reported.rs"]
mod dangling_source_reported;
#[path = "scenarios/duplicate_flow_id_rejected.rs"]
mod duplicate_flow_id_rejected;
#[path = "scenarios/dead_end_detected.rs"]
mod dead_end_detected;
#[path = "scenarios/process_flow_analysis.rs"]
mod process_flow_analysis;

use fixtures::{linear_process, linear_process_with_retargeted_exit};

/// Owned snapshot of the flow analysis computed by the
/// "When I analyze the process flow" step, so that `Then` steps read cached
/// results instead of rebuilding a borrowed `Graph` on each assertion.
#[derive(Debug, Default)]
pub struct FlowAnalysis {
    pub successors: HashMap<String, Vec<String>>,
    pub predecessors: HashMap<String, Vec<String>>,
    pub entry_nodes: Vec<String>,
    pub exit_nodes: Vec<String>,
    pub reachable: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default, World)]
pub struct GraphValidationWorld {
    process: Option<Process>,
    analysis: Option<FlowAnalysis>,
    errors: Option<Vec<String>>,
    succeeded: bool,
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/graph_validation/tests/bdd/slice.feature"
    );
    let summarized = GraphValidationWorld::cucumber()
        .with_writer(writer::Basic::stdout().summarized())
        .run(features_path)
        .await;
    summarized.execution_has_failed()
}
