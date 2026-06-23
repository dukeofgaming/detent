use cucumber::given;
use detent::features::graph_validation::domain::workflow::{Node, NodeType};

use super::super::GraphValidationWorld;

#[given(regex = r#"^a linear process with an orphan "([^"]+)"$"#)]
fn given_orphan(world: &mut GraphValidationWorld, orphan_id: String) {
    let mut w = super::super::linear_process();
    w.nodes.push(Node {
        id: orphan_id,
        node_type: NodeType::Action,
    });
    world.workflow = Some(w);
}
