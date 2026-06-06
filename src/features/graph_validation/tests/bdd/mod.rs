use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::graph_validation::domain::bpmn::Process;

mod dangling_target_reported;
mod duplicate_id_rejected;
#[path = "../fixtures/linear_process.rs"]
mod fixtures;
mod no_end_event_rejected;
mod no_start_event_rejected;
mod orphan_node_detected;
mod steps;
mod well_formed_process_passes;

use fixtures::linear_process;

#[derive(Debug, Default, World)]
pub struct GraphValidationWorld {
    process: Option<Process>,
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
