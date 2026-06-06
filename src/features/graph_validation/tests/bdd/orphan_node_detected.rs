use cucumber::given;
use detent::features::graph_validation::domain::bpmn::Task;

use super::GraphValidationWorld;

#[given(regex = r#"^a linear process with an orphan "([^"]+)"$"#)]
fn given_orphan(world: &mut GraphValidationWorld, orphan_id: String) {
    let mut p = super::linear_process();
    p.tasks.push(Task {
        id: orphan_id,
        name: Some("Orphan".to_string()),
        incoming: vec![],
        outgoing: vec![],
        documentation: None,
    });
    world.process = Some(p);
}
