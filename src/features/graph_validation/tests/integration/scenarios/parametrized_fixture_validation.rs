use cucumber::given;
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
use detent::features::graph_validation::adapters::bpmn::to_workflow;
use detent::features::graph_validation::domain::workflow::Workflow;

use super::GraphValidationWorld;

fn parse_fixture_to_workflow(xml: &str) -> Workflow {
    let defs = parse_bpmn(xml).expect("fixture must parse");
    let process = defs.process.expect("fixture must have a process");
    to_workflow(&process)
}

#[given(regex = r"^the (linear|branching|blog-post|tdd) BPMN fixture$")]
fn given_fixture(world: &mut GraphValidationWorld, name: String) {
    let workflow = match name.as_str() {
        "linear" => super::linear_process(),
        "branching" => super::branching_process(),
        "blog-post" => {
            parse_fixture_to_workflow(include_str!("../../assets/blog_post/blog-post.bpmn2"))
        }
        "tdd" => parse_fixture_to_workflow(include_str!("../../assets/tdd/tdd.bpmn2")),
        _ => panic!("unknown fixture: {}", name),
    };
    world.workflow = Some(workflow);
}
