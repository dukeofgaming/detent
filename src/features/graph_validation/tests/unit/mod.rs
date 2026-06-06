use detent::features::graph_validation::domain::bpmn::{Process, Task};

#[path = "../fixtures/linear_process.rs"]
mod fixtures;

use fixtures::linear_process;

mod graph_operations;
mod reachability;

fn process_with_orphan() -> Process {
    let mut p = linear_process();
    p.tasks.push(Task {
        id: "orphan_1".to_string(),
        name: Some("Orphan".to_string()),
        incoming: vec![],
        outgoing: vec![],
        documentation: None,
    });
    p
}
