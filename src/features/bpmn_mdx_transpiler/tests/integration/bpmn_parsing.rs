use cucumber::{given, then, when};
use detent::features::bpmn_mdx_transpiler::adapters::bpmn::parse_bpmn;

use super::super::ConvertWorld;

#[given("BPMN XML preserving user and manual tasks")]
fn given_task_xml(world: &mut ConvertWorld) {
    world.bpmn_xml = Some(
        r#"<bpmn:definitions id="d" xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
    <bpmn:process id="p">
        <bpmn:userTask id="ut1"><bpmn:incoming>f1</bpmn:incoming></bpmn:userTask>
        <bpmn:manualTask id="mt1"><bpmn:incoming>f2</bpmn:incoming></bpmn:manualTask>
        <bpmn:serviceTask id="st1"/>
    </bpmn:process>
</bpmn:definitions>"#
            .to_string(),
    );
}

#[when("I parse the BPMN XML")]
fn when_parse(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    world.parsed_defs = Some(parse_bpmn(xml).expect("parse should succeed"));
}

#[then("the process has one user task, one manual task, and one service task")]
fn then_task_counts(world: &mut ConvertWorld) {
    let proc = world
        .parsed_defs
        .as_ref()
        .and_then(|d| d.process.as_ref())
        .expect("should have process");
    assert_eq!(proc.user_tasks.len(), 1);
    assert_eq!(proc.user_tasks[0].id, "ut1");
    assert_eq!(proc.manual_tasks.len(), 1);
    assert_eq!(proc.manual_tasks[0].id, "mt1");
    assert_eq!(proc.service_tasks.len(), 1);
    assert_eq!(proc.service_tasks[0].id, "st1");
}

#[given("BPMN XML with an unsupported laneSet")]
fn given_lane_xml(world: &mut ConvertWorld) {
    world.bpmn_xml = Some(
        r#"<bpmn:definitions id="d" xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
    <bpmn:process id="p">
        <bpmn:laneSet id="lanes"><bpmn:lane id="lane1"/></bpmn:laneSet>
        <bpmn:serviceTask id="st1"/>
        <bpmn:serviceTask id="st2"/>
    </bpmn:process>
</bpmn:definitions>"#
            .to_string(),
    );
}

#[then("both service tasks remain after stripping laneSet")]
fn then_service_tasks_remain(world: &mut ConvertWorld) {
    let proc = world
        .parsed_defs
        .as_ref()
        .and_then(|d| d.process.as_ref())
        .expect("should have process");
    assert_eq!(proc.service_tasks.len(), 2);
    assert_eq!(proc.service_tasks[0].id, "st1");
    assert_eq!(proc.service_tasks[1].id, "st2");
}
